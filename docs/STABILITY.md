# Flexfetch Stability Policy

Flexfetch is a long-lived CLI that scripts, prompts, panels, and dotfiles
depend on. This policy is a promise about what can and cannot change between
releases. It is also deliberate counter-positioning: fastfetch's recurring
breaking-change churn (case-sensitive config keys in 2.50, per-module flag
removals, `preRun` removal in 2.67) is a real cost for its users.

## Never breaks (any release)

- **CLI flags** — an existing flag keeps its name, semantics, and output
  contract. Flags may be added; never removed or repurposed.
- **Config keys** — every key in `config.toml` documented for a released
  version keeps working. Unknown keys warn but never fail.
- **JSON export schema** (`-f json`) — existing fields keep their names,
  types, and semantics. New fields may be added (consumers must ignore
  unknowns). Schema changes are announced one minor release ahead via a
  deprecation note in the changelog.
- **Exit codes** — 0 success, non-zero failure classes as documented.
- **Module names** — a module referenced in `modules = [...]` or a template
  keeps its name and its `InfoValue` shape (Scalar/Map/List/Table).

## May change (minor releases, announced)

- Default module set, default theme, default template layout.
- Visual rendering details (spacing, icons, colors) — pixel-level output is
  not a stable interface.
- New modules, new themes, new exporters.

## Deprecation process

1. Deprecated behavior is marked in the changelog and warns at runtime.
2. Removal happens no earlier than two minor releases after deprecation.
3. Major releases (2.x) may break the above — with a migration guide.

_Version 1.1 · Aug 2026_
