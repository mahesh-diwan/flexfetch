# Two-crate workspace split (core library + thin CLI)

The workspace is split into `flexfetch-core` (the library: config, modules, template, theme, export, logo) and `flexfetch-cli` (the thin binary: argument parsing, subcommand dispatch, config loading, output rendering). The CLI dispatches into core functions rather than implementing logic.

We chose this so the collection/render engine is reusable and unit-testable without spawning a CLI, and so the CLI stays a thin layer over one API surface. The cost is a defined `flexfetch-core` public API that must not break — core is the contract, the CLI is a consumer like any other. This is hard to reverse: collapsing back to one crate would couple engine to argument parsing again.
