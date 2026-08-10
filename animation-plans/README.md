# Animation Plans

Recommended execution order, dependencies, and status.

| #   | Plan                             | Severity | Status                            |
| --- | -------------------------------- | -------- | --------------------------------- |
| 001 | Module grid hover + press states | MEDIUM   | DONE (d9b5e06)                    |
| 002 | Toast symmetric exit             | MEDIUM   | DONE (d30d4e2)                    |
| 003 | Reveal stagger via delay tokens  | LOW      | DONE (already shipped in 4eb4750) |
| 004 | Search input press feedback      | LOW      | DONE (bf13fd2)                    |
| 005 | Brand caret hover pop            | LOW      | DONE (bf13fd2)                    |

**Execution order:** 001 → 002 → 003 (no hard dependencies; order is by leverage).

**Dependencies:** none — each plan touches distinct CSS/JS blocks.

**Note:** Plan 003 was written against a stale snapshot; `.d1`–`.d6` stagger already shipped in commit 4eb4750. Plan 001 was re-written against current `global.css:921-934` after the first executor hit drift.
