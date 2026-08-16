use crate::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_round_trip_preserves_entries() {
        // The `--ssh` remote-fetch path depends entirely on
        // to_json -> from_json round-tripping all InfoValue shapes.
        let mut info = SystemInfo::new();
        info.add("scalar", InfoValue::Scalar("value".into()));
        let mut map = HashMap::new();
        map.insert("key".into(), "val".into());
        info.add("map", InfoValue::Map(map));
        info.add("list", InfoValue::List(vec!["a".into(), "b".into()]));
        let mut row = HashMap::new();
        row.insert("col".into(), "cell".into());
        info.add("table", InfoValue::Table(vec![row]));

        let json = info.to_json();
        let back = SystemInfo::from_json(&json).expect("from_json should succeed");
        assert_eq!(back.to_json(), json, "round trip must be lossless");
        assert_eq!(back.entries.len(), 4);
    }

    #[test]
    fn from_json_rejects_non_object() {
        assert!(SystemInfo::from_json(&serde_json::json!([])).is_err());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InfoValue {
    Scalar(String),
    Map(HashMap<String, String>),
    List(Vec<String>),
    Table(Vec<HashMap<String, String>>),
}

impl InfoValue {
    pub fn scalar(s: impl Into<String>) -> Self {
        InfoValue::Scalar(s.into())
    }

    pub fn is_empty(&self) -> bool {
        match self {
            InfoValue::Scalar(s) => s.is_empty(),
            InfoValue::Map(m) => m.is_empty(),
            InfoValue::List(l) => l.is_empty(),
            InfoValue::Table(t) => t.is_empty(),
        }
    }

    pub fn summary(&self) -> String {
        match self {
            InfoValue::Scalar(s) => s.clone(),
            InfoValue::Map(m) => {
                let mut parts: Vec<String> =
                    m.iter().map(|(k, val)| format!("{k}={val}")).collect();
                parts.sort();
                parts.join(", ")
            }
            InfoValue::List(l) => l.join(", "),
            InfoValue::Table(t) => format!("{} rows", t.len()),
        }
    }
}

pub trait Module: Send + Sync {
    fn collect(&self, ctx: &Context) -> crate::Result<InfoValue>;
}

/// Single source of truth for all built-in modules. Every consumer (registry,
/// list_modules, template placeholders, section grouping, label mapping)
/// derives from this catalog — add a module here and it propagates everywhere.
pub struct ModuleEntry {
    pub name: &'static str,
    pub section: Option<&'static str>,
    pub is_static: bool,
    pub label: &'static str,
    pub builder: fn() -> Box<dyn Module>,
}

pub const MODULE_CATALOG: &[ModuleEntry] = &[
    // System
    ModuleEntry {
        name: "os",
        section: Some("System"),
        is_static: true,
        label: "OS",
        builder: || Box::new(crate::modules::os::OsModule),
    },
    ModuleEntry {
        name: "host",
        section: Some("System"),
        is_static: true,
        label: "Host",
        builder: || Box::new(crate::modules::host::HostModule),
    },
    ModuleEntry {
        name: "kernel",
        section: Some("System"),
        is_static: true,
        label: "Kernel",
        builder: || Box::new(crate::modules::kernel::KernelModule),
    },
    ModuleEntry {
        name: "uptime",
        section: Some("System"),
        is_static: false,
        label: "Uptime",
        builder: || Box::new(crate::modules::uptime::UptimeModule),
    },
    ModuleEntry {
        name: "locale",
        section: Some("System"),
        is_static: true,
        label: "Locale",
        builder: || Box::new(crate::modules::locale::LocaleModule),
    },
    ModuleEntry {
        name: "datetime",
        section: Some("System"),
        is_static: false,
        label: "Date/Time",
        builder: || Box::new(crate::modules::datetime::DatetimeModule),
    },
    ModuleEntry {
        name: "loadavg",
        section: Some("System"),
        is_static: false,
        label: "Load Avg",
        builder: || Box::new(crate::modules::loadavg::LoadavgModule),
    },
    ModuleEntry {
        name: "keyboard",
        section: Some("System"),
        is_static: true,
        label: "Keyboard",
        builder: || Box::new(crate::modules::keyboard::KeyboardModule),
    },
    // Software
    ModuleEntry {
        name: "packages",
        section: Some("Software"),
        is_static: true,
        label: "Packages",
        builder: || Box::new(crate::modules::packages::PackagesModule),
    },
    ModuleEntry {
        name: "shell",
        section: Some("Software"),
        is_static: true,
        label: "Shell",
        builder: || Box::new(crate::modules::shell::ShellModule),
    },
    ModuleEntry {
        name: "editor",
        section: Some("Software"),
        is_static: true,
        label: "Editor",
        builder: || Box::new(crate::modules::editor::EditorModule),
    },
    ModuleEntry {
        name: "initsystem",
        section: Some("Software"),
        is_static: true,
        label: "Init",
        builder: || Box::new(crate::modules::initsystem::InitsystemModule),
    },
    ModuleEntry {
        name: "version",
        section: Some("Software"),
        is_static: true,
        label: "Version",
        builder: || Box::new(crate::modules::version::VersionModule),
    },
    ModuleEntry {
        name: "terminal",
        section: Some("Software"),
        is_static: true,
        label: "Terminal",
        builder: || Box::new(crate::modules::terminal::TerminalModule),
    },
    ModuleEntry {
        name: "de",
        section: Some("Software"),
        is_static: true,
        label: "DE",
        builder: || Box::new(crate::modules::de::DeModule),
    },
    ModuleEntry {
        name: "wm",
        section: Some("Software"),
        is_static: true,
        label: "WM",
        builder: || Box::new(crate::modules::wm::WmModule),
    },
    ModuleEntry {
        name: "project",
        section: Some("Software"),
        is_static: true,
        label: "Project",
        builder: || Box::new(crate::modules::project::ProjectModule),
    },
    ModuleEntry {
        name: "git",
        section: Some("Software"),
        is_static: true,
        label: "Git",
        builder: || Box::new(crate::modules::git::GitModule),
    },
    ModuleEntry {
        name: "context",
        section: Some("Software"),
        is_static: true,
        label: "Context",
        builder: || Box::new(crate::modules::context::ContextModule),
    },
    ModuleEntry {
        name: "health",
        section: Some("Software"),
        is_static: true,
        label: "Health",
        builder: || Box::new(crate::modules::health::HealthModule),
    },
    ModuleEntry {
        name: "container",
        section: Some("Software"),
        is_static: true,
        label: "Container",
        builder: || Box::new(crate::modules::container::ContainerModule),
    },
    ModuleEntry {
        name: "wallpaper",
        section: Some("Software"),
        is_static: true,
        label: "Wallpaper",
        builder: || Box::new(crate::modules::wallpaper::WallpaperModule),
    },
    ModuleEntry {
        name: "weather",
        section: Some("Software"),
        is_static: true,
        label: "Weather",
        builder: || Box::new(crate::modules::weather::WeatherModule),
    },
    ModuleEntry {
        name: "fsdeep",
        section: Some("Software"),
        is_static: true,
        label: "Fsdeep",
        builder: || Box::new(crate::modules::fsdeep::FsDeepModule),
    },
    // Hardware
    ModuleEntry {
        name: "bios",
        section: Some("Hardware"),
        is_static: true,
        label: "BIOS",
        builder: || Box::new(crate::modules::bios::BiosModule),
    },
    ModuleEntry {
        name: "board",
        section: Some("Hardware"),
        is_static: true,
        label: "Board",
        builder: || Box::new(crate::modules::board::BoardModule),
    },
    ModuleEntry {
        name: "chassis",
        section: Some("Hardware"),
        is_static: true,
        label: "Chassis",
        builder: || Box::new(crate::modules::chassis::ChassisModule),
    },
    ModuleEntry {
        name: "brightness",
        section: Some("Hardware"),
        is_static: false,
        label: "Brightness",
        builder: || Box::new(crate::modules::brightness::BrightnessModule),
    },
    ModuleEntry {
        name: "tpm",
        section: Some("Hardware"),
        is_static: true,
        label: "TPM",
        builder: || Box::new(crate::modules::tpm::TpmModule),
    },
    ModuleEntry {
        name: "cpu",
        section: Some("Hardware"),
        is_static: true,
        label: "CPU",
        builder: || Box::new(crate::modules::cpu::CpuModule),
    },
    ModuleEntry {
        name: "cpucache",
        section: Some("Hardware"),
        is_static: true,
        label: "Cache",
        builder: || Box::new(crate::modules::cpucache::CpuCacheModule),
    },
    ModuleEntry {
        name: "cpuusage",
        section: Some("Hardware"),
        is_static: false,
        label: "CPU Usage",
        builder: || Box::new(crate::modules::cpuusage::CpuUsageModule),
    },
    ModuleEntry {
        name: "gpu",
        section: Some("Hardware"),
        is_static: true,
        label: "GPU",
        builder: || Box::new(crate::modules::gpu::GpuModule),
    },
    ModuleEntry {
        name: "memory",
        section: Some("Hardware"),
        is_static: false,
        label: "Memory",
        builder: || Box::new(crate::modules::memory::MemoryModule),
    },
    ModuleEntry {
        name: "swap",
        section: Some("Hardware"),
        is_static: false,
        label: "Swap",
        builder: || Box::new(crate::modules::swap::SwapModule),
    },
    ModuleEntry {
        name: "disk",
        section: Some("Hardware"),
        is_static: false,
        label: "Disk",
        builder: || Box::new(crate::modules::disk::DiskModule),
    },
    ModuleEntry {
        name: "battery",
        section: Some("Hardware"),
        is_static: false,
        label: "Battery",
        builder: || Box::new(crate::modules::battery::BatteryModule),
    },
    ModuleEntry {
        name: "temperature",
        section: Some("Hardware"),
        is_static: true,
        label: "Temp",
        builder: || Box::new(crate::modules::temperature::TemperatureModule),
    },
    ModuleEntry {
        name: "display",
        section: Some("Hardware"),
        is_static: true,
        label: "Display",
        builder: || Box::new(crate::modules::display::DisplayModule),
    },
    ModuleEntry {
        name: "resolution",
        section: Some("Hardware"),
        is_static: true,
        label: "Resolution",
        builder: || Box::new(crate::modules::resolution::ResolutionModule),
    },
    ModuleEntry {
        name: "colors",
        section: Some("Hardware"),
        is_static: true,
        label: "Colors",
        builder: || Box::new(crate::modules::colors::ColorsModule),
    },
    // Network
    ModuleEntry {
        name: "network",
        section: Some("Network"),
        is_static: false,
        label: "Network",
        builder: || Box::new(crate::modules::network::NetworkModule),
    },
    ModuleEntry {
        name: "wifi",
        section: Some("Network"),
        is_static: true,
        label: "WiFi",
        builder: || Box::new(crate::modules::wifi::WifiModule),
    },
    ModuleEntry {
        name: "localip",
        section: Some("Network"),
        is_static: true,
        label: "Local IP",
        builder: || Box::new(crate::modules::localip::LocalipModule),
    },
    ModuleEntry {
        name: "publicip",
        section: Some("Network"),
        is_static: true,
        label: "Public IP",
        builder: || Box::new(crate::modules::publicip::PublicIpModule),
    },
    ModuleEntry {
        name: "bluetooth",
        section: Some("Network"),
        is_static: true,
        label: "Bluetooth",
        builder: || Box::new(crate::modules::bluetooth::BluetoothModule),
    },
    ModuleEntry {
        name: "media",
        section: Some("Network"),
        is_static: false,
        label: "Media",
        builder: || Box::new(crate::modules::media::MediaModule),
    },
    ModuleEntry {
        name: "dns",
        section: Some("Network"),
        is_static: true,
        label: "DNS",
        builder: || Box::new(crate::modules::dns::DnsModule),
    },
    // Processes
    ModuleEntry {
        name: "processes",
        section: Some("Processes"),
        is_static: false,
        label: "Processes",
        builder: || Box::new(crate::modules::processes::ProcessesModule),
    },
    // Layout-only (no section, not collected as a module)
    ModuleEntry {
        name: "title",
        section: None,
        is_static: true,
        label: "Title",
        builder: || Box::new(crate::modules::title::TitleModule),
    },
    // Custom commands (always dynamic)
    ModuleEntry {
        name: "custom",
        section: Some("Software"),
        is_static: false,
        label: "Custom",
        builder: || Box::new(crate::modules::custom::CustomCommandsModule),
    },
];

/// Look up a module entry by name.
pub fn find_module(name: &str) -> Option<&'static ModuleEntry> {
    MODULE_CATALOG.iter().find(|m| m.name == name)
}

