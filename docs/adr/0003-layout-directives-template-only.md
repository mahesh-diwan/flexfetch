# Layout directives are template-only, not modules

`title` and `separator` appear in templates and presets but are not modules: they have no `ModuleEntry`, no `builder`, and never call `collect`. They are presentation tokens the renderer interprets directly.

We chose this because they don't collect system state — they control layout. Modeling them as modules would force every module list and the catalog to carry fake "modules" that produce no data, and would let users disable them like data modules. The cost: the renderer and template engine must know these two special names, and module-list tooling must skip them (`--list-modules`, benchmark, etc.). This is a deliberate boundary decision, not an accident — do not "fix" it by converting them into modules.
