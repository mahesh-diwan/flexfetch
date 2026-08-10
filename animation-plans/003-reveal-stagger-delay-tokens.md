# 003 — Reveal stagger via delay tokens

- **Status**: DONE (already implemented — `.d1`–`.d6` transition-delay stagger shipped in commit 4eb4750)
- **Commit**: 4eb4750
- **Severity**: LOW
- **Category**: Cohesion & tokens / Delight
- **Estimated scope**: 2 files (site/src/styles/global.css, site/src/pages/index.astro), small

## Obsolete — already implemented

This plan was written against `global.css` at commit 804d424, where `.d1`/`.d2` were no-op marker classes. The current `global.css` (post `4eb4750`) already ships the stagger:

- `.reveal.d1` – `.reveal.d6` exist as real `transition-delay` rules at `site/src/styles/global.css:1111-1128` (60ms steps: 0.06s, 0.12s, … 0.36s).
- `.feature-list .feature-item:nth-child(n)` block at `site/src/styles/global.css:1131-1148` handles the feature grid stagger.

The intended outcome (30–80ms stagger on grouped entrances, decorative, never blocking) is already achieved. **No work required. Mark DONE.**

(Original problem/target text preserved below for reference.)

---

## Problem

The reveal-on-scroll system (`site/src/styles/global.css:1086-1105`) animates every `.reveal` element with the same 600ms timing. Grouped content — the feature grid (`index.astro:78-121`, 6 cards) and the hero split (`index.astro:10,43`) — pops in all at once. AUDIT.md §7 notes a 30–80ms stagger belongs on grid/list group entrances (decorative, never blocking interaction).

Current CSS (`site/src/styles/global.css:1086-1105`):

```css
.reveal {
  opacity: 0;
  transform: translateY(8px);
  transition:
    opacity var(--duration-normal) var(--ease-out),
    transform var(--duration-normal) var(--ease-out);
}

.reveal.in {
  opacity: 1;
  transform: translateY(0);
}
```

The markup already uses `d1`/`d2` classes (e.g. `index.astro:10` `<div class="hero-text reveal in d1">`, `index.astro:43` `<div class="hero-term reveal d2">`, feature items `index.astro:79,86,93` use `reveal d1`/`reveal d2`).

## Target

Convert the existing `d1`/`d2`/`d3`/`d4` marker classes from no-ops into real delay tokens by adding a `--delay` variable consumed by the reveal transition. Keep 60ms steps (AUDIT.md §7: 30–80ms). Delays only apply when the element is _revealing_ (`.reveal:not(.in)`), so already-visible content never waits.

```css
/* target — add to global.css, replacing the current .reveal block */
.reveal {
  opacity: 0;
  transform: translateY(8px);
  transition:
    opacity var(--duration-normal) var(--ease-out) var(--delay, 0ms),
    transform var(--duration-normal) var(--ease-out) var(--delay, 0ms);
}

.reveal.in {
  opacity: 1;
  transform: translateY(0);
}

.reveal.d1 {
  --delay: 0ms;
}
.reveal.d2 {
  --delay: 60ms;
}
.reveal.d3 {
  --delay: 120ms;
}
.reveal.d4 {
  --delay: 180ms;
}
```

## Repo conventions to follow

- Transition tokens: `--duration-normal: 600ms`, `--ease-out: cubic-bezier(0.16, 1, 0.3, 1)` in `site/src/styles/global.css:31-35`.
- The `d1`/`d2` class convention already exists in markup (`index.astro:10,43`, `index.astro:79,86,93`, `modules.astro:70,78`). The plan reuses it — no markup changes.
- CSS variable + transition-delay pattern is native CSS; no JS changes.

## Steps

1. In `site/src/styles/global.css`, replace the `.reveal` block (lines 1086-1096) with the Target version that consumes `var(--delay, 0ms)`.
2. After `.reveal.in`, add the four `.reveal.d1` through `.reveal.d4` delay-token rules.

## Boundaries

- Do NOT touch `site/src/layouts/Layout.astro` reveal JS (IntersectionObserver logic stays).
- Do NOT add new delay classes to markup — `d1`–`d4` already exist.
- Do NOT change `--duration-normal` or `--ease-out`.
- Only the 4 marker classes; no `d5+`.

## Verification

- **Mechanical**: `cd site && npm run build` — builds clean.
- **Feel check**: hard-refresh `/flexfetch`:
  - The hero split enters together (d1 = 0ms, d2 = 60ms — 60ms gap is barely perceptible, not laggy).
  - Feature grid cards (d1/d2) enter in a soft wave, 60ms apart — a gentle stagger, not a cascade.
  - Scrolling to a `.reveal` that's already `.in` → it never waits for a delay.
  - In DevTools, elements already in the viewport on load keep the `requestAnimationFrame` force-reveal (no delay applied because `.in` is added synchronously).
- **Done when**: grouped reveals feel gently staggered but responsive, with no element visibly "waiting" before appearing.

**Reduced motion**: the reveal is `opacity` + `translateY(8px)` — a small movement. The site has no global reduced-motion override; a future one should keep the opacity fade and drop the `translateY`. The stagger delay itself is harmless under reduced motion (movement is already minimal).
