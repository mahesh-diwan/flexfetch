use crate::{Context, InfoValue, Module, Result};

pub struct ProcessesModule;

impl Module for ProcessesModule {
    fn name(&self) -> &'static str {
        "processes"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let count = process_count();
        Ok(InfoValue::Scalar(count))
    }
}

fn process_count() -> String {
    #[cfg(target_os = "linux")]
    {
        if let Ok(entries) = std::fs::read_dir("/proc") {
            let count = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
                .filter(|e| {
                    e.file_name()
                        .to_str()
                        .map(|s| s.bytes().all(|b| b.is_ascii_digit()))
                        .unwrap_or(false)
                })
                .count();
            return count.to_string();
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = std::process::Command::new("ps").args(["-e"]).output() {
            let s = String::from_utf8_lossy(&output.stdout);
            let count = s.lines().count().saturating_sub(1); // header line
            return count.to_string();
        }
    }

    "unknown".to_string()
}
