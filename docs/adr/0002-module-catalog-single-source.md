# MODULE_CATALOG is the single source of truth for modules

`MODULE_CATALOG` in `flexfetch-core/src/module.rs` is the only place a built-in module is declared. `ModuleRegistry` auto-builds from it, `--list-modules` and the default config derive from it, and each entry carries a `builder` closure that constructs the module.

Previously the catalog and the registry were separate and could drift. Unifying them means adding a module is exactly one new catalog entry plus the module's implementation — the registry and CLI listings can never disagree about what exists. The trade-off: the catalog is now a static table with a builder closure per entry, which is less flexible than a hand-built registry but removes a whole class of "it builds but the module never runs" bugs.
