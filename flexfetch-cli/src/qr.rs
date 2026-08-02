//! QR config sharing (Phase 4.11).
//!
//! `flexfetch --qr` renders the effective config as a terminal QR code and
//! `flexfetch --import-qr <image>` reads it back — a zero-setup way to move a
//! config between machines (or to a phone). Payload format: a magic header
//! followed by `base64(zstd(config_toml))`. Gated behind the `qr` feature
//! (opt-in, like `music`) so the minimal binary and the pure-Rust musl release
//! builds stay free of qrcode/rqrr/zstd/image.

use base64::prelude::BASE64_STANDARD;
use base64::Engine as _;
use qrcode::{Color, QrCode};
use std::path::Path;

/// Magic prefix so `--import-qr` can reject foreign QR payloads early.
const MAGIC: &str = "flexfetch-qr1:";
/// Quiet zone in modules (QR spec minimum is 4).
const QUIET: usize = 4;

/// Encode config TOML into a portable payload string (`MAGIC` + base64(zstd)).
fn encode_payload(toml: &str) -> Result<String, String> {
    let compressed =
        zstd::encode_all(toml.as_bytes(), 3).map_err(|e| format!("zstd compress failed: {e}"))?;
    Ok(format!("{MAGIC}{}", BASE64_STANDARD.encode(compressed)))
}

/// Decode a payload string back into config TOML.
fn decode_payload(payload: &str) -> Result<String, String> {
    let b64 = payload
        .strip_prefix(MAGIC)
        .ok_or_else(|| "not a flexfetch QR payload (missing header)".to_string())?;
    let compressed = BASE64_STANDARD
        .decode(b64)
        .map_err(|e| format!("base64 decode failed: {e}"))?;
    let toml = zstd::decode_all(compressed.as_slice())
        .map_err(|e| format!("zstd decompress failed: {e}"))?;
    String::from_utf8(toml).map_err(|e| format!("payload is not valid UTF-8: {e}"))
}

/// Render a QR code as 2×1 unicode blocks (like `qrencode -t ansiutf8`), with a
/// quiet zone. Terminal cells are ~2:1 tall, so 2 wide × 1 tall reads square.
fn render_unicode(code: &QrCode) -> String {
    let n = code.width();
    let colors = code.to_colors();
    let total = n + QUIET * 2;
    let mut out = String::with_capacity(total * (total * 2 + 1));
    for row in 0..total {
        for col in 0..total {
            let is_dark = row >= QUIET
                && row < QUIET + n
                && col >= QUIET
                && col < QUIET + n
                && colors[(row - QUIET) * n + (col - QUIET)] == Color::Dark;
            out.push_str(if is_dark { "██" } else { "  " });
        }
        out.push('\n');
    }
    out
}

/// `--qr`: serialize the effective config into a terminal QR code.
pub fn render_config_qr(toml: &str) -> Result<String, String> {
    let payload = encode_payload(toml)?;
    let code = QrCode::new(payload.as_bytes()).map_err(|e| format!("QR encode failed: {e}"))?;
    Ok(render_unicode(&code))
}

/// `--import-qr`: decode a QR code from an image file back into config TOML.
pub fn import_qr_image(path: &Path) -> Result<String, String> {
    let img = image::open(path).map_err(|e| format!("cannot open image {path:?}: {e}"))?;
    let luma = img.to_luma8();
    let (w, h) = luma.dimensions();
    // rqrr 0.3.2 pins image 0.23 while we use 0.25, so its `GrayImage` impl
    // won't match our buffer. `prepare_from_greyscale` builds rqrr's own
    // BasicImageBuffer from a closure — version-agnostic (0 = black, 255 = white).
    let mut prepared =
        rqrr::PreparedImage::prepare_from_greyscale(w as usize, h as usize, |x, y| {
            luma.get_pixel(x as u32, y as u32).0[0]
        });
    let grids = prepared.detect_grids();
    if grids.is_empty() {
        return Err("no QR code found in image".to_string());
    }
    let mut payload = None;
    for grid in grids {
        if let Ok((_, content)) = grid.decode() {
            payload = Some(content);
            break;
        }
    }
    let payload = payload.ok_or_else(|| "could not decode any QR code in image".to_string())?;
    decode_payload(&payload)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"[display]
theme = "dracula"
frame = "rounded"

[modules]
"#;

    #[test]
    fn payload_roundtrip() {
        let p = encode_payload(SAMPLE).unwrap();
        assert!(p.starts_with(MAGIC));
        assert_eq!(decode_payload(&p).unwrap(), SAMPLE);
    }

    #[test]
    fn rejects_foreign_payload() {
        assert!(decode_payload("not-a-flexfetch-payload").is_err());
    }

    #[test]
    fn qr_render_has_blocks_and_quiet_zone() {
        let rendered = render_config_qr(SAMPLE).unwrap();
        assert!(rendered.contains('█'));
        // First line is the quiet zone: all spaces.
        let first = rendered.lines().next().unwrap();
        assert!(!first.contains('█'));
    }

    #[test]
    fn image_roundtrip_via_rqrr() {
        // Build a grayscale QR image in memory (module = 8×8 px, quiet zone 4)
        // and decode it back with rqrr — a full encode→render→decode cycle.
        let p = encode_payload(SAMPLE).unwrap();
        let code = QrCode::new(p.as_bytes()).unwrap();
        let n = code.width();
        let colors = code.to_colors();
        let scale = 8usize;
        let dim = (n + QUIET * 2) * scale;
        let mut buf = vec![255u8; dim * dim];
        for y in 0..n {
            for x in 0..n {
                if colors[y * n + x] == Color::Dark {
                    for dy in 0..scale {
                        for dx in 0..scale {
                            let py = (y + QUIET) * scale + dy;
                            let px = (x + QUIET) * scale + dx;
                            buf[py * dim + px] = 0;
                        }
                    }
                }
            }
        }
        let path = std::env::temp_dir().join(format!("flexfetch-qr-{}.png", std::process::id()));
        image::save_buffer(&path, &buf, dim as u32, dim as u32, image::ColorType::L8).unwrap();
        let toml = import_qr_image(&path).unwrap();
        let _ = std::fs::remove_file(&path);
        assert_eq!(toml, SAMPLE);
    }
}
