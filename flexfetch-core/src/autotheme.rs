//! Phase 5.4 — wallpaper auto-theming (`--auto-theme`, feature `auto-theme`).
//!
//! Extracts the wallpaper's dominant colors (a color-thief style bucket+score
//! quantizer) and builds a `ThemeStrings` on the fly. The extracted palette is
//! cached to `/tmp` keyed by wallpaper path + mtime so repeated runs are free;
//! the cache is invalidated whenever the wallpaper changes.

use crate::theme::ThemeStrings;

/// Cache file name: `/tmp/flexfetch-autotheme-<hash>` where the hash covers the
/// wallpaper path + mtime so a wallpaper change picks a fresh key.
fn cache_path(wallpaper: &str, mtime: std::time::SystemTime) -> std::path::PathBuf {
    use std::hash::{Hash, Hasher};
    let m = mtime
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    wallpaper.hash(&mut hasher);
    m.hash(&mut hasher);
    std::path::PathBuf::from("/tmp").join(format!("flexfetch-autotheme-{:016x}", hasher.finish()))
}

/// Read a cached palette (`r,g,b` per line). Returns None on any parse issue.
fn read_cache(path: &std::path::Path) -> Option<Vec<[u8; 3]>> {
    let content = std::fs::read_to_string(path).ok()?;
    let colors: Vec<[u8; 3]> = content
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(',');
            let r: u8 = parts.next()?.trim().parse().ok()?;
            let g: u8 = parts.next()?.trim().parse().ok()?;
            let b: u8 = parts.next()?.trim().parse().ok()?;
            Some([r, g, b])
        })
        .collect();
    if colors.len() < 3 {
        return None;
    }
    Some(colors)
}

fn write_cache(path: &std::path::Path, colors: &[[u8; 3]]) {
    let body = colors
        .iter()
        .map(|c| format!("{},{},{}", c[0], c[1], c[2]))
        .collect::<Vec<_>>()
        .join("\n");
    let _ = std::fs::write(path, body);
}

/// Color-thief style extraction: downscale, bucket into 6-bit-per-channel cells,
/// score buckets by count × saturation (chroma), dedupe close colors, return the
/// top 3 distinct, saturated colors. Pure std + the `image` png/jpeg decoders.
fn extract_palette(path: &str) -> Option<Vec<[u8; 3]>> {
    let img = image::open(path).ok()?;
    // Downscale to a bounded sample (max ~120 px on the long edge) — dominant
    // colors don't need full resolution and this keeps the cost ~ms.
    let (w, h) = (img.width(), img.height());
    let scale = (w.max(h) as f32 / 120.0).max(1.0);
    let tw = (w as f32 / scale).max(1.0) as u32;
    let th = (h as f32 / scale).max(1.0) as u32;
    let thumb = img.thumbnail(tw, th).to_rgb8();

    let mut buckets: std::collections::HashMap<u32, (u32, [u64; 3])> =
        std::collections::HashMap::new();
    let mut total = 0u32;
    for px in thumb.pixels() {
        let [r, g, b] = px.0;
        // 6 bits per channel → 18-bit bucket key.
        let key = (u32::from(r) >> 2) << 12 | (u32::from(g) >> 2) << 6 | (u32::from(b) >> 2);
        let e = buckets.entry(key).or_insert((0, [0, 0, 0]));
        e.0 += 1;
        e.1[0] += u64::from(r);
        e.1[1] += u64::from(g);
        e.1[2] += u64::from(b);
        total += 1;
    }
    if total == 0 {
        return None;
    }

    // Score = frequency × chroma (max−min channel) — favors vivid colors that
    // dominate the image; gray/desaturated walls score low.
    let mut scored: Vec<([u8; 3], f64)> = buckets
        .into_iter()
        .map(|(_, (count, sum))| {
            let n = count.max(1) as f64;
            let avg = [
                (sum[0] as f64 / n).round() as u8,
                (sum[1] as f64 / n).round() as u8,
                (sum[2] as f64 / n).round() as u8,
            ];
            let max = *avg.iter().max().unwrap();
            let min = *avg.iter().min().unwrap();
            let chroma = f64::from(max) - f64::from(min);
            let freq = f64::from(count) / f64::from(total);
            (avg, freq * (chroma + 1.0))
        })
        .collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Greedy distinct pick: keep a color if it's far enough from every pick so
    // far (L∞ distance > 32 per channel-ish, summed > 72).
    let mut picks: Vec<[u8; 3]> = Vec::new();
    for (c, _) in scored {
        if picks.len() >= 3 {
            break;
        }
        let far = picks.iter().all(|p| {
            (i32::from(c[0]) - i32::from(p[0])).abs()
                + (i32::from(c[1]) - i32::from(p[1])).abs()
                + (i32::from(c[2]) - i32::from(p[2])).abs()
                > 72
        });
        if far {
            picks.push(c);
        }
    }
    if picks.len() < 2 {
        // Too few distinct colors (flat image): take what we have.
        return None;
    }
    Some(picks)
}

