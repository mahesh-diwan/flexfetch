# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Stack

Existing codebase answers this: Astro static site (`site/`), deployed at `https://mahesh-diwan.github.io/flexfetch/`. Landing + modules pages use shared design tokens in `site/src/styles/global.css`.

## Users

Confirmed audiences (multi-audience):

- **Developer customizing their terminal** — the ricing/tinkerer crowd who already use neofetch, fastfetch, or pfetch; care about theming, logo art, gradients, and layout control.
- **Pro/ops engineer fetching state** — wants the fetch fast on servers, plus exports, live TUI dashboard, diff mode, and SSH remote fetching.
- **Curious new user** — landed from GitHub or social, wants a one-line install and a pretty first fetch.

## Product Purpose

flexfetch is a blazing-fast, themeable system information tool for Linux and macOS. It reads the system directly from `/proc`, sysfs, and platform APIs with **zero subprocesses**, then renders the result as a terminal logo + key/value block through a configurable template. Success means a warm run completes in milliseconds from a single ~1.7 MB static binary.

## Positioning

Two claims flexfetch can truthfully lead with that neighbors (neofetch, fastfetch, pfetch) cannot copy outright:

1. **Zero subprocesses** — every collector reads kernel data directly; no forking, no parsing overhead, warm runs in milliseconds.
2. **Everything in one binary** — themes, 527+ logos, exports, live TUI, QR sharing, SSH remote fetch, plugin runtime — ~1.7 MB, static, no runtime dependencies.

## Operating Context

- Installed via a one-line `curl … | sh` installer (checksum + cosign signature verified) or `cargo install`.
- Configured through `~/.config/flexfetch/config.toml` or an interactive TUI wizard.
- Runs inside a terminal; the fetch is displayed, shared as an image/QR, or exported (text, JSON, Markdown, SVG, HTML, PNG, GitHub Actions).
- Plugins: Lua 5.4 dropped into `~/.config/flexfetch/plugins/`, or sandboxed WASM, installed from a signed registry.
- Releases are CI-built across 5 targets from git tags; never created manually.

## Capabilities and Constraints

Confirmed features: 38 built-in modules, 27 theme presets, 527+ distro logos, Lua + WASM plugins, Tera templates, live TUI dashboard, diff mode, QR sharing, SSH remote fetch, flash mode, custom modules (inline shell), 8 export formats, shell completions, health score, watch mode, config wizard, signed plugin registry.

Technical constraints: zero subprocesses (architecture, not just a flag); single static binary; Linux + macOS (Windows not a target); MIT licensed.

Terminology (from CONTEXT.md): **Fetch**, **Module**, **InfoValue**, **Module catalog**, **Layout directive**, **Preset**, **Theme**, **Config**, **Context**, **SystemInfo**, **Exporter**, **Logo**. Site copy must use these terms correctly.

## Brand Commitments

- Name: flexfetch (always lowercase).
- Tagline: "A fast, beautiful system information tool." Hero currently: "Your whole system. Fetched in milliseconds."
- Voice: honest, minimal, performance-first. No fabricated claims — a prior audit (v0.27.0) removed invented claims and AI-slop copy; do not reintroduce them.
- Visual commitments: dark terminal aesthetic, Instrument Serif + DM Mono + Space Grotesk typography, blueprint-dot grid, per-slot theme colors. (Recorded; visual redesign decisions belong in DESIGN.md, not here.)

## Evidence on Hand

- Real screenshot: `site/public/terminal-shot.png` (renders in the hero terminal).
- Real stats: 1.7 MB binary, 38 modules, 527 logos, 0 subprocesses, 27 themes.
- Real features list in `README.md` and `CHANGELOG.md` (do not exceed the changelog's claims).
- 14-page mdBook docs at `book/`, built into `site/docs`.

Absences to respect: no user testimonials, no install/download counts, no benchmarks beyond what the changelog actually records — never fabricate these.

## Product Principles

1. **Speed is the identity** — zero subprocesses, millisecond warm runs, 1.7 MB single binary. Every claim on the site must trace to a real, verifiable fact.
2. **One binary, everything included** — the full feature surface ships in the binary; the site can show breadth without overselling.
3. **Truthfulness over hype** — only claims the product actually makes appear; audits have cleaned fabricated copy twice and will again.
4. **The terminal is the product** — the site's terminal mock, ASCII art, and themes must look like real flexfetch output, because the audience is terminal people.
5. **Developers, ops, and newcomers all convert through install** — the one-liner is the CTA; the GitHub star is the secondary.

## Accessibility & Inclusion

No product-specific accessibility requirement has been established. The site should follow standard web accessibility (reduced-motion respected for animations, semantic markup, keyboard-usable copy buttons) without a formal commitment.
