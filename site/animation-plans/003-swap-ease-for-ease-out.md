# 003 — Swap bare ease for ease-out on hover effects

- **Status**: TODO
- **Commit**: f122afc
- **Severity**: HIGH
- **Category**: Easing & duration
- **Estimated scope**: 1 file (global.css), ~6 edits

## Problem

Multiple hover transitions use bare `ease` which starts slow — the exact moment the user is watching. Per AUDIT.md: "ease-in on UI is always a finding" and bare `ease` is too weak for deliberate motion.

Affected lines:

- `global.css:201` — `.nav-links a` color transition
- `global.css:474` — `.inst` border-color
- `global.css:818` — `.foot-col a` color
- `global.css:911` — `.foot-bottom a` color
- `global.css:930` — `.social-proof a` color

## Target

Replace `ease` with `ease-out` on all hover color/border transitions:

```css
/* target — e.g. .nav-links a */
transition: color var(--duration-fast) ease-out;
```

## Repo conventions to follow

- Use `--duration-fast` token from plan 001
- Exemplar: `global.css:626-629` — `.feature-item` already uses explicit properties with `ease` (which we'll also fix)

## Steps

1. Open `site/src/styles/global.css`
2. Replace all `transition: color 0.15s ease` with `transition: color var(--duration-fast) ease-out`
3. Replace `transition: border-color 0.15s ease` with `transition: border-color var(--duration-fast) ease-out`
4. Replace `transition: background 0.15s ease` with `transition: background var(--duration-fast) ease-out`
5. Update `.feature-item` transitions (lines 626-629) to use `ease-out` instead of `ease`
6. Update `.inst` transitions (lines 682-684) to use `ease-out`

## Boundaries

- Do NOT change the reveal animation easing (`cubic-bezier(0.16, 0.7, 0.3, 1)`) — it's already correct
- Do NOT change `.btn` transitions — those are handled in plan 004

## Verification

- **Mechanical**: `npm run build` succeeds
- **Feel check**: Hover over nav links, footer links, feature cards — color changes should feel snappier (starts fast)
- **Done when**: `grep "transition.*ease[^-]" global.css` returns 0 matches (no bare `ease` left)
