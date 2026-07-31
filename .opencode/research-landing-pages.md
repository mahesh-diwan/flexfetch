# Research: Does flexfetch need a landing page?

**Date:** 2026-07-31
**Status:** Complete

---

## Executive Summary

**No — flexfetch does not need a dedicated landing page right now.** The overwhelming majority of successful CLI tools (fastfetch, neofetch, bat, ripgrep) rely on their GitHub README as the primary landing experience. Developer tool adoption is driven by community channels (HN, Reddit, word of mouth — 70%+ of discovery), not by polished marketing sites. A well-crafted README beats a mediocre landing page every time.

---

## 1. Do popular CLI tools have dedicated landing pages?

| Tool               | Stars | Dedicated Landing Page?                                            | Primary Presence     |
| ------------------ | ----- | ------------------------------------------------------------------ | -------------------- |
| **fastfetch**      | 24k   | **No**                                                             | GitHub README        |
| **neofetch**       | 23.7k | **No**                                                             | GitHub README + wiki |
| **bat**            | 58.6k | **No**                                                             | GitHub README        |
| **ripgrep**        | 66.7k | **Unofficial only** (ripgrep.dev — not affiliated with BurntSushi) | GitHub README        |
| **exa**            | 24.4k | **Yes** — the.exa.website (minimal, clean)                         | Website + GitHub     |
| **eza** (exa fork) | 24k+  | **No**                                                             | GitHub README        |
| **lazygit**        | 55k+  | **No**                                                             | GitHub README        |
| **btop**           | 24k+  | **No**                                                             | GitHub README        |
| **starship**       | 47k+  | **Yes** — starship.rs                                              | Website + GitHub     |
| **delta**          | 25k+  | **Minimal** — dandavison.github.io/delta                           | GitHub README        |
| **zoxide**         | 25k+  | **No**                                                             | GitHub README        |
| **fzf**            | 67k+  | **No**                                                             | GitHub README        |

**Key finding:** Only ~20% of top CLI tools have dedicated landing pages. The ones that do (exa, starship) tend to be more "product-like" — they have distinct branding, a visual identity separate from GitHub, and often serve dual purposes (docs + marketing). Most tools thrive with just a great README.

---

## 2. Typical GitHub README structure for successful CLI tools

Based on analysis of fastfetch, bat, ripgrep, neofetch, and exa READMEs:

### Common sections (in order):

1. **Logo/badge + tagline** — one-liner describing what it does
2. **Screenshot/GIF** — visual proof of output
3. **Installation** — per-platform install commands (Arch, Ubuntu, macOS, Windows, etc.)
4. **Usage/Quick Start** — basic commands, common flags
5. **Features** — bullet list or screenshots
6. **Configuration** — how to customize
7. **FAQ** — common questions
8. **Contributing** — how to help
9. **License**

### What makes them effective:

- **Show, don't tell** — screenshots/GIFs of actual terminal output
- **Install in 30 seconds** — platform-specific one-liners right at the top
- **No marketing fluff** — technical, direct, no "revolutionary" or "game-changing"
- **Badges** — build status, stars, license, version
- **Links to deeper docs** — wiki, GUIDE.md, man pages

---

## 3. Do landing pages actually drive adoption for developer tools?

**Data says: not primarily.**

From a survey of 202 open-source developers at All Things Open 2025:

| Discovery Channel                          | % of Developers |
| ------------------------------------------ | --------------- |
| Tech social platforms (HN, Reddit, dev.to) | 30.2%           |
| Word of mouth                              | 20.3%           |
| General social media                       | 19.8%           |
| Meetups and events                         | 18.8%           |
| GitHub browsing                            | 10.9%           |

**70.1% of discovery is social or community-driven.** Only 10.9% discover tools by browsing GitHub.

What actually drives adoption:

- **Hacker News "Show HN" post** — single highest-impact event
- **Reddit posts** (r/commandline, r/linux, r/rust, r/unixporn)
- **Awesome lists** — inclusion in awesome-rust, awesome-cli, etc.
- **Package manager inclusion** — being in `pacman`, `brew`, `apt`
- **Word of mouth** — someone tweets about it, tells a friend

