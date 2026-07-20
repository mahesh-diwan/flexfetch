use crate::{Module, InfoValue, Context, Result};

pub struct KernelModule;

impl Module for KernelModule {
    fn name(&self) -> &'static str { "kernel" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let uname = std::process::Command::new("uname")
            .args(["-srm"])
            .output()
            .ok()
            .and_then(|o| {
                let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                if s.is_empty() { None } else { Some(s) }
            })
            .unwrap_or_else(|| "unknown".to_string());

        Ok(InfoValue::Scalar(uname))
    }
}
