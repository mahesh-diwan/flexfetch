use crate::{Context, InfoValue, Module, Result};
use std::path::Path;

/// Default editor — `$VISUAL` (preferred) then `$EDITOR`, basename only.
pub struct EditorModule;

impl Module for EditorModule {
    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let editor = std::env::var("VISUAL")
            .ok()
            .or_else(|| std::env::var("EDITOR").ok())
            .filter(|s| !s.trim().is_empty())
            .map(|s| {
                // `code --wait` style invocations → keep the program name only.
                Path::new(s.split_whitespace().next().unwrap_or(&s))
                    .file_name()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap_or(s)
            })
            .unwrap_or_else(|| "unknown".into());

        Ok(InfoValue::Scalar(editor))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::test_ctx;
    use crate::fs::MockFs;

    // Env vars are process-global; these tests must not run interleaved.
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn prefers_visual_over_editor() {
        let _g = ENV_LOCK.lock().unwrap();
        std::env::set_var("VISUAL", "vim");
        std::env::set_var("EDITOR", "nano");
        let ctx = test_ctx(MockFs::new());
        let v = EditorModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), "vim");
    }

    #[test]
    fn falls_back_to_editor() {
        let _g = ENV_LOCK.lock().unwrap();
        std::env::remove_var("VISUAL");
        std::env::set_var("EDITOR", "/usr/bin/nvim");
        let ctx = test_ctx(MockFs::new());
        let v = EditorModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), "nvim");
    }

    #[test]
    fn strips_invocation_flags() {
        let _g = ENV_LOCK.lock().unwrap();
        std::env::remove_var("VISUAL");
        std::env::set_var("EDITOR", "code --wait");
        let ctx = test_ctx(MockFs::new());
        let v = EditorModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), "code");
    }

    #[test]
    fn unknown_when_unset() {
        let _g = ENV_LOCK.lock().unwrap();
        std::env::remove_var("VISUAL");
        std::env::remove_var("EDITOR");
        let ctx = test_ctx(MockFs::new());
        let v = EditorModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), "unknown");
    }
}
