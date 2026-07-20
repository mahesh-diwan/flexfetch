use crate::{Module, InfoValue, Context, Result};

pub struct CustomCommandModule {
    pub name: &'static str,
    pub command: String,
    pub label: Option<String>,
    pub shell: Option<String>,
}

impl Module for CustomCommandModule {
    fn name(&self) -> &'static str { self.name }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let shell = self.shell.as_deref().unwrap_or("sh");
        let output = std::process::Command::new(shell)
            .arg("-c")
            .arg(&self.command)
            .output()
            .map_err(|e| crate::Error::Io(e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let label = self.label.clone().unwrap_or_else(|| self.name.to_string());

        if stdout.is_empty() {
            Ok(InfoValue::scalar(format!("{label}: ")))
        } else {
            Ok(InfoValue::scalar(format!("{label}: {stdout}")))
        }
    }
}
