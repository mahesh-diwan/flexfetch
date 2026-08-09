# 004 — Add button press feedback

- **Status**: TODO
- **Commit**: f122afc
- **Severity**: MEDIUM
- **Category**: Physicality & origin
- **Estimated scope**: 1 file (global.css), ~8 lines

## Problem

No `:active` press feedback on interactive elements. Buttons feel dead on click — no tactile response.

Per AUDIT.md: "Press feedback: `transform: scale(0.97)` on `:active` with `transition: transform 160ms ease-out`. Keep it subtle (0.95–0.98)."

## Target

Add press feedback to all button variants:

```css
/* global.css — target */
.btn:active {
  transform: scale(0.97);
}

.btn:active {
  transition: transform 160ms ease-out;
}
```

Also add to `.copy-btn` and `.hamburger`:

```css
.copy-btn:active {
  transform: scale(0.97);
  transition: transform 160ms ease-out;
}

.hamburger:active {
  transform: scale(0.95);
  transition: transform 160ms ease-out;
}
```

## Repo conventions to follow

- Buttons already have `transition: all 0.15s ease` — add `:active` after the existing hover rules
- Exemplar: none yet — this is the first press feedback in the codebase

## Steps

1. Open `site/src/styles/global.css`
2. After `.btn-ghost:hover` (line ~215), add:
   ```css
   .btn:active {
     transform: scale(0.97);
     transition: transform 160ms ease-out;
   }
   ```
3. After `.copy-btn:hover` (line ~280), add:
   ```css
   .copy-btn:active {
     transform: scale(0.97);
     transition: transform 160ms ease-out;
   }
   ```
4. After `.hamburger span` styles (line ~228), add:
   ```css
   .hamburger:active span {
     transform: scale(0.95);
     transition: transform 160ms ease-out;
   }
   ```

## Boundaries

- Do NOT add press feedback to the nav links (they're `<a>` not buttons)
- Do NOT change existing button styles — only add `:active` rules

## Verification

- **Mechanical**: `npm run build` succeeds
- **Feel check**: Click any button — subtle缩should feel responsive, not mushy. The缩should be barely noticeable (0.97 is 3%缩)
- **Done when**: All clickable buttons have `:active` with `scale(0.97)`
