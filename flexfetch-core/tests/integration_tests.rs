use flexfetch_core::{Config, Context, ModuleRegistry, TeraEngine};
use std::collections::HashMap;
use std::path::PathBuf;

fn create_test_context() -> Context {
    Context::new(
        PathBuf::from("/tmp/flexfetch-test-config"),
        PathBuf::from("/tmp/flexfetch-test-cache"),
        false,
        HashMap::new(),
    )
}

#[test]
fn test_config_default() {
    let config = Config::default_for_testing();
    assert!(config.display.gradient_title);
    assert!(config.display.progress_bars);
    assert_eq!(config.display.box_style, "rounded");
    assert_eq!(config.display.palette_style, "blocks");
    assert_eq!(config.display.frame, "none");
}

#[test]
fn test_module_registry_exists() {
    let registry = ModuleRegistry::get();
    let modules = vec![
        "os", "cpu", "memory", "colors", "disk", "network", "git", "project", "context",
    ];
    for module in modules {
        let result = registry.run_individual(module, &create_test_context());
        assert!(result.is_some(), "Module {} should return a value", module);
    }
}

#[test]
fn test_template_render() {
    let engine = TeraEngine::new_default();
    let config = Config::default_for_testing();
    let ctx = create_test_context();
    let registry = ModuleRegistry::get();
    let modules = Config::default_modules();
    let template_content = TeraEngine::default_template_content();
    let info = registry.run_selected(&modules, &std::sync::Arc::new(ctx), template_content);
    let result = engine.render(&info, &config);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("OS"));
    assert!(output.contains("CPU"));
    assert!(output.contains("Memory"));
    // Phase 6 visual overhaul: no tree connectors, keys are padded to align
    // values (default template uses `Key<sep>value` rows, not `├─ Key: value`).
    assert!(
        !output.contains("├─"),
        "tree connector must be gone: {output}"
    );
    assert!(
        !output.contains("╰─"),
        "end connector must be gone: {output}"
    );
}

#[test]
fn test_template_dedup_de_wm_and_display_resolution() {
    use flexfetch_core::InfoValue;
    use std::collections::HashMap;

    let engine = TeraEngine::new_default();
    let config = Config::default_for_testing();
    let mut info = flexfetch_core::SystemInfo::new();
    let mut os_map = HashMap::new();
    os_map.insert("pretty_name".to_string(), "CachyOS".to_string());
    info.add("os", InfoValue::Map(os_map));
    info.add("de", InfoValue::Scalar("Hyprland".into()));
    let mut wm_map = HashMap::new();
    wm_map.insert("name".to_string(), "Hyprland".to_string());
    info.add("wm", InfoValue::Map(wm_map));
    info.add("display", InfoValue::Scalar("1920x1080 @ 60.00".into()));
    info.add("resolution", InfoValue::Scalar("1920x1080".into()));

    let output = engine.render(&info, &config).unwrap();
    // DE == WM -> WM row is dropped
    assert!(output.contains("DE"), "DE row missing: {output}");
    assert!(
        !output.contains("WM:") && !output.contains("WM "),
        "WM row should be deduped when equal to DE: {output}"
    );
    // Display already contains the resolution -> Resolution row is dropped
    assert!(output.contains("Display"), "Display row missing: {output}");
    assert!(
        !output.contains("Resolution"),
        "Resolution row should be deduped when Display already reports it: {output}"
    );
}

#[test]
fn test_template_with_box_style_double() {
    let engine = TeraEngine::new_default();
    let mut config = Config::default_for_testing();
    config.display.box_style = "double".into();
    let ctx = create_test_context();
    let registry = ModuleRegistry::get();
    let modules = Config::default_modules();
    let template_content = TeraEngine::default_template_content();
    let info = registry.run_selected(&modules, &std::sync::Arc::new(ctx), template_content);
    let result = engine.render(&info, &config);
    assert!(result.is_ok());
    let output = result.unwrap();
    // Phase 6: box_style no longer injects tree connectors into default.tera
    // rows — the template owns its own aligned row format.
    assert!(!output.contains("├─"), "{output}");
    assert!(!output.contains("╰─"), "{output}");
}

#[test]
fn test_template_with_box_style_ascii() {
    let engine = TeraEngine::new_default();
    let mut config = Config::default_for_testing();
    config.display.box_style = "ascii".into();
    let ctx = create_test_context();
    let registry = ModuleRegistry::get();
    let modules = Config::default_modules();
    let template_content = TeraEngine::default_template_content();
    let info = registry.run_selected(&modules, &std::sync::Arc::new(ctx), template_content);
    let result = engine.render(&info, &config);
    assert!(result.is_ok());
    let output = result.unwrap();
    // Phase 6: box_style no longer injects tree connectors into default.tera
    // rows — the template owns its own aligned row format.
    assert!(!output.contains("├─"), "{output}");
    assert!(!output.contains("╰─"), "{output}");
}
