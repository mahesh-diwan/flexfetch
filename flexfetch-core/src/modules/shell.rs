use crate::{Context, InfoValue, Module, Result};
use std::path::Path;

pub struct ShellModule;

impl Module for ShellModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        let shell = std::env::var("SHELL")
            .ok()
            .and_then(|s| {
                Path::new(&s)
                    .file_name()
                    .map(|f| f.to_string_lossy().to_string())
            })
            .or_else(|| {
                let user = std::env::var("USER").ok()?;
                let content = ctx.read_file("/etc/passwd").ok()?;
                for line in content.lines() {
                    if line.starts_with(&format!("{}:", user)) {
                        let shell_path = line.split(':').nth(6)?;
                        return Path::new(shell_path)
                            .file_name()
                            .map(|f| f.to_string_lossy().to_string());
                    }
                }
                None
            })
            .unwrap_or_else(|| "unknown".to_string());

        Ok(InfoValue::Scalar(shell))
    }
}
