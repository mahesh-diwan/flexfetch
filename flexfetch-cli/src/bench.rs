use flexfetch_cli::Cli;
use flexfetch_core::{Config, Context, ModuleRegistry, TeraEngine};
use std::time::Instant;

/// Micro-benchmark (`--benchmark` / `--benchmark N`).
///
/// Reports, in order:
/// 1. the **real path** — one parallel `run_selected` + render, measured before
///    the sequential loop below populates the cache, so the headline equals
///    what a stopwatch on a normal `flexfetch` invocation shows;
/// 2. per-module timings (cold, sequential — informational, for finding slow
///    collectors);
/// 3. N full-pipeline warm iterations reporting min/avg.
pub fn run(
    modules: &[String],
    ctx: &std::sync::Arc<Context>,
    registry: &'static ModuleRegistry,
    template_content: &str,
    config: &Config,
    cli: &Cli,
    t_cold_start: Instant,
) {
    let iterations = cli.benchmark.unwrap_or(1).max(1);
    let t0 = Instant::now();

    // Cache state sampled BEFORE any collection: the real-path run and the
    // per-module loop below populate the cache, so checking afterwards would
    // always say "warm".
    let cached = ctx
        .cache
        .lock()
        .map(|c| c.get("wifi").is_some())
        .unwrap_or(false);

    // Real path: exactly what a normal invocation does — one parallel collect
    // plus one render — timed from process entry so `cold start` is honest.
    let t = Instant::now();
    let info = registry.run_selected(modules, ctx, template_content);
    let real_collect = t.elapsed();
    let engine = TeraEngine::new_default();
    let t = Instant::now();
    let _ = engine.render(&info, config);
    let real_render = t.elapsed();
    let real_total = t_cold_start.elapsed();

    // Per-module timing (cold, sequential — informational only).
    let mut timings = Vec::new();
    for name in modules {
        if name == "title" || name == "separator" {
            continue;
        }
        let t = Instant::now();
        let _ = registry.run_individual(name, ctx);
        timings.push((name.clone(), t.elapsed()));
    }
    timings.sort_by_key(|&(_, dur)| std::cmp::Reverse(dur));

    // Warm pipeline: run the full selected pipeline N times, keeping the last
    // `info` around so the single-iteration branch can render it directly
    // instead of running `run_selected` a second time.
    let mut run_selected_times = Vec::new();
    let mut render_times = Vec::new();
    let mut last_info = None;
    for _ in 0..iterations {
        let t = Instant::now();
        let info = registry.run_selected(modules, ctx, template_content);
        run_selected_times.push(t.elapsed());
        let engine = TeraEngine::new_default();
        let t = Instant::now();
        let _ = engine.render(&info, config);
        render_times.push(t.elapsed());
        last_info = Some(info);
    }

    eprintln!(
        "--- flexfetch benchmark ({iterations} iteration{}) ---",
        if iterations == 1 { "" } else { "s" }
    );
    eprintln!(
        "  cache:           {} (checked before any collection)",
        if cached { "warm" } else { "cold" }
    );
    eprintln!(
        "  real path:       collect {:?} + render {:?}",
        real_collect, real_render
    );
    eprintln!(
        "  cold start:      {:?} (process entry -> first render)",
        real_total
    );
    eprintln!("  per-module (cold, sequential, informational):");
    for (name, dur) in &timings {
        eprintln!("    {name:15} {dur:?}");
    }
    if iterations > 1 {
        let avg = |v: &[std::time::Duration]| -> std::time::Duration {
            let sum: std::time::Duration = v.iter().sum();
            sum / iterations as u32
        };
        let min = |v: &[std::time::Duration]| -> std::time::Duration {
            *v.iter().min().unwrap_or(&std::time::Duration::ZERO)
        };
        eprintln!(
            "  run_selected:    avg {:?} (min {:?})",
            avg(&run_selected_times),
            min(&run_selected_times)
        );
        eprintln!(
            "  template render: avg {:?} (min {:?})",
            avg(&render_times),
            min(&render_times)
        );
        eprintln!("  total:           {:?}", t0.elapsed());
    } else {
        eprintln!("  run_selected:    {:?}", run_selected_times[0]);
        eprintln!("  template render: {:?}", render_times[0]);
        eprintln!("  total:           {:?}", t0.elapsed());
    }
    eprintln!("---");

    if let Some(ref format) = cli.export {
        let info =
            last_info.unwrap_or_else(|| registry.run_selected(modules, ctx, template_content));
        crate::render_output::export(&info, config, format, cli.output.as_deref());
        return;
    }
    if cli.format == "json" {
        let info =
            last_info.unwrap_or_else(|| registry.run_selected(modules, ctx, template_content));
        println!(
            "{}",
            serde_json::to_string_pretty(&info.to_json()).unwrap_or_else(|_| "{}".into())
        );
    }
}
