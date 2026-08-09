# 001 — Add easing and duration tokens

- **Status**: TODO
- **Commit**: f122afc
- **Severity**: LOW
- **Category**: Cohesion & tokens
- **Estimated scope**: 1 file (global.css), ~15 lines

## Problem

All easing and duration values are hardcoded throughout `global.css`. Six occurrences of `transition: all X ease` and multiple bare `ease` keywords. No shared tokens means inconsistent motion and harder future tuning.

## Target

Add CSS custom properties in `:root`:

```css
/* global.css :root — target */
--ease-out: cubic-bezier(0.23, 1, 0.32, 1);
--ease-in-out: cubic-bezier(0.77, 0, 0.175, 1);
--duration-fast: 150ms;
--duration-normal: 300ms;
```

## Repo conventions to follow

- Tokens live at the top of `global.css` in `:root` (existing pattern for `--bg`, `--accent`, etc.)
- Exemplar: `global.css:7-30` — the existing `:root` block

## Steps

1. Open `site/src/styles/global.css`
2. In the `:root` block (after `--shadow-lg`), add:
   ```css
   --ease-out: cubic-bezier(0.23, 1, 0.32, 1);
   --ease-in-out: cubic-bezier(0.77, 0, 0.175, 1);
   --duration-fast: 150ms;
   --duration-normal: 300ms;
   ```

## Boundaries

- Do NOT change any existing transition values yet — that's plan 002/003
- Do NOT touch any other files

## Verification

- **Mechanical**: `npm run build` succeeds
- **Feel check**: No visible change — tokens are definitions only
- **Done when**: `:root` contains the four new tokens
