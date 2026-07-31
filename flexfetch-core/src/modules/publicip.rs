use crate::{Context, InfoValue, Module, Result};
use std::process::Command;

pub struct PublicIpModule;

impl Module for PublicIpModule {
    fn name(&self) -> &'static str {
        "publicip"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let out = Command::new("curl")
            .args(["--silent", "--max-time", "5", "https://api.ipify.org"])
            .output();
        match out {
            Ok(o) if o.status.success() => {
                let ip = String::from_utf8_lossy(&o.stdout).trim().to_string();
                if !ip.is_empty() {
                    Ok(InfoValue::Scalar(ip))
                } else {
                    Ok(InfoValue::Scalar("unknown".into()))
                }
            }
            _ => Ok(InfoValue::Scalar("unknown".into())),
        }
    }
}
