# 002 — Replace transition:all with specific properties

- **Status**: TODO
- **Commit**: f122afc
- **Severity**: HIGH
- **Category**: Performance
- **Estimated scope**: 1 file (global.css), ~8 edits

## Problem

Four elements use `transition: all` which animates unintended properties off-GPU:

```css
/* global.css:228 — hero-install .copy-btn */
transition: all 0.15s ease;

/* global.css:275 — .copy-btn */
transition: all 0.15s ease;

/* global.css:406 — .mod */
transition: all 0.15s ease;

/* global.css:758 — .btt */
transition: all 0.3s ease;
```

## Target

Replace each with the specific properties being animated:

```css
/* hero-install .copy-btn — target */
transition:
  color var(--duration-fast) ease-out,
  border-color var(--duration-fast) ease-out;

/* .copy-btn — target */
transition:
  color var(--duration-fast) ease-out,
  border-color var(--duration-fast) ease-out;

/* .mod — target */
transition: background var(--duration-fast) ease-out;

/* .btt — target */
transition:
  opacity var(--duration-normal) ease-out,
  transform var(--duration-normal) ease-out,
  visibility var(--duration-normal) ease-out;
```

## Repo conventions to follow

- Use the tokens from plan 001 (`--duration-fast`, `--duration-normal`)
- Exemplar: `global.css:626-629` (`.feature-item`) — already uses specific properties

## Steps

1. Open `site/src/styles/global.css`
2. Find `.hero-install .copy-btn` (line ~244) and replace its `transition: all 0.15s ease` with:
   `transition: color var(--duration-fast) ease-out, border-color var(--duration-fast) ease-out;`
3. Find `.copy-btn` (line ~275) and replace its `transition: all 0.15s ease` with:
   `transition: color var(--duration-fast) ease-out, border-color var(--duration-fast) ease-out;`
4. Find `.mod` (line ~406) and replace its `transition: all 0.15s ease` with:
   `transition: background var(--duration-fast) ease-out;`
5. Find `.btt` (line ~758) and replace its `transition: all 0.3s ease` with:
   `transition: opacity var(--duration-normal) ease-out, transform var(--duration-normal) ease-out, visibility var(--duration-normal) ease-out;`

## Boundaries

- Do NOT touch `.hamburger span` (line 228) — its `transition: all 0.2s ease` is intentional for the open/close animation
- Do NOT change any markup or structure

## Verification

- **Mechanical**: `npm run build` succeeds
- **Feel check**: Hover over copy buttons, module grid items, and back-to-top button — transitions should feel identical or slightly snappier
- **Done when**: `grep -c "transition: all" global.css` returns 1 (only hamburger remains)
