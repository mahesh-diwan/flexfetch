use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;
use std::process::Command;

pub struct GitModule;

fn run_git(args: &[&str]) -> Option<String> {
    let out = Command::new("git").args(args).output().ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
    } else {
        None
    }
}

impl Module for GitModule {
    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        // Not a git repo (or git not installed) -> return empty so the template
        // omits the line entirely rather than printing an error.
        let Some(branch) = run_git(&["rev-parse", "--abbrev-ref", "HEAD"]) else {
            return Ok(InfoValue::Map(map));
        };
        if branch.is_empty() || branch == "HEAD" {
            return Ok(InfoValue::Map(map));
        }
        map.insert("branch".into(), branch);

        // Ahead/behind vs upstream (skip when no upstream is configured)
        if let Some(ahead) = run_git(&["rev-list", "--count", "@{upstream}..HEAD"]) {
            if !ahead.is_empty() && ahead != "0" {
                map.insert("ahead".into(), ahead);
            }
        }
        if let Some(behind) = run_git(&["rev-list", "--count", "HEAD..@{upstream}"]) {
            if !behind.is_empty() && behind != "0" {
                map.insert("behind".into(), behind);
            }
        }

        // Dirty file count (porcelain = one line per changed file, uncommitted)
        if let Some(status) = run_git(&["status", "--porcelain"]) {
            let dirty = status.lines().filter(|l| !l.is_empty()).count();
            if dirty > 0 {
                map.insert("dirty".into(), dirty.to_string());
            }
        }

        Ok(InfoValue::Map(map))
    }
}

// Restores the original working directory when dropped, so a failing assertion
// inside a test that chdir'd can't leak a changed cwd into parallel tests.
#[cfg(test)]
struct CwdGuard(Option<std::path::PathBuf>);

#[cfg(test)]
impl CwdGuard {
    fn switch_to(dir: &std::path::Path) -> Self {
        let prev = std::env::current_dir().ok();
        std::env::set_current_dir(dir).expect("chdir to temp test dir");
        CwdGuard(prev)
    }
}

#[cfg(test)]
impl Drop for CwdGuard {
    fn drop(&mut self) {
        if let Some(p) = self.0.take() {
            std::env::set_current_dir(p).ok();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_map_when_not_a_repo() {
        // Run in a temp dir that is not a git repository.
        let dir = std::env::temp_dir().join(format!("flexfetch-git-test-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).unwrap();
        let _guard = CwdGuard::switch_to(&dir);

        let module = GitModule;
        let ctx = crate::Context::new(
            std::path::PathBuf::from("/tmp/flexfetch-test-config"),
            std::path::PathBuf::from("/tmp/flexfetch-test-cache"),
            false,
            HashMap::new(),
        );
        let value = module.collect(&ctx).unwrap();
        let empty = match value {
            InfoValue::Map(m) => m.is_empty(),
            _ => false,
        };
        assert!(empty, "non-repo cwd should produce an empty map");

        drop(_guard);
        std::fs::remove_dir_all(&dir).ok();
    }
}
