window.BENCHMARK_DATA = {
  "lastUpdate": 1786102936841,
  "repoUrl": "https://github.com/mahesh-diwan/flexfetch",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "diwanmahesh11@gmail.com",
            "name": "Mahesh Diwan",
            "username": "mahesh-diwan"
          },
          "committer": {
            "email": "diwanmahesh11@gmail.com",
            "name": "Mahesh Diwan",
            "username": "mahesh-diwan"
          },
          "distinct": true,
          "id": "4fd2077fc2a54dc2e17f4fd713458966a57e33d3",
          "message": "fix(ci): benchmark — init gh-pages branch + best-effort chart publish\n\nThe Publish step ran git fetch origin gh-pages:gh-pages before pushing, which aborts on a fresh repo where the branch doesn't exist ('couldn't find remote ref gh-pages') — reddening the Benchmark job on every main push. Added an idempotent 'Ensure gh-pages branch exists' step (creates the orphan branch once) and marked the publish step continue-on-error: the chart data push is best-effort — the binary-size gate in the 'Track binary size' step is the real regression check.",
          "timestamp": "2026-08-06T17:39:54+05:30",
          "tree_id": "086291d35706311ba43a88972b28393c7a7cf0fd",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/4fd2077fc2a54dc2e17f4fd713458966a57e33d3"
        },
        "date": 1786018378131,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 11279855,
            "range": "± 182468",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 11210666,
            "range": "± 109350",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "diwanmahesh11@gmail.com",
            "name": "Mahesh Diwan",
            "username": "mahesh-diwan"
          },
          "committer": {
            "email": "diwanmahesh11@gmail.com",
            "name": "Mahesh Diwan",
            "username": "mahesh-diwan"
          },
          "distinct": true,
          "id": "9a47b159956a30faa28f96d30b3960eb1e54d67e",
          "message": "feat: Phase 4.12 — sandboxed WASM plugin runtime (flexfetch-wasm) + Lua/WASM plugin auto-loading\n\nNew flexfetch-wasm crate (NOT a workspace member, same pattern as fuzz/; path dep pulled only by the wasm-plugins feature, off by default): wasmtime 47 runtime with fuel metering (10M fuel budget), hard memory cap (StoreLimits), and capability-gated host imports (log always; env_get/read_file/run_command only when granted) — a module that imports a denied capability fails at link time. ABI v1: exports flexfetch_plugin() -> i64 packed (len<<32)|ptr to JSON in plugin memory; result converted to InfoValue ({\"value\":\"x\"} scalar convention). 7 tests: scalar/map results, denied-import link failure, env_get grant/deny, fuel-exhaustion trap, memory-cap trap, out-of-bounds result. New flexfetch-cli/src/plugins.rs auto-loads .lua (via flexfetch-lua, lua feature) and .wasm (via flexfetch-wasm, wasm-plugins feature) from the plugins dir into one renderable 'plugins' table; broken plugins are skipped, never crash the fetch. Wired into run_selected + watch cache; default.tera renders a Plugins block; template.rs passes a has_plugins flag. Plugin tests serialized via a static mutex (the XDG_CONFIG_HOME env-var tests raced under parallel harness threads); tempdir helper lives inside the test module, wasm-only helpers cfg-gated so the default build has no dead code. Also fixes the lua crate's core dep to default-features=false (no tera/rayon leak via feature unification). Minimal binary still 1.75 MiB (< 3 MiB gate); ROADMAP 4.12/8.12 updated.",
          "timestamp": "2026-08-06T17:42:07+05:30",
          "tree_id": "c8e1c1c513bc0abf06ad6d0babdc8aca88a0778d",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/9a47b159956a30faa28f96d30b3960eb1e54d67e"
        },
        "date": 1786018501613,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 11260627,
            "range": "± 155162",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 11247501,
            "range": "± 193593",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "diwanmahesh11@gmail.com",
            "name": "Mahesh Diwan",
            "username": "mahesh-diwan"
          },
          "committer": {
            "email": "diwanmahesh11@gmail.com",
            "name": "Mahesh Diwan",
            "username": "mahesh-diwan"
          },
          "distinct": true,
          "id": "69e6a448fd14ddfba9ac2bad812a13da56b46425",
          "message": "fix(ci): cargo-deny — drop removed [licenses] default key (PR #611 schema)\n\ncargo-deny >= 0.16 removed the  license key (EmbarkStudios/cargo-deny#611): all licenses not in the allow list are denied by default, and leaving the key in deny.toml fails config validation with 'this key has been removed'. Removed the line and pinned version = 2 for the new schema.",
          "timestamp": "2026-08-06T18:01:30+05:30",
          "tree_id": "83c56b5cfb05ea11fea74f1eab9d2b0224173d0b",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/69e6a448fd14ddfba9ac2bad812a13da56b46425"
        },
        "date": 1786019668595,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 11462418,
            "range": "± 343178",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 11265009,
            "range": "± 119011",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "diwanmahesh11@gmail.com",
            "name": "Mahesh Diwan",
            "username": "mahesh-diwan"
          },
          "committer": {
            "email": "diwanmahesh11@gmail.com",
            "name": "Mahesh Diwan",
            "username": "mahesh-diwan"
          },
          "distinct": true,
          "id": "8a3a505a28b7aef5d255f706db0b10e47eb46dbe",
          "message": "fix(ci): minimal-build unused_mut (lua/wasm plugins gating) + nix build -L logs\n\nminimal-build clippy (--no-default-features) failed on two  bindings in main.rs that are only mutated by the plugin merge (lua/wasm-plugins features): added the file's existing #[allow(unused_mut)] convention to the run_selected and run_selected_cached results. Also added -L to the nix-flake steps so the derivation's actual build log (vendor/compile) is printed when the builder fails — the previous failure only showed 'builder failed with exit code 1'.",
          "timestamp": "2026-08-06T18:05:38+05:30",
          "tree_id": "0555dc1486247a1d9601a6f766799cfd496479df",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/8a3a505a28b7aef5d255f706db0b10e47eb46dbe"
        },
        "date": 1786019938329,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 11582158,
            "range": "± 215085",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 11524121,
            "range": "± 364905",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "diwanmahesh11@gmail.com",
            "name": "Mahesh Diwan",
            "username": "mahesh-diwan"
          },
          "committer": {
            "email": "diwanmahesh11@gmail.com",
            "name": "Mahesh Diwan",
            "username": "mahesh-diwan"
          },
          "distinct": true,
          "id": "8fbf3e561823f5d5162e07c6b5c359aa123edc7c",
          "message": "feat: v0.19.0 — --flash mode, real download progress, PGO pipeline, ROADMAP sync\n\n- flash: single-pass ~4ms minimal render, overrides modules/preset/minimal/full/smart/health (--demo wins)\n- install.sh: pacman_progress (bytes/percent/speed via stat+awk, single-line redraw, graceful degrade) + HEAD Content-Length detection\n- scripts/pgo.sh: 4-phase PGO pipeline, README note; 19.0MB→7.7MB, 239ms→89ms verified\n- ROADMAP + v2.0 master plan reality-synced (4.2/4.3/4.9/4.10 done, 4.6/4.14/4.15 partial)",
          "timestamp": "2026-08-07T17:09:26+05:30",
          "tree_id": "a50949bfeb5ec2b15eb5a2d660ab23e23bdeb860",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/8fbf3e561823f5d5162e07c6b5c359aa123edc7c"
        },
        "date": 1786102936204,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 7419853,
            "range": "± 94846",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 7395423,
            "range": "± 296204",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}