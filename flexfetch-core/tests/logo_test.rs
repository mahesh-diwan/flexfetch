use crate::logo::{
    detect, logo_width, render, visible_len, Logo, ALPINE_LOGO, ARCH_LOGO, BATTERY_LOGO,
    CENTOS_LOGO, COLORS_LOGO, CPU_LOGO, CUSTOM_LOGO, DEBIAN_LOGO, DE_LOGO, DISK_LOGO,
    ENDEAVOUROS_LOGO, FEDORA_LOGO, GENERIC_LOGO, GENTOO_LOGO, GPU_LOGO, HOST_LOGO, KALI_LOGO,
    KERNEL_LOGO, LOCALE_LOGO, MACOS_LOGO, MANJARO_LOGO, MEMORY_LOGO, NETWORK_LOGO, NIXOS_LOGO,
    OPENSUSE_LOGO, OS_LOGO, PACKAGES_LOGO, PROCESSES_LOGO, RESOLUTION_LOGO, SHELL_LOGO,
    TERMINAL_LOGO, TITLE_LOGO, UBUNTU_LOGO, UPTIME_LOGO, VOID_LOGO, WM_LOGO,
};

#[test]
fn test_visible_len_with_normal_string() {
    assert_eq!(visible_len("hello"), 5);
}

#[test]
fn test_visible_len_with_escape_sequences() {
    assert_eq!(visible_len("hello\\x1b[31m"), 5);
}

#[test]
fn test_visible_len_empty_string() {
    assert_eq!(visible_len(""), 0);
}

#[test]
fn test_logo_width_single_line() {
    let logo = Logo {
        lines: &["hello".to_string(), "world".to_string()],
        colors: &["red".to_string()],
    };
    let rendered = render(&logo, 2);
    assert_eq!(logo_width(&rendered), 5);
}

#[test]
fn test_logo_detect_title() {
    assert!(detect("title") == &TITLE_LOGO);
}

#[test]
fn test_logo_detect_cpu() {
    assert!(detect("cpu") == &CPU_LOGO);
}

#[test]
fn test_logo_detect_unknown() {
    assert!(detect("unknown_module") == &GENERIC_LOGO);
}
