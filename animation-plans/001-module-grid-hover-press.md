# 001 — Module grid hover + press states

- **Status**: TODO
- **Commit**: 804d424
- **Severity**: MEDIUM
- **Category**: Feedback / Missed opportunities
- **Estimated scope**: 1 file (site/src/styles/global.css), small

## Problem

The 38 module cards on `/modules` (`site/src/pages/modules.astro:20-58`, class `.mod`) are static divs with a background-color hover but no lift and no `:active` press feedback. They are the primary exploration surface of the page, yet nothing signals interactivity on hover (just a subtle bg tint) and the press gives zero confirmation.

Current `.mod` style (`site/src/styles/global.css:921-934`):

```css
.mod {
  display: flex;
  align-items: center;
  gap: 8px;
  background: var(--bg);
  padding: 12px 14px;
  font: 12px var(--font-mono);
  color: var(--text);
  transition: background var(--duration-fast) var(--ease-out);
}

.mod:hover {
  background: var(--surface);
}
```

## Target

Keep the existing background transition (don't remove it) and add a hover lift + a press scale. All values pulled from the repo's existing tokens (`--ease-out: cubic-bezier(0.16, 1, 0.3, 1)`, `--duration-fast: 150ms`) and matching the existing `.feature-item:hover` lift pattern at `site/src/styles/global.css:715-718`.

```css
/* target — replace the current .mod and .mod:hover rules */
.mod {
  display: flex;
  align-items: center;
  gap: 8px;
  background: var(--bg);
  padding: 12px 14px;
  font: 12px var(--font-mono);
  color: var(--text);
  transition:
    background var(--duration-fast) var(--ease-out),
    transform var(--duration-fast) var(--ease-out);
}

.mod:hover {
  background: var(--surface);
  transform: translateY(-1px);
}

.mod:active {
  transform: scale(0.98);
  transition: transform 100ms var(--ease-out);
}
```

## Repo conventions to follow

- Easing tokens live in `site/src/styles/global.css` `:root`: `--ease-out: cubic-bezier(0.16, 1, 0.3, 1)`, `--duration-fast: 150ms`.
- Exemplar: `.feature-item:hover` at `site/src/styles/global.css:715-718` — same `translateY(-1px)` + `border-color` lift pattern.
- Press feedback exemplar: `.btn:active` at `site/src/styles/global.css:363-366` — `transform: scale(0.97)`, `transition: transform 160ms var(--ease-out)`.

## Steps

1. In `site/src/styles/global.css`, find the `.mod` rule (line ~921). Replace its single `transition: background var(--duration-fast) var(--ease-out);` line with the two-property transition from Target.
2. Update the `.mod:hover` rule (line ~932) to add `transform: translateY(-1px);`.
3. Add the `.mod:active` rule directly after the `.mod:hover` rule.

## Boundaries

- Do NOT touch `site/src/pages/modules.astro` markup.
- Do NOT change `.mod-search`, `.mod-grid`, or any other class.
- Motion properties only — no layout changes.

## Verification

- **Mechanical**: `cd site && npm run build` — builds clean, no CSS errors.
- **Feel check**: open `/flexfetch/modules`, hover a module card:
  - Border brightens and card lifts 1px over 150ms `var(--ease-out)`.
  - Click (or touch-press) → card scales to 0.98 over 100ms, releases smoothly.
  - In DevTools Animations panel at 10% speed: hover and press transitions are distinct (150ms vs 100ms).
- **Done when**: cards visibly lift on hover and scale on press, matching the site's existing feedback language.

**Reduced motion**: this is a transform-only interaction on a hover/press — the existing site has no global reduced-motion override for these, and the motion is within the acceptable near-imperceptible range for its frequency tier. No change needed. If a global reduced-motion block is later added, gate the `translateY`/`scale` transforms behind `@media (prefers-reduced-motion: reduce)` while keeping the `border-color` change.