A landing page doesn't appear in any of the top discovery channels. A great README + HN post + Reddit launch does.

---

## 4. Pros/Cons: Landing Page vs Good README

### Landing Page

**Pros:**

- Branded experience (custom domain, design)
- Better SEO for non-GitHub audiences
- Can include interactive demos
- Signals maturity/professionalism
- Better for tools with paid tiers or SaaS components

**Cons:**

- Extra maintenance burden (separate codebase/deployment)
- Requires design skills or a template
- Creates another thing to keep in sync with the repo
- Most developers skip it and go to GitHub anyway
- Overkill for a CLI tool with no commercial component
- **YAGNI** — you don't need it until you have 10k+ users and a brand to maintain

### Good README

**Pros:**

- Lives where developers already are (GitHub)
- Zero extra deployment/maintenance
- Version-controlled alongside the code
- Instant credibility — stars, issues, activity visible
- Rendered beautifully on GitHub, npm, crates.io
- All successful CLI tools prove this works

**Cons:**

- Limited design control (GitHub's markdown rendering)
- No custom domain/branding
- Can't host interactive demos easily

---

## 5. Examples of excellent CLI tool landing pages (when they exist)

### exa (the.exa.website)

- Minimal, clean, dark theme
- One-line description + screenshot
- Installation instructions
- Links to GitHub for source
- **Verdict:** Works because exa is visual (colorful ls replacement). The screenshots sell it.

### starship.rs

- Full-featured site with docs, customization gallery
- Interactive prompt builder
- **Verdict:** Works because starship is a "product" — cross-shell prompt with deep customization. The website IS the docs.

### ripgrep.dev (unofficial)

- Clone of GitHub README content in a nicer layout
- Not affiliated with the actual project
- **Verdict:** Shows that even ripgrep's creator didn't think a landing page was worth building. Someone else made one to practice web design.

---

## Recommendation for flexfetch

### Do NOT build a landing page now. Instead:

1. **Polish the README** — this is your landing page
   - Add a hero screenshot/GIF showing flexfetch output
   - One-liner tagline
   - Install commands for all platforms (one-liners)
   - Feature highlights with terminal screenshots
   - Comparison with fastfetch/neofetch (if differentiation exists)

2. **Launch strategically** (when ready)
   - "Show HN" post on Hacker News (Tuesday-Thursday, 8-10 AM PT)
   - Reddit: r/commandline, r/linux, r/rust, r/unixporn
   - Post in relevant Discord/Slack communities
   - Submit to awesome-rust, awesome-cli lists

3. **Add a landing page later** when:
   - You have 5k+ GitHub stars and active community
   - You need a branded experience (custom domain, logo)
   - You have docs that outgrow GitHub wiki
   - You add a paid tier or SaaS component
   - You're building a "product" not just a tool

### When a landing page DOES make sense:

- The tool becomes a product with multiple components
- You need to reach non-GitHub audiences (DevOps managers, etc.)
- You have a brand to maintain
- You're monetizing (freemium, hosted version, etc.)

---

## Sources

- Evil Martians: "We studied 100 dev tool landing pages" (2025) — https://evilmartians.com/chronicles/we-studied-100-devtool-landing-pages-here-is-what-actually-works-in-2025
- Catchy Agency: "What 202 Open Source Developers Taught Us About Tool Adoption" (2025) — https://www.catchyagency.com/post/what-202-open-source-developers-taught-us-about-tool-adoption
- peal.dev: "Landing Page Optimization for Developer Tools" (2026) — https://www.peal.dev/blog/landing-page-developer-tools-conversion-what-works
- Nakora: "Developer landing pages: Best examples, templates and tips" (2025) — https://nakora.ai/blog/developer-landing-page
- GitHub repos: fastfetch, neofetch, bat, ripgrep, exa, starship, lazygit, btop, fzf, delta, zoxide
- Landing pages examined: the.exa.website, starship.rs, ripgrep.dev
