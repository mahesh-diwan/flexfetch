# Animation Audit — flexfetch landing page

**Stack**: Plain CSS transitions + IntersectionObserver scroll reveals. No animation libraries.
**Personality**: Developer tool, terminal aesthetic — should stay crisp, not playful.
**Motion surface**: `global.css` (1135 lines), Layout.astro (inline JS for reveals, toast, nav).

## Findings

| #   | Severity | Category      | Location                     | Finding                                                                  | Fix                                                                              |
| --- | -------- | ------------- | ---------------------------- | ------------------------------------------------------------------------ | -------------------------------------------------------------------------------- |
| 1   | HIGH     | Easing        | `global.css` (6 occurrences) | `transition: all X ease` — bare `ease` starts slow on hover/interactions | Replace `ease` with `ease-out` for entries, keep `ease` for color-only           |
| 2   | HIGH     | Performance   | `global.css:228,275,406,758` | `transition: all` animates unintended properties off-GPU                 | List specific properties (color, border-color, background, box-shadow)           |
| 3   | MEDIUM   | Physicality   | `.btn`, `.copy-btn`          | No `:active` press feedback on interactive elements                      | Add `transform: scale(0.97)` on `:active` with 160ms ease-out                    |
| 4   | MEDIUM   | Accessibility | `global.css:1126-1128`       | `prefers-reduced-motion` sets `transition-duration: 0.01ms` — too abrupt | Keep opacity transitions at 0.1s, only skip transforms                           |
| 5   | LOW      | Cohesion      | Global                       | No easing/duration tokens — all values hardcoded                         | Add `--ease-out`, `--ease-in-out`, `--duration-fast`, `--duration-normal` tokens |

## Missed opportunities

1. **Feature list stagger**: Items have `transition-delay` via `.d1`/`.d2` classes but no intentional stagger pattern — could be more deliberate (30-80ms increments)
2. **Toast bounce**: Toast slides up with `ease` — a subtle spring would feel more alive
3. **Button press feedback**: No tactile response on click — `scale(0.97)` would add polish

## Recommended execution order

1. **001** — Add easing/duration tokens (foundation for all other fixes)
2. **002** — Replace `transition: all` with specific properties
3. **003** — Swap bare `ease` for `ease-out` on hover effects
4. **004** — Add button press feedback
5. **005** — Fix reduced-motion accessibility

## Dependencies

- 001 must come first (other plans reference the tokens)
- 002 and 003 can run in parallel
- 004 and 005 are independent
