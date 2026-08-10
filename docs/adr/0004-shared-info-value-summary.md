# InfoValue has one shared summary; exporters render through a common interface

`InfoValue::summary()` is the single flatten of all four value shapes into a one-line string, shared by the diff table, the GitHub export, and any other consumer. Exporters are free functions over the same `(SystemInfo, Config)` interface rather than a trait or visitor.

Two consequences. First, `summary()` lives on the type so no crate can grow a second copy that silently diverges — the CLI's diff table and core's exporters call the same method. Second, exporters stay plain functions because each target format differs enough that a shared abstraction would be speculative; the common seam is the `InfoValue` shapes and `render_lines`, not a trait. Revisit the trait only when a third category of exporter appears that can't be written as a function over the shared interface.
