//! Phase 8.6 — cold-start benchmark (criterion). Tracks the minimal-build
//! startup cost across history so regressions are caught before release.
//!
//! Run: `cargo bench -p flexfetch-cli --bench cold_start`
//! CI:  `.github/workflows/bench.yml` runs this and pushes results to Pages.

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_cold_start_minimal(c: &mut Criterion) {
    // Build once; benchmark the process spawn + collection of the minimal
    // module set (all zero-spawn file reads per Phase 4.1).
    let binary = env!("CARGO_BIN_EXE_flexfetch");
    c.bench_function("cold_start_minimal", |b| {
        b.iter(|| {
            let out = std::process::Command::new(binary)
                .arg("--minimal")
                .arg("--pipe")
                .output()
                .expect("flexfetch should run");
            black_box(out.stdout);
        })
    });
}

fn bench_cold_start_default(c: &mut Criterion) {
    let binary = env!("CARGO_BIN_EXE_flexfetch");
    c.bench_function("cold_start_default_pipe", |b| {
        b.iter(|| {
            let out = std::process::Command::new(binary)
                .arg("--pipe")
                .arg("--modules")
                .arg("os:kernel:uptime")
                .output()
                .expect("flexfetch should run");
            black_box(out.stdout);
        })
    });
}

criterion_group!(benches, bench_cold_start_minimal, bench_cold_start_default);
criterion_main!(benches);
