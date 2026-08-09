# 005 — Fix reduced-motion accessibility

- **Status**: TODO
- **Commit**: f122afc
- **Severity**: MEDIUM
- **Category**: Accessibility
- **Estimated scope**: 1 file (global.css), ~10 lines

## Problem

The current `prefers-reduced-motion` implementation sets `transition-duration: 0.01ms` which is essentially instant — too abrupt for users who need reduced motion. Per AUDIT.md: "Reduced motion means fewer and gentler animations, not zero — keep transitions that aid comprehension, remove position changes."

Current code:

```css
/* global.css:1124-1133 */
@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
    scroll-behavior: auto !important;
  }
  .reveal {
    opacity: 1;
    transform: none;
  }
}
```

## Target

Keep opacity/color transitions for comprehension, skip transforms and position changes:

```css
/* target */
@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.1s !important;
    transition-property:
      opacity, color, background-color, border-color !important;
    scroll-behavior: auto !important;
  }
  .reveal {
    opacity: 1;
    transform: none;
  }
}
```

## Repo conventions to follow

- The `.reveal` rule already handles the scroll animation — keep it
- Exemplar: none — this is the only reduced-motion rule

## Steps

1. Open `site/src/styles/global.css`
2. Find the `@media (prefers-reduced-motion: reduce)` block (line ~1124)
3. Replace `transition-duration: 0.01ms !important;` with:
   ```css
   transition-duration: 0.1s !important;
   transition-property:
     opacity, color, background-color, border-color !important;
   ```
4. Keep the `.reveal` override as-is

## Boundaries

- Do NOT remove the `.reveal` rule — it correctly shows content without animation
- Do NOT change the `animation-duration` rules — keyframe animations should still be instant

## Verification

- **Mechanical**: `npm run build` succeeds
- **Feel check**: In DevTools, toggle `prefers-reduced-motion` (Rendering panel). Hover effects should still show color changes but without position/transform movement. The toast should still appear but without sliding.
- **Done when**: Reduced motion keeps color/opacity feedback, removes transforms
