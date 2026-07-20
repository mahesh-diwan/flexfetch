# Task 1 Report: Workspace scaffold + core trait types

## Implemented

- Cargo workspace root `Cargo.toml` with 3 members (sysfetch-core, sysfetch-cli, sysfetch-lua)
- `sysfetch-core/Cargo.toml` — deps: serde, serde_json, toml, tera, rayon, mlua (opt), chrono, walkdir; features: lua
- `sysfetch-cli/Cargo.toml` — deps: sysfetch-core (lua feat), clap, serde_json; binary crate
- `sysfetch-lua/Cargo.toml` — deps: sysfetch-core, mlua; lib crate (stub)
- `sysfetch-core/src/error.rs` — Error enum (Io, Parse, Config, Template, Lua, Module), Display impl, From<Io>, Result alias
- `sysfetch-core/src/module.rs` — InfoValue enum (Scalar/Map/List/Table), Module trait (name, collect), SystemInfo struct (entries, add, to_json)
- `sysfetch-core/src/context.rs` — Context struct (config_dir, cache_dir, debug), new()
- `sysfetch-core/src/lib.rs` — module declarations + public re-exports
- `.gitignore` — /target, Cargo.lock

## Tested

`cargo build -p sysfetch-core` — compiled successfully (1 crate, 0 warnings)

## Files changed

```
A  .gitignore
A  .superpowers/sdd/task-1-brief.md (pre-existing)
A  AGENTS.md (pre-existing)
A  Cargo.toml
A  docs/superpowers/plans/2026-07-20-flexfetch-implementation.md (pre-existing)
A  sysfetch-cli/Cargo.toml
A  sysfetch-cli/src/main.rs
A  sysfetch-core/Cargo.toml
A  sysfetch-core/src/context.rs
A  sysfetch-core/src/error.rs
A  sysfetch-core/src/lib.rs
A  sysfetch-core/src/module.rs
A  sysfetch-lua/Cargo.toml
A  sysfetch-lua/src/lib.rs
```

## Self-review findings

1. **Need for `crate::Context` import in module.rs** — `Module::collect` signature uses `&Context` but the import was missing from module.rs. Added `use crate::Context;`. Not in original brief but required for compilation.
2. **sysfetch-cli needed `src/main.rs`** — cargo requires at least one target (bin/lib) per manifest. Created empty `fn main() {}` stub.
3. **sysfetch-lua needed `src/lib.rs`** — same reason. Created empty lib stub with `#![allow(dead_code)]`.
4. **sysfetch-cli TOML parse error was misleading** — error "failed to parse manifest" was actually "no targets specified", not a TOML syntax issue.

## Issues

None. Build passes cleanly.
