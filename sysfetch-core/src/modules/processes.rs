use crate::{Module, InfoValue, Context, Result};

pub struct ProcessesModule;

impl Module for ProcessesModule {
    fn name(&self) -> &'static str { "processes" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let count = get_process_count().unwrap_or(0);
        Ok(InfoValue::Scalar(count.to_string()))
    }
}

fn get_process_count() -> Option<usize> {
    #[cfg(target_os = "linux")]
    {
        let entries = std::fs::read_dir("/proc").ok()?;
        let mut count = 0usize;
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.chars().all(|c| c.is_ascii_digit()) {
                count += 1;
            }
        }
        Some(count)
    }
    #[cfg(target_os = "macos")]
    {
        let out = std::process::Command::new("ps")
            .args(["-e", "--no-headers"])
            .output().ok()?;
        let count = String::from_utf8_lossy(&out.stdout).lines().count();
        Some(count)
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    { None }
}