pub struct SystemInfo {
    pub entries: Vec<(&'static str, InfoValue)>,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemInfo {
    pub fn new() -> Self {
        SystemInfo {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, name: &'static str, value: InfoValue) {
        self.entries.push((name, value));
    }

    pub fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        for (name, value) in &self.entries {
            map.insert(
                name.to_string(),
                serde_json::to_value(value).unwrap_or_default(),
            );
        }
        serde_json::Value::Object(map)
    }

    /// Rebuild a `SystemInfo` from the JSON produced by `to_json` (e.g. parsed
    /// from a remote `flexfetch --format json` run over SSH).
    pub fn from_json(value: &serde_json::Value) -> crate::Result<Self> {
        let obj = value
            .as_object()
            .ok_or_else(|| crate::Error::Template("remote output is not a JSON object".into()))?;
        let mut info = SystemInfo::new();
        for (name, val) in obj {
            let parsed = serde_json::from_value::<InfoValue>(val.clone()).map_err(|e| {
                crate::Error::Template(format!("parse remote value for '{name}': {e}"))
            })?;
            // Box the name to a leaked 'static string: the registry keys are
            // 'static but remote module names are dynamic. A few leaked strings
            // per fetch is negligible for a CLI process.
            let leaked: &'static str = Box::leak(name.clone().into_boxed_str());
            info.add(leaked, parsed);
        }
        Ok(info)
    }
}
