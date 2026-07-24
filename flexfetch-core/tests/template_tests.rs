use flexfetch_core::template::{frame_wrap, get_box_chars};

#[test]
fn test_frame_wrap_double() {
    let text = "Hello\nWorld";
    let result = frame_wrap(text, "double", "\x1b[34m");
    assert!(result.contains("╔"));
    assert!(result.contains("╗"));
    assert!(result.contains("╚"));
    assert!(result.contains("╝"));
    assert!(result.contains("Hello"));
    assert!(result.contains("World"));
}

#[test]
fn test_frame_wrap_single() {
    let text = "Test";
    let result = frame_wrap(text, "single", "");
    assert!(result.contains("┌"));
    assert!(result.contains("┐"));
    assert!(result.contains("└"));
    assert!(result.contains("┘"));
}

#[test]
fn test_frame_wrap_none() {
    let text = "Unchanged";
    let result = frame_wrap(text, "none", "");
    assert_eq!(result, text);
}

#[test]
fn test_get_box_chars_rounded() {
    let chars = get_box_chars("rounded");
    assert_eq!(chars.header_left, "╭─ ");
    assert_eq!(chars.row, "│");
}

#[test]
fn test_get_box_chars_double() {
    let chars = get_box_chars("double");
    assert_eq!(chars.header_left, "╔═ ");
    assert_eq!(chars.row, "║");
}

#[test]
fn test_get_box_chars_ascii() {
    let chars = get_box_chars("ascii");
    assert_eq!(chars.header_left, "+- ");
    assert_eq!(chars.row, "|");
}

#[test]
fn test_get_box_chars_unknown_defaults() {
    let chars = get_box_chars("unknown");
    assert_eq!(chars.header_left, "╭─ ");
}
