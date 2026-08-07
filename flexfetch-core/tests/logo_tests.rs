use flexfetch_core::logo::{detect, visible_len};

#[test]
fn test_detect_arch() {
    let logo = detect("arch");
    assert!(!logo.lines.is_empty());
}

#[test]
fn test_detect_ubuntu() {
    let logo = detect("ubuntu");
    assert!(!logo.lines.is_empty());
}

#[test]
fn test_detect_unknown() {
    let logo = detect("unknown_distro");
    assert!(!logo.lines.is_empty());
}

#[test]
fn test_visible_len_plain() {
    assert_eq!(visible_len("hello"), 5);
}

#[test]
fn test_visible_len_ansi() {
    let colored = "\x1b[31mred\x1b[0m";
    assert_eq!(visible_len(colored), 3);
}

#[test]
fn test_visible_len_empty() {
    assert_eq!(visible_len(""), 0);
}
