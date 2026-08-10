# Flexfetch

A blazing-fast, themeable system info fetch tool for Linux and macOS. Reads the system directly from `/proc`, sysfs, and platform APIs — zero subprocesses — then renders the result as a terminal logo + key/value block through a configurable template.

## Language

**Fetch**:
The whole read-and-render pipeline that produces a system information display: collect modules, render through a template, apply theme, draw the logo.
_Avoid_: output, print, run

**Module**:
A single named collector that reads one slice of system state (os, kernel, cpu, memory, disk, network, …) and returns exactly one `InfoValue`. Modules are the unit of composition — users select which modules run via template references and presets.
_Avoid_: collector, sensor, provider, widget

**InfoValue**:
The result of a module's collection, typed as one of four shapes: `Scalar` (a string), `Map` (key/value pairs, e.g. cpu vendor/model), `List` (a flat array of strings, e.g. mount points), or `Table` (rows of key/value pairs, e.g. processes). Every exporter and the diff table render these four shapes through a single shared summary.
_Avoid_: result, data, value (bare)

**Module catalog** (`MODULE_CATALOG`):
The single source of truth describing every built-in module — name, section, static-or-collected flag, label, and a builder. The registry, `--list-modules`, and default config all derive from it. Adding a module means adding one entry here plus the module's implementation.
_Avoid_: registry (the registry is derived from the catalog), module table

**Layout directive**:
A template-only token that controls presentation rather than collecting data — `title` and `separator`. They appear in templates and presets but are not modules: no `ModuleEntry`, no collection.
_Avoid_: module, virtual module

**Preset**:
A named grouping of modules for a display scenario — `default`, `minimal`, `full`, `dev`, `server`, `laptop`, `ci`, `neofetch`. Users can also define their own presets as TOML files in the config dir. Presets are the user-facing shorthand for "which modules do I want".
_Avoid_: profile, module group (a module group is the CLI's internal term for the same concept)

**Theme**:
A named color scheme applied at render time, defining per-slot ANSI strings (title, keys, values, separator, section) plus optional truecolor RGB and gradient stops. All built-in themes live in one table; `resolve` derives the final strings from the config with override/truecolor/ANSI fallback.
_Avoid_: color scheme (informal), palette

**Config**:
The user's `config.toml` merged with CLI flags. Holds module selection, display options (theme, gradient), and override values. `DisplayConfig` is the display-specific slice, merged field-by-field from CLI overrides via `merge()`.
_Avoid_: settings, options

**Context**:
The per-run carrier of working directories, caches, and feature flags passed to modules during collection. Modules read files through `ctx.read_file()` so collectors stay testable without touching real `/proc` or `/etc`.
_Avoid_: environment, state (bare)

**SystemInfo**:
The collected result of a run: an ordered list of `(module name, InfoValue)` entries, plus logo and layout hints. This is what templates and every exporter consume.
_Avoid_: output, report, snapshot

**Exporter**:
A function that renders a `SystemInfo` into a specific machine format — svg, png, html, markdown, csv, prometheus, github annotations, ansible facts, terraform HCL. Each is a free function over the same `(SystemInfo, Config)` interface.
_Avoid_: serializer, formatter, writer

**Logo**:
Distro ASCII art with per-line color hints. Flexfetch ships its own high-quality logos plus 527 imported from fastfetch; resolution prefers the taller of custom vs fastfetch art. Logo data lives in `logo_data.rs`, logic in `logo.rs`.
_Avoid_: ascii art, icon, emblem
