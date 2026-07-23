# flexfetch Enhancement Plan

> **For agentic workers:** Use subagent-driven-development or executing-plans to implement this plan.

**Goal:** Enhance flexfetch with more themes, better logos, faster performance, and deeper customization — while staying lean.

**Architecture:** Extend existing theme/logo/config systems in flexfetch-core. No new crates. Research-driven additions from fastfetch/neofetch/macchina patterns.

**Tech Stack:** Rust 2021, Tera templates, rayon, mlua. No new dependencies.

---

## Phase 1: Research (Tasks 1-5, parallel)

### Task 1: Theme Research

**Files:** Read-only research, output to `docs/superpowers/research/themes.md`

Research targets:
- fastfetch: theme structure (JSONC), color slots, presets directory
- neofetch: distro-based auto-coloring, 6-element color arrays
- macchina: TOML theme files, per-field color overrides
- Popular schemes: Catppuccin, Dracula, Nord, Gruvbox, Tokyo Night, Solarized, Rosé Pine, Everforest, Bamboo, Oxocarbon
- Gradient support: how tools do rainbow/gradient across lines

**Deliverable:** Markdown doc with 10+ new preset color schemes, gradient approach, per-field override patterns.

### Task 2: Logo Research

**Files:** Read-only research, output to `docs/superpowers/research/logos.md`

Research targets:
- fastfetch: logo system (Pascal source → image data, 100+ distros, small/medium/large)
- neofetch: ASCII art database (300+ distros), color mapping per logo
- pfetch: minimal logos, colored ASCII
- How to detect distro beyond /etc/os-release (lsb_release, /etc/os-version)
- Colored ASCII art: per-line or per-character ANSI coloring

**Deliverable:** Markdown doc with distro detection strategy, logo sizing, color mapping approach, list of most popular distros to support.

### Task 3: Performance Research

**Files:** Read-only research, output to `docs/superpowers/research/performance.md`

Research targets:
- fastfetch: lazy module loading, parallel collection, benchmark mode
- Current bottlenecks: all modules always loaded, no caching between runs, Tera template parsed every time
- Template caching: compile once, reuse
- Module skip lists: only run what the template uses
- Binary size: current 5.1MB, targets

**Deliverable:** Markdown doc with prioritized optimization list, estimated impact.

### Task 4: Customization Research

**Files:** Read-only research, output to `docs/superpowers/research/customization.md`

Research targets:
- fastfetch: config cascade (global → user → project → CLI), presets system
- neofetch: simple key=value config
- macchina: TOML with defaults override
- Per-module color/format overrides
- Config file discovery: XDG, platform conventions

**Deliverable:** Markdown doc with config cascade design, per-module override patterns.

### Task 5: Integration Research

**Files:** Read-only research, output to `docs/superpowers/research/integration.md`

Research targets:
- What makes fastfetch successful (speed, configurability, presets, community themes)
- What makes neofetch successful (simplicity, portability, ASCII art culture)
- What pfetch/macchina do well (minimalism, Rust ecosystem)
- Design patterns to adopt: preset system, color scheme sharing, module toggle groups
- Keep lean: what NOT to add (avoid bloat)

**Deliverable:** Markdown doc with ranked features to adopt, anti-patterns to avoid.

---

## Phase 2: Implementation (Tasks 6-12, sequential)

### Task 6: Enhanced Theme System

**Files:**
- Modify: `flexfetch-core/src/theme.rs`
- Modify: `flexfetch-core/src/config.rs`

Add 5-10 new theme presets (Solarized, Rosé Pine, Everforest, Bamboo, Oxocarbon). Add gradient support for title line. Per-module color overrides in config.

### Task 7: Dynamic Logo System

**Files:**
- Modify: `flexfetch-core/src/logo.rs`

Expand distro detection: lsb_release, /etc/os-version fallback. Add 10+ more distro logos. Add colored ASCII support. Auto-pad logo to match info line count (remove hardcoded empty lines).

### Task 8: Performance Optimizations

**Files:**
- Modify: `flexfetch-core/src/template.rs`
- Modify: `flexfetch-core/src/module_registry.rs`

Template caching (compile once). Module skip list (only run modules the template references). Benchmark mode (`--benchmark`).

### Task 9: Config Cascade

**Files:**
- Modify: `flexfetch-core/src/config.rs`

Config file discovery: XDG_CONFIG_HOME/flexfetch/config.toml → ~/.config/flexfetch/config.toml → ./flexfetch.toml → CLI overrides. Preset configs in themes/ directory.

### Task 10: Research Integration

**Files:** As needed based on research findings

Integrate best patterns from research. Add preset command (`--preset <name>`). Add `--list-themes` and `--list-logos` commands.

### Task 11: Verification

Run full test suite. Build release. Test all themes. Test all logos. Benchmark performance. Test config cascade.

### Task 12: Polish

Update README with new features. Update default.svg with new output. Tag release. Update install.sh if needed.