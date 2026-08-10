---
target: flexfetch site homepage
total_score: 29
max_score: 40
na_heuristics: 
p0_count: 0
p1_count: 3
timestamp: 2026-08-10T10-26-19Z
slug: site-src-pages-index-astro
---
# flexfetch site critique — site/src/pages/index.astro

## Design Health Score
| # | Heuristic | Score | Key Issue |
|---|-----------|-------|-----------|
| 1 | Visibility of System Status | 3 | Copy button + toast + platform line excellent; copy failure still reports success |
| 2 | Match System / Real World | 4 | Real terminal render, /proc/sysfs language, exact curl one-liner |
| 3 | User Control and Freedom | 3 | Back-to-top, Esc-closes-menu; no jump-to-section nav |
| 4 | Consistency and Standards | 3 | Tokens disciplined, but dead install tab, JetBrains vs DM Mono, dead .stats-grid CSS |
| 5 | Error Prevention | 3 | Install verification messaging strong; few error surfaces |
| 6 | Recognition Rather Than Recall | 3 | Module glyphs help, but they're emoji (tofu risk on target terminals) |
| 7 | Flexibility and Efficiency | 2 | Search efficient but name-only; 4 duplicate Install links |
| 8 | Aesthetic and Minimalist Design | 3 | Cohesive, but redundant stats band + double checkmarks |
| 9 | Error Recovery | 2 | Clipboard failure silently swallowed, Copied regardless |
| 10 | Help and Documentation | 3 | Docs linked; nothing defines what is a fetch for newcomers |
| **Total** | | **29/40** | **Good** |

## Design Specificity Verdict
Content-specific, visual-template-generic. Content layer unmistakably flexfetch (zero subprocesses, 1.7MB musl-static, 38 modules, 527 logos, signed registry, --live TUI). Strongest anchors: real terminal-shot.png render + hand-built live-TUI mock on modules. But Instrument Serif + blueprint grid + orange/cyan gradient + reveal-stagger is a costume half the CLI-tool landing pages wear. Needs one visual motif that could only belong to flexfetch.

Deterministic scan: 1 CLI finding (overused-font, mis-attributed to Instrument Serif, real case Space Grotesk 20%) + 20 overlay anti-patterns homepage / 11 modules. Confirmed: white-on-orange 2.46:1, #6e6e80 faint 3.9-4.1:1, gradient-text ai-color-palette, tiny 11px text, 10px badge, line-length 107-153, skipped headings, bounce-easing, codex-grid-background.

## Overall Impression
Honest copy, real proof, disciplined micro-interaction — and a visual template that doesn't belong to flexfetch. Biggest opportunity: let the terminal output own the page.

## Priority Issues
- [P1] Primary CTA fails contrast: white on --accent #e8913a = 2.46:1. Fix: dark text on orange.
- [P1] Broken skip-link: Layout.astro:31 closes <a> with </div>. Phantom empty link in nav, .bg-grid swallowed.
- [P1] Faint text #6e6e80 fails AA (4.06:1 / 3.93:1): plat-hint, install-note, hero-stats, social-proof, footer h5.
- [P2] 1.2MB PNG for milliseconds audience: terminal-shot.png 1913x1024, below the fold on mobile. Fix: downscale + WebP.
- [P2] Redundant stats band + dead .stats-grid/.stat-value CSS: strip repeats hero numbers, rendered as 13px pills not 64px grid.
- [P2] Double checkmark: each .ok renders ✓ twice.
- [P2] No-JS blank sections: .reveal defaults opacity:0.

## Persona Red Flags
- Jordan (first-timer): terminal screenshot below the fold on mobile (~725px); single install tab leaves doubt.
- Sam (a11y): 11-12px faint text 3.9-4.1:1, white-on-orange 2.46:1, broken skip-link, skipped headings, no :focus-visible.
- Alex (power-user): two bands repeat same four numbers; single-tab install hides cargo install; emoji tofu risk.

## Minor Observations
4 links to #install; hero desktop bottom-misaligned (~167px dead space); unused public/hero.html; Space Grotesk 500/600 unused; mobile install wraps mid-URL; no closing CTA (arc ends on flat footer).

## Questions to Consider
1. Strongest proof is a 1.2MB image on a page pitching milliseconds — what would the site running as fast as the fetch demand?
2. Delete the stats band — would anyone notice?
3. Why is the last persuasive moment also the weakest?
