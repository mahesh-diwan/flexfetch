use crate::{Context, InfoValue, Module, Result};

/// Chassis type, decoded from the DMI numeric code (SMBIOS 3.x table 7).
pub struct ChassisModule;

impl Module for ChassisModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        #[cfg(target_os = "linux")]
        {
            let code = ctx
                .read_file("/sys/class/dmi/id/chassis_type")
                .ok()
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
            if code.is_empty() {
                return Ok(InfoValue::Scalar("unknown".into()));
            }
            Ok(InfoValue::Scalar(chassis_name(&code)))
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = ctx;
            Ok(InfoValue::Scalar("unknown".into()))
        }
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use crate::fs::{test_ctx, MockFs};

    #[test]
    fn decodes_common_types() {
        assert_eq!(chassis_name("2"), "Notebook");
        assert_eq!(chassis_name("9"), "Laptop");
        assert_eq!(chassis_name("7"), "Tower");
        assert_eq!(chassis_name("23"), "Rack Mount");
    }

    #[test]
    fn unknown_code_and_missing_file() {
        assert_eq!(chassis_name("999"), "Unknown");
        assert_eq!(chassis_name("garbage"), "Unknown");
        let ctx = test_ctx(MockFs::new());
        let v = ChassisModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), "unknown");
    }
}

fn chassis_name(code: &str) -> String {
    let code: u16 = code.parse().unwrap_or(0);
    let name = match code {
        1 | 4 => "Desktop",
        2 => "Notebook",
        3 => "Desktop", // desktop on the side — keep it friendly
        5 => "Pizza Box",
        6 => "Mini Tower",
        7 => "Tower",
        8 => "Portable",
        9 => "Laptop",
        10 => "Notebook",
        11 => "Handheld",
        12 => "Docking Station",
        13 => "All-in-One",
        14 => "Sub-Notebook",
        15 => "Space-Saving",
        16 => "Lunch Box",
        17 => "Main Server",
        18 => "Expansion",
        19 => "Sub-Chassis",
        20 => "Bus Expansion",
        21 => "Peripheral",
        22 => "RAID",
        23 => "Rack Mount",
        24 => "Sealed-case PC",
        25 => "Multi-system",
        26 => "Compact PCI",
        27 => "Advanced TCA",
        28 => "Blade",
        29 => "Blade Enclosure",
        30 => "Tablet",
        31 => "Convertible",
        32 => "Detachable",
        33 => "IoT Gateway",
        34 => "Embedded PC",
        35 => "Mini PC",
        36 => "Stick PC",
        _ => "Unknown",
    };
    name.to_string()
}
