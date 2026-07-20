# Task 3 Report: Simple detection modules

## Status
Complete — all files created, build passes, committed, pushed.

## Files created
- `sysfetch-core/src/modules/mod.rs` — all 19 module declarations (os through custom)
- `sysfetch-core/src/modules/os.rs` — OsModule: reads /etc/os-release (Linux), sw_vers (macOS)
- `sysfetch-core/src/modules/host.rs` — HostModule: /proc/sys/kernel/hostname (Linux), libc::gethostname (macOS)
- `sysfetch-core/src/modules/kernel.rs` — KernelModule: `uname -srm`
- `sysfetch-core/src/modules/uptime.rs` — UptimeModule: /proc/uptime (Linux), sysctl KERN_BOOTTIME (macOS), format_uptime helper
- `sysfetch-core/src/modules/locale.rs` — LocaleModule: LANG, LC_CTYPE, LC_ALL env vars
- Stubs for cpu, memory, disk, gpu, network, battery, processes, shell, terminal, de, wm, packages, colors, custom

## Files modified
- `sysfetch-core/src/lib.rs` — added `pub mod modules;` + test block

## Build
`cargo build -p sysfetch-core` — OK

## Test
`cargo test -p sysfetch-core` — fails on `test_uptime_format`:
- `format_uptime(120)` returns `"2 mins"` (correct: 120s = 2min)
- test expects `"2h 0m"` (typo in brief)
- Known issue, not fixed per "EXACT content from brief" instruction

## Commits
`1f1e42e` feat: os, host, kernel, uptime, locale modules — pushed to main

## Concerns
- Brief test for 120s has wrong expected value. Fix: change `"2h 0m"` to `"2 mins"` in test assertion.
