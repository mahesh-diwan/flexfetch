window.BENCHMARK_DATA = {
  "lastUpdate": 1786117334813,
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
          "id": "5feaf4241e50b3510efe859523e3085898e37fe4",
          "message": "chore: harden install.sh, curl-only distribution, truthful README, drop AI tooling\n\ninstall.sh:\n- bash shebang (arrays), portable numeric tag sort (BSD-safe, no sort -V)\n- cursor-safety EXIT trap + guarded tty escapes (set -e safe in pipe)\n- green success banner with real on-disk size; verified end-to-end (tty + pipe)\n\ndistribution decision (2026-08-07): curl script is the ONLY install channel.\nROADMAP/master plan mark 5.1/5.9/5.11/4.17 rejected; docker job removed from\nrelease.yml; SBOM step fixed (cargo-cyclonedx has no --output-dir) + made\nnon-blocking so a missing SBOM never reds the amd64 artifact.\n\nREADME: truthful measured numbers (0.8 MB shipped/UPX'd, 1.8 MB minimal,\n2.4 MB release-set, 6.9 MB full, 527 logos) + --flash section. Screenshots\nverified against real export output.\n\nrepo hygiene: gitignore AI tooling dirs (.claude/.opencode/.playwright*/.superpowers/.memory/.ruff_cache) and untrack 22 committed agent artifacts.",
          "timestamp": "2026-08-07T17:46:38+05:30",
          "tree_id": "28ab2814ef5f234088c5f23ea1f941b085cb0510",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/5feaf4241e50b3510efe859523e3085898e37fe4"
        },
        "date": 1786105187737,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 11224796,
            "range": "± 306045",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 11227873,
            "range": "± 152826",
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
          "id": "76c4a215fe38ba28f02b0fb8562651e478724c8c",
          "message": "docs: man page via clap_mangen, CHANGELOG for v0.19.0 + v0.20.0\n\n- gen_man example: regenerates doc/flexfetch.1 from clap derive defs\n  (clap_mangen 0.2 dev-dep). Run: cargo run --example gen_man\n- lib.rs: exposes Cli/Commands/PluginAction for examples to import\n- CHANGELOG: added v0.19.0 (--flash, PGO, install.sh hardening) and\n  v0.20.0 (audit cuts, -2300 lines, 7 deps removed) entries\n- doc/flexfetch.1: regenerated, no stale flags, version 0.20.0",
          "timestamp": "2026-08-07T19:04:08+05:30",
          "tree_id": "c7f9184f458d748d8c5853bdd193e09501dcca6f",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/76c4a215fe38ba28f02b0fb8562651e478724c8c"
        },
        "date": 1786109845060,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 11078076,
            "range": "± 247297",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 11047333,
            "range": "± 51277",
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
          "id": "b989f674bc8331b89ff78015edaa8be992450d3d",
          "message": "chore: strip repo bloat, rewrite README 680→110 lines\n\nDeleted non-user-facing files (22):\n- docs/superpowers/ (11) — AI planning artifacts, research, specs\n- fuzz/ (2) — fuzzing targets\n- scripts/ (4) — internal build scripts (kept terminal_matrix.sh for CI)\n- cliff.toml — git-cliff changelog config\n- packaging/ (3) — rejected distro channels (PKGBUILD, formula, action)\n- tests/fixtures/ — unused test data\n- flexfetch-core/create_logos.py — logo generation script\n- doc/plugins.md, doc/templates.md — internal plugin/template docs\n- schemas/config.json — JSON schema\n\nKept: deny.toml (CI audit), registry/plugins.toml (code reference),\nsite/ (hand-maintained landing page).\n\nREADME rewritten: 680→110 lines. Hero image, one-line install, quick\nstart, features bullet list, config pointer, license. No stale flags.",
          "timestamp": "2026-08-07T19:12:37+05:30",
          "tree_id": "5a575cbeed8711081dbbd3b7287e713e98050a37",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/b989f674bc8331b89ff78015edaa8be992450d3d"
        },
        "date": 1786110356211,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 11507216,
            "range": "± 310495",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 11150783,
            "range": "± 303276",
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
          "id": "39b8843f7bbc8dd2d769086f1f0650dc6b2beab5",
          "message": "site: visual design polish\n\n- Inter → Outfit (body font), tighter line-height\n- Desaturated blue accent, blue-violet glow\n- Tighter nav (68→56px), wider hero terminal column\n- Stats border separators, spring easing on feature cards\n- GPU-composited transitions (will-change, transform-only)\n- Snappier cursor blink (1.3→1s), copy button scale press\n- Fixed duplicate emojis (terminal, wallpaper)",
          "timestamp": "2026-08-07T19:36:58+05:30",
          "tree_id": "ebb2ff5ba4f2c055ad76803e0c9dffadbd1cf251",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/39b8843f7bbc8dd2d769086f1f0650dc6b2beab5"
        },
        "date": 1786111847432,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 11111874,
            "range": "± 312655",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 11064068,
            "range": "± 831174",
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
          "id": "2d95543b15fa761db0873afa6496645ee9df0c5e",
          "message": "site: neobrutalist polish, curl-only install, mobile responsive\n\n- install.sh: replace `file` check with `tar -tzf` (portable)\n- site: remove Nix/cargo/AUR install cards, keep curl only\n- site: subtle neobrutalism — 2px borders, hard offset shadows, accent border-top\n- site: remove anti-slop — eyebrow kickers, gradient text, backdrop-filter blur\n- site: hero terminal with full flexfetch output (flat spans)\n- site: pill navbar, hamburger mobile menu\n- site: mobile responsive — stacked hero, 2-col grids, 44px touch targets\n- hero.html: simplified with Catppuccin colors",
          "timestamp": "2026-08-07T20:15:48+05:30",
          "tree_id": "82793463142df7a73d18060f935bb1cdd371bbf6",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/2d95543b15fa761db0873afa6496645ee9df0c5e"
        },
        "date": 1786114131389,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 10300105,
            "range": "± 101216",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 10243670,
            "range": "± 100975",
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
          "id": "dc69c729c05e89258cfa32ef612281d597d1a044",
          "message": "site: bold neobrutalism + DM Mono font\n\n- Font: DM Mono for body + code (monospace developer aesthetic)\n- Headings: Space Grotesk (kept)\n- Borders: 3px solid #fff on all cards/terminal/inputs\n- Shadows: hard offset 6px 6px 0 #000 (terminal), 4px cards, 3px buttons\n- Corners: border-radius 0 everywhere (sharp brutalist), nav pill 9999px\n- Accent: Catppuccin pink (#f5c2e7) on stats, buttons, active chips\n- Primary button: solid pink bg with dark text\n- Typography: 700 weight headings, -0.02em tracking",
          "timestamp": "2026-08-07T20:42:34+05:30",
          "tree_id": "8eec11f2908131aa27854574804ca64ca2a7e348",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/dc69c729c05e89258cfa32ef612281d597d1a044"
        },
        "date": 1786115733974,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 11101333,
            "range": "± 72720",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 11063944,
            "range": "± 79948",
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
          "id": "a3d395165b4d4ba22287917424f24a6f33e26b4c",
          "message": "chore: delete orphaned logo PNGs and unused SVGs\n\n- Delete assets/logos/ (13 distro PNGs + 21 module PNGs) — not referenced anywhere\n- Delete assets/json.svg, assets/themes.svg — unused\n- Keep assets/default.svg (used in README hero)",
          "timestamp": "2026-08-07T20:49:03+05:30",
          "tree_id": "c19a3d0200929b0a7dcbeebd0335a8e823a4c15a",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/a3d395165b4d4ba22287917424f24a6f33e26b4c"
        },
        "date": 1786116124120,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 11693156,
            "range": "± 177680",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 11899577,
            "range": "± 423650",
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
          "id": "d7fc17ef58c5d42f0a19b0f3209db5867058a13c",
          "message": "repo: cleanup stale files, optimize CI\n\nRemoved:\n- ROADMAP.md, GOVERNANCE.md (use GitHub Issues/Discussions)\n- doc/flexfetch.1 (generated man page)\n- scripts/ (dev-only utilities)\n- flake.nix, Dockerfile, .dockerignore (rejected install channels)\n\nCI optimization:\n- ci.yml: path-filtered (only runs on Rust source changes, not docs)\n- ci.yml: removed perf-gate, valgrind, terminal-matrix, nix-flake jobs\n- deep-test.yml: new workflow for perf-gate + valgrind (tags/manual only)\n- Typical push: 5 jobs (test, clippy, fmt, windows, minimal-build) ~3-4 min\n\nUpdated references:\n- CONTRIBUTING.md: removed ROADMAP references\n- PR template: removed ROADMAP checklist\n- Issue templates: linked to Discussions instead of ROADMAP\n- SECURITY.md: removed scripts/ reference\n- README.md: removed ROADMAP link\n- .gitignore: cleaned up stale entries",
          "timestamp": "2026-08-07T20:59:54+05:30",
          "tree_id": "35fec109ca13a09666e0bb4b8d4320fa5c66e592",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/d7fc17ef58c5d42f0a19b0f3209db5867058a13c"
        },
        "date": 1786116782451,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 11158360,
            "range": "± 142564",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 11046286,
            "range": "± 97809",
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
          "id": "a80465305b7436ebf97c383b35def7e9799a67b8",
          "message": "site: redesign — drop neobrutalism for clean dark terminal aesthetic\n\n- CSS: 3px solid borders → 1px subtle, hard shadows → soft/none, cards 8px radius\n- CSS: nav backdrop-filter blur, reduced motion media query, refined hover states\n- HTML: removed numbered markers, feature tags, roadmap section\n- HTML: simplified hero subtext, cleaned footer\n- JS: added smooth scroll + active nav section highlighting",
          "timestamp": "2026-08-07T21:08:58+05:30",
          "tree_id": "15db0fb5b5cbf11922abaecbd5e8876faed3de5e",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/a80465305b7436ebf97c383b35def7e9799a67b8"
        },
        "date": 1786117333101,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 11191980,
            "range": "± 183910",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 11454698,
            "range": "± 192874",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}