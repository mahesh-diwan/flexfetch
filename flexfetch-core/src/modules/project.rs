use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;
use std::path::Path;

pub struct ProjectModule;

/// Manifest files that identify a project, mapped to a friendly type label.
const MANIFESTS: &[(&str, &str)] = &[
    ("Cargo.toml", "Rust"),
    ("package.json", "Node.js"),
    ("go.mod", "Go"),
    ("pyproject.toml", "Python"),
    ("requirements.txt", "Python"),
    ("pom.xml", "Java"),
    ("composer.json", "PHP"),
    ("Gemfile", "Ruby"),
    ("mix.exs", "Elixir"),
    ("build.gradle", "Gradle"),
    ("CMakeLists.txt", "C/C++"),
    ("Makefile", "C/C++"),
    ("Dockerfile", "Docker"),
    ("docker-compose.yml", "Docker"),
];

/// Walk up from `start` looking for a project manifest. Returns
/// (type_label, project_name) where project_name is the enclosing directory
/// name when the manifest doesn't carry a name we can cheaply read.
fn detect_project(ctx: &Context, start: &Path) -> Option<(String, String)> {
    let mut dir = start.to_path_buf();
    loop {
        for (manifest, kind) in MANIFESTS {
            if ctx.exists(dir.join(manifest)) {
                let name = dir
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                return Some((kind.to_string(), name));
            }
        }
        if !dir.pop() {
            break;
        }
    }
    None
}

impl Module for ProjectModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        if let Ok(cwd) = std::env::current_dir() {
            if let Some((kind, name)) = detect_project(ctx, &cwd) {
                map.insert("type".into(), kind);
                map.insert("name".into(), name);
            }
        }

        Ok(InfoValue::Map(map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(tag: &str) -> std::path::PathBuf {
        let dir =
            std::env::temp_dir().join(format!("flexfetch-project-{tag}-{}", std::process::id()));
        // Remove any stale dir from a previous crashed run so detection is clean.
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn detects_cargo_project() {
        let dir = temp_dir("cargo");
        std::fs::write(dir.join("Cargo.toml"), "").unwrap();
        let ctx = crate::Context::new(
            std::env::temp_dir().join("ff-proj-cfg"),
            std::env::temp_dir().join("ff-proj-cache"),
            false,
            Default::default(),
        );
        let got = detect_project(&ctx, &dir);
        assert_eq!(
            got,
            Some((
                "Rust".to_string(),
                dir.file_name().unwrap().to_string_lossy().to_string()
            ))
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn detects_node_project() {
        let dir = temp_dir("node");
        std::fs::write(dir.join("package.json"), "{}").unwrap();
        let ctx = crate::Context::new(
            std::env::temp_dir().join("ff-proj-cfg"),
            std::env::temp_dir().join("ff-proj-cache"),
            false,
            Default::default(),
        );
        let got = detect_project(&ctx, &dir);
        assert_eq!(
            got,
            Some((
                "Node.js".to_string(),
                dir.file_name().unwrap().to_string_lossy().to_string()
            ))
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn empty_when_no_manifest() {
        let dir = temp_dir("empty");
        let ctx = crate::Context::new(
            std::env::temp_dir().join("ff-proj-cfg"),
            std::env::temp_dir().join("ff-proj-cache"),
            false,
            Default::default(),
        );
        let got = detect_project(&ctx, &dir);
        assert_eq!(got, None);
        std::fs::remove_dir_all(&dir).ok();
    }
}
