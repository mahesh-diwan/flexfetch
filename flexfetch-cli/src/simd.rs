//! Phase 4.3 — SIMD micro-benchmarks.
//!
//! Runtime CPU-feature detection with scalar fallback, so this works on any
//! x86_64/ARM machine. `--bench-cpu` runs a vectorized integer benchmark and
//! reports which SIMD path was used; `--bench-memory` measures write bandwidth
//! on a 64 MiB buffer via `volatile` stores (no libc needed).
//!
//! NOTE: the `#[target_feature]` functions below are strictly cfg-gated to
//! their owning architecture — `#[target_feature(enable = "avx2")]` is an
//! E0635 compile error on aarch64, so the x86 intrinsics functions must never
//! exist on non-x86 targets (the macOS aarch64 CI build would otherwise fail).

use std::hint::black_box;
use std::time::{Duration, Instant};

/// CPU features detected at runtime (best available).
///
/// `allow(dead_code)`: which variants are ever *constructed* is purely a
/// function of the build architecture (Avx2/Sse4 only on x86_64, Neon only on
/// aarch64, Scalar elsewhere), so every single-arch build trips this lint on
/// the variants it can't construct. The variants are all still matched in
/// `bench_cpu`, so this is a cross-arch false positive, not real dead code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SimdLevel {
    Avx2,
    Sse4,
    Neon,
    Scalar,
}

pub fn detect() -> SimdLevel {
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            SimdLevel::Avx2
        } else if std::is_x86_feature_detected!("sse4.2") {
            SimdLevel::Sse4
        } else {
            SimdLevel::Scalar
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        // NEON is mandatory on aarch64.
        SimdLevel::Neon
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        SimdLevel::Scalar
    }
}

/// Run the integer benchmark N times; returns the best duration per iteration.
pub fn bench_cpu(iterations: u64) -> (SimdLevel, Duration) {
    let level = detect();
    let mut best = Duration::MAX;
    for _ in 0..iterations {
        let t = Instant::now();
        // `let _ =` is required: a bare `match` in statement position forces
        // arms of type `()`, which would mis-infer black_box::<()> and fail
        // with E0308. Binding the value keeps the benchmark result used.
        let _ = match level {
            SimdLevel::Avx2 => unsafe { black_box(simd_bench_avx2()) },
            SimdLevel::Sse4 => unsafe { black_box(simd_bench_sse4()) },
            SimdLevel::Neon => unsafe { black_box(simd_bench_neon()) },
            SimdLevel::Scalar => black_box(simd_bench_scalar()),
        };
        best = best.min(t.elapsed());
    }
    (level, best)
}

/// Memory-write bandwidth over a 64 MiB buffer (best of N runs).
pub fn bench_memory(iterations: u64) -> (f64, Duration) {
    const SIZE: usize = 64 * 1024 * 1024;
    let mut buf = vec![0u8; SIZE];
    let mut best = Duration::MAX;
    for _ in 0..iterations {
        let t = Instant::now();
        for slot in buf.iter_mut() {
            // volatile write defeats the optimizer without extra deps.
            unsafe { std::ptr::write_volatile(slot, 0x5a) }
        }
        let d = t.elapsed();
        if d < best {
            best = d;
        }
    }
    let mib = SIZE as f64 / (1024.0 * 1024.0);
    let gib_per_s = mib / best.as_secs_f64() / 1024.0;
    (gib_per_s, best)
}

// ---------------------------------------------------------------------------
// x86_64: AVX2 / SSE4.2 intrinsics — cfg-gated so aarch64 builds never see the
// `#[target_feature(enable = "avx2")]` attribute (E0635 on non-x86 targets).
// ---------------------------------------------------------------------------

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn simd_bench_avx2() -> u64 {
    let mut acc = std::arch::x86_64::_mm256_set1_epi64x(1);
    for _ in 0..10_000_000 {
        acc = std::arch::x86_64::_mm256_add_epi64(acc, acc);
    }
    std::arch::x86_64::_mm256_extract_epi64::<0>(acc) as u64
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn simd_bench_avx2() -> u64 {
    simd_bench_scalar()
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse4.2")]
unsafe fn simd_bench_sse4() -> u64 {
    let mut acc = std::arch::x86_64::_mm_set1_epi64x(1);
    for _ in 0..10_000_000 {
        acc = std::arch::x86_64::_mm_add_epi64(acc, acc);
    }
    std::arch::x86_64::_mm_cvtsi128_si64(acc) as u64
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn simd_bench_sse4() -> u64 {
    simd_bench_scalar()
}

// ---------------------------------------------------------------------------
// aarch64: real NEON intrinsics (vaddq_u64 / vdupq_n_u64 / vaddvq_u64).
// ---------------------------------------------------------------------------

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn simd_bench_neon() -> u64 {
    let mut acc = std::arch::aarch64::vdupq_n_u64(1);
    for _ in 0..10_000_000 {
        acc = std::arch::aarch64::vaddq_u64(acc, acc);
    }
    std::arch::aarch64::vaddvq_u64(acc)
}

#[cfg(not(target_arch = "aarch64"))]
unsafe fn simd_bench_neon() -> u64 {
    simd_bench_scalar()
}

fn simd_bench_scalar() -> u64 {
    let mut acc: u64 = 1;
    for _ in 0..10_000_000 {
        acc = acc.wrapping_add(acc);
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_bench_finishes() {
        let _ = black_box(simd_bench_scalar());
    }

    #[test]
    fn bench_cpu_returns_bounded_duration() {
        let (level, d) = bench_cpu(1);
        assert!(matches!(
            level,
            SimdLevel::Avx2 | SimdLevel::Sse4 | SimdLevel::Neon | SimdLevel::Scalar
        ));
        assert!(d.as_millis() < 5000, "benchmark should be quick");
    }

    #[test]
    fn bench_memory_returns_sane_bandwidth() {
        let (gb_s, _d) = bench_memory(1);
        assert!(gb_s > 0.0 && gb_s < 200.0, "bandwidth out of range: {gb_s}");
    }
}
