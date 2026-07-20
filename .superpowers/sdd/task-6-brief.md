# Task 6: Lua plugin system (sysfetch-lua crate)

**Files:**
- Create: `sysfetch-lua/src/lib.rs`
- Create: `tests/fixtures/plugins/hello.lua`

Note: sysfetch-lua/Cargo.toml exists from Task 1. It depends on sysfetch-core and mlua.

## sysfetch-lua/src/lib.rs

```rust
use std::collections::HashMap;
use std::path::Path;

use mlua::{Lua, Function, Value};
use sysfetch_core::{Module, InfoValue, Context, Result, Error};

pub struct LuaModule {
    name: String,
    script_path: std::path::PathBuf,
}

impl LuaModule {
    pub fn new(script_path: std::path::PathBuf) -> Self {
        let name = script_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("lua_plugin")
            .to_string();
        LuaModule { name, script_path }
    }
}

impl Module for LuaModule {
    fn name(&self) -> &'static str {
        Box::leak(self.name.clone().into_boxed_str())
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let lua = Lua::new();
        let result = lua.context(|lua_ctx| -> mlua::Result<InfoValue> {
            let code = std::fs::read_to_string(&self.script_path)
                .map_err(|e| mlua::Error::RuntimeError(format!("read plugin: {e}")))?;

            let api_table = lua_ctx.create_table()?;
            api_table.set("read_file", lua_ctx.create_function(|_, path: String| {
                std::fs::read_to_string(&path)
                    .map_err(|e| mlua::Error::RuntimeError(format!("read_file: {e}")))
            })?)?;
            api_table.set("run_command", lua_ctx.create_function(|_, cmd: String| {
                let output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&cmd)
                    .output()
                    .map_err(|e| mlua::Error::RuntimeError(format!("run_command: {e}")))?;
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            })?)?;
            api_table.set("get_env", lua_ctx.create_function(|_, key: String| {
                Ok(std::env::var(&key).unwrap_or_default())
            })?)?;
            lua_ctx.globals().set("ctx", api_table)?;

            let chunk = lua_ctx.load(&code);
            let result: Value = chunk.eval()?;

            if let Value::Table(t) = result {
                if let Ok(func) = t.get::<_, Function>("collect") {
                    let res: Value = func.call::<_, Value>(lua_ctx.globals().get::<_, mlua::Table>("ctx")?)?;
                    return Ok(lua_value_to_info(res));
                }
            }

            Ok(InfoValue::Scalar("no collect function".into()))
        });

        result.map_err(|e| Error::Lua(e.to_string()))
    }
}

fn lua_value_to_info(val: Value) -> InfoValue {
    match val {
        Value::String(s) => InfoValue::Scalar(s.to_string_lossy().to_string()),
        Value::Table(t) => {
            if let Ok(val_str) = t.get::<_, String>("value") {
                return InfoValue::Scalar(val_str);
            }
            let mut map = HashMap::new();
            for pair in t.pairs::<Value, Value>() {
                if let Ok((k, v)) = pair {
                    map.insert(format_value(&k), format_value(&v));
                }
            }
            if map.is_empty() {
                InfoValue::Scalar("table".into())
            } else {
                InfoValue::Map(map)
            }
        }
        Value::Integer(i) => InfoValue::Scalar(i.to_string()),
        Value::Number(f) => InfoValue::Scalar(format!("{f}")),
        Value::Boolean(b) => InfoValue::Scalar(if b { "yes".into() } else { "no".into() }),
        _ => InfoValue::Scalar("?".into()),
    }
}

fn format_value(val: &Value) -> String {
    match val {
        Value::String(s) => s.to_string_lossy().to_string(),
        Value::Integer(i) => i.to_string(),
        Value::Number(f) => format!("{f}"),
        Value::Boolean(b) => (if *b { "yes" } else { "no" }).to_string(),
        _ => "?".to_string(),
    }
}
```

## hello.lua test plugin

```bash
mkdir -p tests/fixtures/plugins
```
Create `tests/fixtures/plugins/hello.lua`:
```lua
return {
    name = "hello_test",
    collect = function(ctx)
        local user = ctx.get_env("USER")
        return { value = "Hello from Lua, " .. user .. "!", type = "scalar" }
    end
}
```

## Verify compilation

Run: `cargo build -p sysfetch-lua`

## Commit

```bash
git add -A && git commit -m "feat: Lua plugin system"
git push
```
