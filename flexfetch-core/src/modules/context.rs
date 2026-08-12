use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;
use std::path::Path;

pub struct ContextModule;

fn detect_container(ctx: &Context) -> Option<String> {
    // Docker
    if ctx.exists("/.dockerenv") {
        return Some("docker".into());
    }
    // Podman / other container runtimes
    if ctx.exists("/run/.containerenv") {
        return Some("podman".into());
    }
    // cgroup v1/v2 indicators
    if let Ok(cgroup) = ctx.read_file("/proc/1/cgroup") {
        if cgroup.contains("docker") {
            return Some("docker".into());
        }
        if cgroup.contains("lxc") || cgroup.contains("libpod") {
            return Some("container".into());
        }
    }
    None
}

impl Module for ContextModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        // Container
        if let Some(kind) = detect_container(ctx) {
            map.insert("container".into(), kind);
        }

        // Python virtualenv
        if let Ok(venv) = std::env::var("VIRTUAL_ENV") {
            let name = Path::new(&venv)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("venv")
                .to_string();
            map.insert("venv".into(), name);
        }

        // SSH session
        if std::env::var("SSH_CLIENT").is_ok() || std::env::var("SSH_CONNECTION").is_ok() {
            map.insert("ssh".into(), "connected".into());
        }

        Ok(InfoValue::Map(map))
    }
}
