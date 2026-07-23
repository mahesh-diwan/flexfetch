# flexfetch v0.6.0 — Visual Design Features

> **Goal:** Add 7 visual/UX features to flexfetch, filtered by terminal compatibility.

## Terminal Compatibility

All features must work in the majority of terminals. Fallbacks provided where support is partial.

| Feature | Support | Fallback |
|---------|---------|----------|
| Gradient (truecolor) | 90%+ terminals | Basic ANSI colors |
| Bar graphs (Unicode blocks) | 100% | Text percentages |
| Side-by-side logo | 100% (ASCII) | Current single-column |
| Section ordering | 100% (config) | Default order |
| New themes | 100% (config) | Existing 20 |
| Image protocols | Mixed per terminal | ASCII art fallback |
| Export formats | 100% (file output) | N/A |

## Features

### 1. Gradient Title in Template
- Wire existing `gradient_text()` from theme.rs into Tera template
- Config: `gradient: true`, `gradient_colors: ["#hex1", "#hex2"]`
- Fallback: use theme's `color_title` ANSI escape when gradient disabled or truecolor unsupported

### 2. Bar Graphs for Resources
- Visual progress bars for CPU, Memory, Swap, Disk, Battery
- Color-coded: green (<50%), yellow (50-80%), red (>80%)
- Characters: `█` filled, `░` empty
- Config: `display.bars: true` (default false)

### 3. Side-by-Side Logo Layout
- ASCII distro logo rendered left of info text
- Auto-detect logo from OS name
- Config: `logo_mode: "side"` (default "none" for current layout)
- Fallback: current single-column when logo_mode is "none"

### 4. Fix Kitty Protocol + Implement Sixel
- **Kitty:** Use correct `_Gf=100,t=f,a=T` protocol with chunked base64
- **Sixel:** Implement via image crate (convert PNG to Sixel escape sequences)
- **Block:** Unicode block characters `▀▄█` with ANSI colors (always works)
- Auto-detection: check env vars (KITTY_WINDOW_ID, ITERM_SESSION_ID, TERM)
- Fallback: ASCII art when no image protocol detected

### 5. Section Ordering
- Config: `display.section_order: ["hardware", "software", "system", "palette"]`
- Config: `display.hide_sections: ["palette"]`
- Default: system, software, hardware, palette

### 6. New Theme Presets (8)
- monokai, monokai-pro, ayu-dark, ayu-mirage
- palenight, material-ocean, kanagawa, mellow-purple
- Each defines: title, keys, values, sep, section ANSI colors

### 7. Export Formats
- `--export svg` — terminal-style SVG with ANSI-colored text
- `--export html` — styled terminal card (dark bg, monospace font)
- `--export png` — screenshot via image crate
- `-o <path>` for output file

## Implementation Phases

### Phase 1: Quick Wins (no new deps)
1. Gradient title wiring
2. Section ordering config
3. New theme presets

### Phase 2: Visual Overhaul
4. Side-by-side logo layout
5. Bar graphs for resources

### Phase 3: Image Protocols
6. Fix Kitty + implement Sixel + block fallback

### Phase 4: Export
7. SVG/HTML/PNG export

## Files Modified
- `flexfetch-core/src/template.rs` — gradient, bars, side-by-side, ordering
- `flexfetch-core/src/theme.rs` — 8 new theme presets
- `flexfetch-core/src/config.rs` — section_order, hide_sections, bars, logo_mode
- `flexfetch-core/src/image_logo.rs` — fix Kitty, implement Sixel, block fallback
- `flexfetch-core/src/logo.rs` — logo_width for side-by-side layout
- `flexfetch-core/src/export.rs` — NEW: SVG/HTML/PNG export
- `flexfetch-cli/src/main.rs` — --export flag
- `templates/default.tera` — gradient, bars, side-by-side support

## Testing
- Each feature tested with `flexfetch --benchmark`
- Image protocols tested on Kitty, iTerm2, foot (if available)
- Export tested by generating files and verifying output
- All existing tests must pass
- No new runtime deps except image crate (already in Cargo.toml)
