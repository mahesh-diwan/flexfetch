use crate::{Context, InfoValue, Module, Result};
use rayon::prelude::*;
use std::collections::HashMap;
use std::process::Command;

pub struct CustomCommandsModule;

impl Module for CustomCommandsModule {
    fn name(&self) -> &'static str {
        "custom"
    }

    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        let entries: Vec<_> = ctx
            .custom_modules
            .par_iter()
            .filter_map(|(key, custom)| {
                let parts: Vec<&str> = custom.command.split_whitespace().collect();
                if parts.is_empty() {
                    return None;
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
                        Some(row)
                    }
                    Err(e) => {
                        if ctx.debug {
                            eprintln!("[flexfetch] custom module {key} error: {e}");
                        }
                        None
                    }
                }
            })
            .collect();

        Ok(InfoValue::Table(entries))
    }
}
