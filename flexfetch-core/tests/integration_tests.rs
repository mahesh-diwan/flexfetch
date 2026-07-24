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
    assert!(!config.display.pixel_logo);
    assert_eq!(config.display.palette_style, "blocks");
    assert_eq!(config.display.frame, "none");
}

#[test]
fn test_module_registry_exists() {
    let registry = ModuleRegistry::get();
    let modules = vec!["os", "cpu", "memory", "colors", "disk", "network"];
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
    let info = registry.run_selected(&modules, &ctx, template_content);
    let result = engine.render(&info, &config);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("OS"));
    assert!(output.contains("CPU"));
    assert!(output.contains("Memory"));
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
    let info = registry.run_selected(&modules, &ctx, template_content);
    let result = engine.render(&info, &config);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("╔"));
    assert!(output.contains("║"));
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
    let info = registry.run_selected(&modules, &ctx, template_content);
    let result = engine.render(&info, &config);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("+-"));
    assert!(output.contains("|"));
}
