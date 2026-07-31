use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct DnsModule;

impl Module for DnsModule {
    fn name(&self) -> &'static str {
        "dns"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut servers = Vec::new();
        let mut domain = String::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/etc/resolv.conf") {
                for line in content.lines() {
                    let line = line.trim();
                    if line.starts_with('#') || line.is_empty() {
                        continue;
                    }
                    if let Some(rest) = line.strip_prefix("nameserver") {
                        let addr = rest.trim();
                        if !addr.is_empty() {
                            servers.push(addr.to_string());
                        }
                    } else if let Some(rest) = line.strip_prefix("domain") {
                        domain = rest.trim().to_string();
                    } else if let Some(rest) = line.strip_prefix("search") {
                        if domain.is_empty() {
                            domain = rest.trim().to_string();
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(content) = std::fs::read_to_string("/etc/resolv.conf") {
                for line in content.lines() {
                    let line = line.trim();
                    if let Some(rest) = line.strip_prefix("nameserver") {
                        let addr = rest.trim();
                        if !addr.is_empty() {
                            servers.push(addr.to_string());
                        }
                    } else if let Some(rest) = line.strip_prefix("domain") {
                        domain = rest.trim().to_string();
                    }
                }
            }
        }

        if servers.is_empty() {
            return Ok(InfoValue::Scalar("unknown".into()));
        }

        let mut map = HashMap::new();
        map.insert("servers".into(), servers.join(", "));
        if !domain.is_empty() {
            map.insert("domain".into(), domain);
        }
        Ok(InfoValue::Map(map))
    }
}
