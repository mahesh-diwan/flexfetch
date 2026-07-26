use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;
use std::process::Command;

pub struct CustomCommandsModule;

impl Module for CustomCommandsModule {
    fn name(&self) -> &'static str {
        "custom"
    }

    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        let mut rows = Vec::new();

        for (key, custom) in &ctx.custom_modules {
            // Split command safely without shell - parse into program + args
            let parts: Vec<&str> = custom.command.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            let (program, args) = (parts[0], &parts[1..]);

            let output = Command::new(program).args(args).output();
            match output {
                Ok(out) => {
                    let value = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    let label = custom.label.clone().unwrap_or_else(|| key.clone());
                    let mut row = HashMap::new();
                    row.insert("label".into(), label);
                    row.insert("value".into(), value);
                    rows.push(row);
                }
                Err(e) => {
                    if ctx.debug {
                        eprintln!("[flexfetch] custom module {key} error: {e}");
                    }
                }
            }
        }

        Ok(InfoValue::Table(rows))
    }
}