/// Build a full ThemeStrings from a palette. Slot mapping (fastfetch-ish):
/// title = bold #1, keys = #1, values = #2, sep = #3 (dim), section = #1.
/// gradient uses the same palette stops, so the logo blends with the wallpaper.
fn build_theme(colors: &[[u8; 3]]) -> ThemeStrings {
    let title = crate::theme::truecolor(colors[0], true);
    let keys = crate::theme::truecolor(colors[0], false);
    let values = crate::theme::truecolor(colors[1], false);
    let sep = crate::theme::truecolor(colors.get(2).copied().unwrap_or([90, 90, 90]), false);
    let section = crate::theme::truecolor(colors[0], true);
    ThemeStrings {
        title,
        keys,
        values,
        sep,
        section,
        reset: "\x1b[0m",
        gradient: true,
        gradient_colors: colors.to_vec(),
    }
}

/// Compute the auto theme from the current wallpaper (cached to /tmp).
/// Returns None when no wallpaper is found, it can't be decoded, or the palette
/// is too flat — callers should fall back to a preset.
pub fn auto_theme() -> Option<ThemeStrings> {
    // The extracted theme is truecolor-only; without 24-bit support the preset
    // fallback (16-color ANSI) is strictly better.
    if !crate::theme::supports_truecolor() {
        return None;
    }
    // Feature-gated and off the hot render path, so a throwaway RealFs
    // Context is fine here: the wallpaper config reads are user config, and
    // the image decode + cache mtime stay on std::fs regardless.
    let ctx = crate::Context::new(
        std::env::temp_dir().join("flexfetch-autotheme"),
        crate::get_cache_dir(),
        false,
        std::collections::HashMap::new(),
    );
    let wallpaper = crate::modules::wallpaper::detect_wallpaper(&ctx)?;
    let mtime = std::fs::metadata(&wallpaper).ok()?.modified().ok()?;
    let cache = cache_path(&wallpaper, mtime);

    if let Some(colors) = read_cache(&cache) {
        return Some(build_theme(&colors));
    }

    let colors = extract_palette(&wallpaper)?;
    write_cache(&cache, &colors);
    Some(build_theme(&colors))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_roundtrip() {
        let dir = std::env::temp_dir();
        let path = dir.join("flexfetch-autotheme-test-cache");
        let colors = [[10u8, 20, 30], [40, 50, 60], [70, 80, 90]];
        write_cache(&path, &colors);
        let got = read_cache(&path).expect("cache should parse");
        assert_eq!(got, colors);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn cache_path_changes_with_mtime() {
        let w = "/tmp/wallpaper.png";
        let t1 = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1000);
        let t2 = std::time::UNIX_EPOCH + std::time::Duration::from_secs(2000);
        assert_ne!(cache_path(w, t1), cache_path(w, t2));
        assert_ne!(cache_path(w, t1), cache_path("/tmp/other.png", t1));
    }

    #[test]
    fn build_theme_shape() {
        let colors = [[10u8, 20, 30], [40, 50, 60], [70, 80, 90]];
        let t = build_theme(&colors);
        assert!(t.title.starts_with("\x1b[1;38;2;10;20;30m"));
        assert!(t.keys.starts_with("\x1b[38;2;10;20;30m"));
        assert!(t.values.starts_with("\x1b[38;2;40;50;60m"));
        assert_eq!(t.gradient_colors.len(), 3);
    }
}
