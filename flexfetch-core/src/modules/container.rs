use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;
#[cfg(unix)]
use std::io::{Read, Write};
use std::path::Path;

pub struct ContainerModule;

impl Module for ContainerModule {
    fn name(&self) -> &'static str {
        "container"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        // Phase 4.15: deep container introspection. Only meaningful inside a
        // container (or when a docker/podman socket is mounted).
        let mut map = HashMap::new();

        let runtime = detect_runtime();
        let container_id = container_id();

        if runtime.is_none() && container_id.is_none() {
            // Not in a container and no socket: empty map → line omitted.
            return Ok(InfoValue::Map(map));
        }

        if let Some(id) = &container_id {
            map.insert("id".into(), id.clone());
        }

        // Docker / Podman socket: query the engine API for the current container.
        if let Some(info) = socket_container_info(&container_id) {
            for (k, v) in info {
                map.insert(k, v);
            }
        }

        // Kubernetes serviceaccount metadata.
        if let Some(ns) = read_first_line("/var/run/secrets/kubernetes.io/serviceaccount/namespace")
        {
            map.insert("k8s_namespace".into(), ns);
        }
        if let Ok(pod) = std::env::var("HOSTNAME") {
            if !pod.is_empty() {
                map.insert("k8s_pod".into(), pod);
            }
        }

        if let Some(r) = runtime {
            map.insert("runtime".into(), r);
        }

        Ok(InfoValue::Map(map))
    }
}

fn detect_runtime() -> Option<String> {
    if Path::new("/.dockerenv").exists() {
        return Some("docker".into());
    }
    if Path::new("/run/.containerenv").exists() {
        return Some("podman".into());
    }
    // Cgroup v1/v2 mention of containers.
    if let Ok(cgroup) = std::fs::read_to_string("/proc/self/cgroup") {
        if cgroup.contains("docker") {
            return Some("docker".into());
        }
        if cgroup.contains("libpod") || cgroup.contains("podman") {
            return Some("podman".into());
        }
    }
    None
}

/// Container ID from /proc/self/cgroup (64-hex digest in any of the segments).
fn container_id() -> Option<String> {
    let cgroup = std::fs::read_to_string("/proc/self/cgroup").ok()?;
    parse_container_id(&cgroup)
}

/// Extract the first 64-hex container id from a cgroup file's contents.
fn parse_container_id(cgroup: &str) -> Option<String> {
    for line in cgroup.lines() {
        for seg in line.split('/') {
            let seg = seg.trim();
            if seg.len() == 64 && seg.chars().all(|c| c.is_ascii_hexdigit()) {
                return Some(seg.chars().take(12).collect());
            }
        }
    }
    None
}

/// Query the docker/podman Unix socket for the current container's image info.
fn socket_container_info(id: &Option<String>) -> Option<HashMap<String, String>> {
    let socket = if Path::new("/var/run/docker.sock").exists() {
        "/var/run/docker.sock"
    } else if Path::new("/run/podman/podman.sock").exists() {
        "/run/podman/podman.sock"
    } else {
        return None;
    };

    let endpoint = match id {
        Some(id) => format!("/containers/{id}/json"),
        None => "/info".to_string(),
    };
    let body = unix_http_get(socket, &endpoint)?;
    let v: serde_json::Value = serde_json::from_str(&body).ok()?;

    let mut out = HashMap::new();
    if let Some(name) = v.get("Name").and_then(|n| n.as_str()) {
        out.insert("name".into(), name.trim_start_matches('/').to_string());
    }
    if let Some(image) = v
        .get("Config")
        .and_then(|c| c.get("Image"))
        .and_then(|i| i.as_str())
    {
        out.insert("image".into(), image.to_string());
    }
    if let Some(created) = v.get("Created").and_then(|c| c.as_str()) {
        out.insert("created".into(), created.chars().take(10).collect());
    }
    if let Some(mounts) = v.get("Mounts").and_then(|m| m.as_array()) {
        out.insert("mounts".into(), mounts.len().to_string());
    }
    Some(out)
}

/// Minimal HTTP/1.1 GET over a Unix domain socket (docker engine API).
#[cfg(unix)]
fn unix_http_get(socket: &str, path: &str) -> Option<String> {
    use std::os::unix::net::UnixStream;
    let mut stream = UnixStream::connect(socket).ok()?;
    let req = format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
    stream.write_all(req.as_bytes()).ok()?;
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).ok()?;
    let text = String::from_utf8_lossy(&buf);
    text.split("\r\n\r\n").nth(1).map(|s| s.to_string())
}

#[cfg(not(unix))]
fn unix_http_get(_socket: &str, _path: &str) -> Option<String> {
    None
}

fn read_first_line(path: &str) -> Option<String> {
    std::fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}
#[cfg(test)]
mod tests {
    use super::parse_container_id;

    #[test]
    fn detects_64_hex_container_id() {
        // Simulated cgroup with a docker id.
        let cg =
            "12:devices:/docker/0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        assert_eq!(parse_container_id(cg), Some("0123456789ab".into()));
    }

    #[test]
    fn rejects_non_hex_cgroup() {
        assert_eq!(parse_container_id("0::/system.slice/sshd.service"), None);
    }
}
