use std::collections::HashMap;

use mlua::{Function, Lua, Value};
use sysfetch_core::{Context, Error, InfoValue, Module, Result};

pub struct LuaModule {
    name: &'static str,
    script_path: std::path::PathBuf,
}

impl LuaModule {
    pub fn new(script_path: std::path::PathBuf) -> Self {
        let name = script_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("lua_plugin")
            .to_string();
        LuaModule {
            name: Box::leak(name.into_boxed_str()),
            script_path,
        }
    }
}

impl Module for LuaModule {
    fn name(&self) -> &'static str {
        self.name
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let lua = Lua::new();

        let result = (|| -> mlua::Result<InfoValue> {
            let code = std::fs::read_to_string(&self.script_path)
                .map_err(|e| mlua::Error::RuntimeError(format!("read plugin: {e}")))?;

            let api_table = lua.create_table()?;
            api_table.set(
                "read_file",
                lua.create_function(|_, path: String| {
                    std::fs::read_to_string(&path)
                        .map_err(|e| mlua::Error::RuntimeError(format!("read_file: {e}")))
                })?,
            )?;
            api_table.set(
                "run_command",
                lua.create_function(|_, cmd: String| {
                    let output = std::process::Command::new("sh")
                        .arg("-c")
                        .arg(&cmd)
                        .output()
                        .map_err(|e| mlua::Error::RuntimeError(format!("run_command: {e}")))?;
                    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
                })?,
            )?;
            api_table.set(
                "get_env",
                lua.create_function(|_, key: String| Ok(std::env::var(&key).unwrap_or_default()))?,
            )?;
            lua.globals().set("ctx", api_table)?;

            let chunk = lua.load(&code);
            let result: Value = chunk.eval()?;

            if let Value::Table(t) = result {
                if let Ok(func) = t.get::<Function>("collect") {
                    let ctx_table = lua.globals().get::<mlua::Table>("ctx")?;
                    let res: Value = func.call::<Value>(ctx_table)?;
                    return Ok(lua_value_to_info(res));
                }
            }

            Ok(InfoValue::Scalar("no collect function".into()))
        })();

        result.map_err(|e| Error::Lua(e.to_string()))
    }
}

fn lua_value_to_info(val: Value) -> InfoValue {
    match val {
        Value::String(s) => InfoValue::Scalar(s.to_string_lossy().to_string()),
        Value::Table(t) => {
            if let Ok(val_str) = t.get::<String>("value") {
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
