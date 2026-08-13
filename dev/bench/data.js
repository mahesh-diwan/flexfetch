window.BENCHMARK_DATA = {
  "lastUpdate": 1786590063483,
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
          "id": "5e3b94bb2d787e24eaab9ceade978838078916ea",
          "message": "site: fix navbar overflow, swap to Outfit font, add custom docs theme\n\n- Navbar max-width 700px → 820px, tighter link padding for fit\n- Font swap: Space Grotesk → Outfit (cleaner pairing with DM Mono)\n- Rounded corners (6px) on mod grid, search, chips, install row, copy btn\n- Hero grid rebalanced 0.42fr/0.58fr\n- Section padding reduced to 88px (56px mobile)\n- Custom mdbook theme: dark terminal-native, Outfit headings, DM Mono body\n- Docs theme matches landing page palette and typography",
          "timestamp": "2026-08-07T21:20:44+05:30",
          "tree_id": "42a8259a4c6bcf5ef1225d720c7eb564a2c338ab",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/5e3b94bb2d787e24eaab9ceade978838078916ea"
        },
        "date": 1786118006681,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 8615333,
            "range": "± 272637",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 8595988,
            "range": "± 39821",
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
          "id": "9c115cd0a3c0118d828fbb29dcab130ad9b9b5a4",
          "message": "site: fix design token consistency — reduce clutter\n\n- Border-radius: 8 values → 3 (6px cards, 0 terminals, 9999px nav)\n- Gap: 20+ values → scale {4, 8, 12, 16, 24, 32, 48}px\n- Border colors: hardcoded rgba → CSS variables (--border, --border-strong, --accent-border)\n- Padding: snapped to consistent scale for cards, inputs, compact elements",
          "timestamp": "2026-08-07T21:38:14+05:30",
          "tree_id": "c1610ebd0911a262a58c98376789fd6c59b67441",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/9c115cd0a3c0118d828fbb29dcab130ad9b9b5a4"
        },
        "date": 1786119090557,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 10289717,
            "range": "± 122925",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 10207519,
            "range": "± 91708",
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
          "id": "accb66ad6559a752c707ce6d24a13f9fa64b5096",
          "message": "site: fix 5 visual bugs found via screenshot diagnosis\n\n- Terminal overflow: overflow-x hidden → auto (scrollable long lines)\n- Scroll-padding: 100px → 120px (navbar no longer overlaps headings)\n- Install URL: word-break break-all → overflow-wrap break-word (natural breaks)\n- Feature grid: explicit align-items stretch (equal height cards)",
          "timestamp": "2026-08-07T22:07:26+05:30",
          "tree_id": "4f4fd1ade6bd39ce82ec2464f443e641ab42cd20",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/accb66ad6559a752c707ce6d24a13f9fa64b5096"
        },
        "date": 1786120855251,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 11400292,
            "range": "± 163207",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 11300241,
            "range": "± 109448",
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
          "id": "2151a24cbbb46d49bbfdab05622f98918722dc59",
          "message": "site: hero — terminal on top, stacked layout\n\nTerminal moved above hero text, full-width, no side-by-side squeeze.\nText centered below. Responsive grid references cleaned up.",
          "timestamp": "2026-08-07T22:13:43+05:30",
          "tree_id": "44b33a9c27fe5ffe49561aa9e918c5eb08d42057",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/2151a24cbbb46d49bbfdab05622f98918722dc59"
        },
        "date": 1786121226782,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 11135972,
            "range": "± 169747",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 11132013,
            "range": "± 157215",
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
          "id": "36fa99456c0c17d32756f499aee728387ee12a54",
          "message": "install: add progress indicators for every silent step\n\n- Resolving latest version... (before API call)\n- Validating download... (before tar check)\n- Extracting... (before tar extract)\n- Installing... (before file move)\n- wget now shows progress (removed -q)",
          "timestamp": "2026-08-08T08:31:47+05:30",
          "tree_id": "4c88490f2e32c1a2923d664eee8665f295a63ed0",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/36fa99456c0c17d32756f499aee728387ee12a54"
        },
        "date": 1786158294829,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 11063724,
            "range": "± 86470",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 11080348,
            "range": "± 468425",
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
          "id": "f6d9ea839dd2944dda42359f53d5acb806005b5d",
          "message": "site: remove themes section\n\n- Removed themes grid from HTML (8 theme cards + note)\n- Removed themes CSS (.theme-grid, .theme, .theme-note)\n- Removed theme swatch JS click handler\n- Removed Themes link from nav (desktop + mobile)\n- Removed Themes/Custom themes from footer\n- Cleaned up responsive theme-grid references",
          "timestamp": "2026-08-08T08:53:42+05:30",
          "tree_id": "49300ecfc9cf91e2f39c5db787fd8b6173bdb4c5",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/f6d9ea839dd2944dda42359f53d5acb806005b5d"
        },
        "date": 1786159615617,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 11079643,
            "range": "± 110238",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 11077858,
            "range": "± 75017",
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
          "id": "7ccf46332634a6a5c9fd8681981ef7a8c74bf419",
          "message": "site+cli: --live matches site mockup, hero gets ASCII logo\n\n- live.rs: flat layout (no bordered blocks), CPU/Memory gauges,\n  network sparkline, footer with net rates + quit hint\n- hero.html: compact ASCII art logo added\n- site: live section description updated (removed disk/swap/battery)",
          "timestamp": "2026-08-08T09:03:52+05:30",
          "tree_id": "022ea7ab60491f3169fe8321946af5d5d3b7e5c1",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/7ccf46332634a6a5c9fd8681981ef7a8c74bf419"
        },
        "date": 1786160226447,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 11533109,
            "range": "± 198754",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 12012007,
            "range": "± 306076",
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
          "id": "400d737cba0d74ededc11d51b90cc1e6eb7b55a4",
          "message": "perfect score push — back-to-top, module search, typing fallback, platform hint, jargon tooltips\n\n- back-to-top button: fixed position, appears after scrolling past hero\n- module search: filter 39 modules by name with live count\n- hero typing animation: line-by-line fallback when hero.html fails to load\n- platform hint: detects OS/arch from UA, shows under install command\n- jargon tooltips: abbr underlines on WASM/supply-chain terms\n- CSS: .btt, .mod-search, .plat-hint, abbr styling",
          "timestamp": "2026-08-08T15:44:06+05:30",
          "tree_id": "e9c1080309c7b2ccc762f81078d8b6241d157588",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/400d737cba0d74ededc11d51b90cc1e6eb7b55a4"
        },
        "date": 1786184651193,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 11549101,
            "range": "± 384776",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 11429679,
            "range": "± 281917",
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
          "id": "b60c6cb8027ecac73abc2ecce83b5f6e8f50c289",
          "message": "fix: render logo without tera feature (unblock release build)\n\nrender_logo and render_ascii_logo were cfg(feature = 'tera') gated but\ncalled unconditionally from TeraEngine::render, breaking the release\nfeature set (live,image-logos,completions,parallel — no tera). Drop the\ngates so logos render in all builds; also silence the unused 'cli' param\nwarning in cli_dispatch when completions is off. Bump v0.30.1.",
          "timestamp": "2026-08-10T23:16:34+05:30",
          "tree_id": "d4675fbde65ba17878ee69d298ce733bb2ceaa25",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/b60c6cb8027ecac73abc2ecce83b5f6e8f50c289"
        },
        "date": 1786384161845,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 12050098,
            "range": "± 102676",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 12115370,
            "range": "± 367860",
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
          "id": "a90473394cb9b9c79a07ea8068201cc23fd689bc",
          "message": "chore: sync Cargo.lock to v0.31.0",
          "timestamp": "2026-08-11T15:59:59+05:30",
          "tree_id": "c1e09dcfa4f77ba39b7bab9830c586082a52a026",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/a90473394cb9b9c79a07ea8068201cc23fd689bc"
        },
        "date": 1786444363019,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2664378,
            "range": "± 34205",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2662934,
            "range": "± 124487",
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
          "id": "9809971886e71030441ae7271c7885c634ef2b47",
          "message": "site: design-system token layer + full refactor (v0.32.0)\n\nAdd DESIGN.md + theme.css as the closed token layer (schema v1, generated).\nLayout imports theme.css and forces class=\"dark\".\n\nglobal.css: replace :root literal block (20 hex) with aliases to semantic\ntokens; 8 inline rgba literals -> color-mix() over --color-accent;\ndrop single-outline :focus-visible (double-layer ring from theme wins);\nfonts IBM Plex Sans -> Inter, display stays mono (JetBrains Mono).\ndocs/[slug].astro: 7 literals -> color-mix(). Shiki -> css-variables\ntheme with --astro-code-*/--shiki-* tokens (code blocks no longer inline\ngithub-dark). theme-color meta -> neutral-950.",
          "timestamp": "2026-08-11T17:28:34+05:30",
          "tree_id": "87995261aaeb5f6f41f6c9958a88407e0f344a2f",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/9809971886e71030441ae7271c7885c634ef2b47"
        },
        "date": 1786449724708,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 1967033,
            "range": "± 43128",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 1961508,
            "range": "± 44384",
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
          "id": "d0833cfa1b4806c711a9dbf22049684232f73365",
          "message": "site: design-system showcase page (live token reference)\n\nRender every token from DESIGN.md live via CSS vars (auto-updates on\ntheme regen). Primitive palette, semantic colors, fluid type scale,\nspacing/radius/elevation, motion durations, breakpoints, live viewport\nindicator. Plain CSS, no React — matches the site's framework-less\nstack. Route: /flexfetch/design-system/.",
          "timestamp": "2026-08-11T17:37:16+05:30",
          "tree_id": "41edb47385538210fd2b1e075bf5c44eebb1246d",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/d0833cfa1b4806c711a9dbf22049684232f73365"
        },
        "date": 1786450189457,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 1914781,
            "range": "± 42418",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 1923821,
            "range": "± 69048",
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
          "id": "8fda1ca340d6fc9794539464cea79d7db94666e4",
          "message": "site: corporate motion pass (motion-design skill)\n\nOne archetype: Corporate. Signature --ease-out everywhere, 0% overshoot.\n\n- --duration-normal alias 500ms -> 300ms (was aliasing to 'slower',\n  making reveal/toast/btt sluggish; 500ms is outside the palette's use)\n- reveal: now actually fires — 'js' class was never added to <html>, so\n  html.js .reveal was dead; reveal-on-scroll was a no-op\n- drawer nav: exit 30% faster than enter (base vs .open transitions)\n- hamburger/caret/toast/copy-btn: spring and raw durations/easings ->\n  token easings; margin-left/top animation -> transform (DESIGN.md 7.3)\n- skip-link + caps arrow + btn active: raw values -> tokens\n- design-system .dur demo: transform transition had no duration (bug)\n- DESIGN.md 7.0: document motion personality + palette",
          "timestamp": "2026-08-11T18:23:53+05:30",
          "tree_id": "052be0c043cc245796d8f1d606fa99a563c9b5a9",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/8fda1ca340d6fc9794539464cea79d7db94666e4"
        },
        "date": 1786452997283,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2584803,
            "range": "± 33199",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2591610,
            "range": "± 31340",
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
          "id": "570e666c2ae3a2e05b55514256f9b7ab6db01db1",
          "message": "site: strip emoji module glyphs (design-taste skill)\n\n38 emoji glyphs overflowed the 10px .glyph box (12px font, overflow\nvisible) — a real visual defect. Module grid now uses the clean\nterminal-dot motif (.glyph spot square), matching the site's mono\naesthetic and the skill's anti-emoji policy.",
          "timestamp": "2026-08-11T18:32:51+05:30",
          "tree_id": "8edc8f81ff9eac5de7ad29f8a3fbc917d6cc8519",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/570e666c2ae3a2e05b55514256f9b7ab6db01db1"
        },
        "date": 1786453544034,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2645842,
            "range": "± 51504",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2636744,
            "range": "± 65065",
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
          "id": "f415bad4ec0d5d24b777eefd08ab5be2f169d26a",
          "message": "site: warm amber/amber terminal theme + warm neutrals\n\n- Brand blue → golden amber (hue 80), 10-step ramp\n- Neutral pure-gray → warm charcoal (#fff8de→#6e2f00) with tint\n- Semantic: dark accent brand-500 with near-black fg-on-accent\n  (classic phosphor: dark text on bright amber), light\n  accent brand-700 (AA), hover brand-800\n- Warning hue 86→100 so it no longer collides with amber\n- Layout theme-color #0a0a0a→#0e0906 (warm)\n- DESIGN.md narrative + theme.css both updated",
          "timestamp": "2026-08-11T19:08:07+05:30",
          "tree_id": "a09e51c891444d4ddbb19f18b85caf736e095fb4",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/f415bad4ec0d5d24b777eefd08ab5be2f169d26a"
        },
        "date": 1786455674571,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2559560,
            "range": "± 43513",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2563418,
            "range": "± 31745",
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
          "id": "6f35951a9ffd4c39b47fa432b28311509450a5af",
          "message": "feat(site): design-taste v5 redesign — bento features, kinetic marquee, live module grid\n\nVisual overhaul of the static landing page following the design-taste directives: Geist + Geist Mono (no Inter/serif); single amber accent on zinc neutrals (competing cyan removed, warm-tinted shadows); asymmetric split hero with the real render terminal, kinetic module marquee, hairline stats band, bento grid of 8 spotlight cards with perpetual micro-interactions (latency bars, swatch pulse, logo shimmer, gauge + sparkline, chip marquee, module typewriter, plugin locks), 39-module filterable grid with empty state, install stack, CTA band. Motion is transform/opacity only: magnetic buttons (CSS-var driven, rAF), 3D terminal tilt with settle snap-back, staggered reveals, html.js progressive enhancement, prefers-reduced-motion. Terminal shows skeleton to real render with error + retry. Fixes: canonical install command (was opencode.ai), dead modules.html link removed. Verified by Playwright: render lines preserved (49), reveals visible (opacity 1), filter + empty state, 0 console errors, 0px mobile overflow.",
          "timestamp": "2026-08-11T21:43:32+05:30",
          "tree_id": "76420948c60143412e99713c03b59cfd3df853f3",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/6f35951a9ffd4c39b47fa432b28311509450a5af"
        },
        "date": 1786464986703,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2579907,
            "range": "± 30752",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2569637,
            "range": "± 35956",
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
          "id": "e5354d1c03ad816eba7d1f6a7a4a26e771a0e545",
          "message": "chore: remove AI planning artifacts, screenshots, and the Astro prototype from the repo\n\nDeletes .impeccable/, animation-plans/, docs/superpowers/ planning docs, all agent screenshots and scratch cjs scripts, the orphaned assets/default.svg, book/src anchor-file junk, and the entire Astro prototype tree that was committed under site/ (src, dist, node_modules 13k files, .astro cache, DESIGN.md, package.json) — site/ now contains only the static landing page (index.html + assets/) plus generated docs. All removed categories are .gitignored so they cannot be re-committed.",
          "timestamp": "2026-08-11T22:05:22+05:30",
          "tree_id": "ddbb174a7936f61ad533ecd39d0f24263f2c3f9c",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/e5354d1c03ad816eba7d1f6a7a4a26e771a0e545"
        },
        "date": 1786466280217,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2582439,
            "range": "± 30092",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2623307,
            "range": "± 23310",
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
          "id": "1f4a7e07afc5959a0219f38971bb69f8bc492a8a",
          "message": "feat(docs): terminal-sheet theme for mdBook docs\n\nRestyle the deployed /docs/ to match the landing page's v2 language: zinc-950 base with amber accent, Geist + Geist Mono typography, hairline borders, amber active chapter links. CSS uses #mdbook-content-prefixed selectors to beat mdBook's default stylesheet (tested against mdbook 0.5.4's runtime DOM). Also correct the intro table: 27 themes, 1.75 MB minimal binary, honest dependency claim.",
          "timestamp": "2026-08-11T22:17:20+05:30",
          "tree_id": "7989e258c75c07b060c66eee21a18c462985d338",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/1f4a7e07afc5959a0219f38971bb69f8bc492a8a"
        },
        "date": 1786467005352,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2709129,
            "range": "± 32332",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2735980,
            "range": "± 308570",
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
          "id": "6c25717a8e871bda7ffe6fcf7a246849dc95e3fb",
          "message": "fix(docs+site): purge blank anchor junk, correct every fabricated claim\n\nDocs and site are now fully grounded in the real binary and source.\n\nBlank entries: mdbook 0.5.4 materializes a stub file for every #fragment link in SUMMARY.md (34 junk files, deployed into /docs/). Removed all #fragment links from SUMMARY; 0 stubs in build output.\n\nThemes: was '28 curated' with a fake 'none' preset and fabricated key/value colors. Real: 27 presets, colors regenerated from theme.rs ANSI codes (e.g. catppuccin bright-blue/bright-cyan, not blue/teal).\n\nCounts: FAQ said 39 modules (real 38 + 2 layout directives), quick-start said 28 themes (real 27), building.md said 77 tests/13 suites (real 103/7).\n\nBuilding: real measured sizes (default 6.8 MB, release pipeline 2.4 MB, minimal 1.75 MB), real --benchmark numbers (32 ms collection / 0.35 ms render / 74 ms total) replacing fabricated flash-mode numbers, correct pipeline feature set (live,image-logos,completions,parallel).\n\nExports: the earlier site 'fix' was itself wrong - SVG/HTML/PNG/Markdown exports DO exist via --export (verified: wrote real .svg/.html/.png), while ansible/terraform/csv/prometheus/github are -f formats. output.md formats table now lists all 8 -f values; README mentions both paths.\n\nConfiguration: removed fabricated [cache] config section (cache is internal-only, 60s TTL hardcoded in context.rs), fixed broken awk example, corrected modules_config TOML shape.\n\nSite: 39->38 modules (title/separator tagged 'layout', JS count excludes them), replaced fabricated Lua+WASM plugins card with truthful QR/diff/ssh 'Share & compare' card, replaced non-existent SLSA claims with real CycloneDX SBOM, honest checksum wording (installer skips verification when no sha256 tool).\n\nSite verified: 21-check Playwright suite green (38-module count, filters, no WASM/SLSA, SBOM present, 0px overflow, 0 console errors). Docs verified: clean build, no anchor junk, theme check green.",
          "timestamp": "2026-08-11T23:17:35+05:30",
          "tree_id": "fa4bb6891f1b7c1bff1e35464961ce4683169c0c",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/6c25717a8e871bda7ffe6fcf7a246849dc95e3fb"
        },
        "date": 1786470589895,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2161952,
            "range": "± 24403",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2173057,
            "range": "± 18474",
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
          "id": "e870340333591546833c32f1c1b32d218d71f797",
          "message": "fix(site): correct stale benchmark numbers, polish data typography\n\nTruth fixes after a full audit against the real binary and source:\n\n- Latency widget: 25.8 ms -> 33 ms (real run_selected), 0.22 ms -> 0.37 ms (real template render); cpu stays 0.7 ms. Bar widths re-proportioned.\n\n- Head-note no longer contradicts card A: core modules read /proc/sysfs/macOS APIs directly, a few optional ones shell out only when needed.\n\n- Hero kicker: 'reads /proc directly' -> 'reads the system directly' (macOS has no /proc).\n\n- Card A: 'finishes in under a millisecond' -> 'typically finishes in under a millisecond' (matches real 0.55-1.08ms spread).\n\nPolish per design skills:\n\n- data-count count-up micro-interaction for hero + stats band numbers (rAF, cubic ease-out, fires once on reveal, prefers-reduced-motion + no-IO fallbacks).\n\n- font-variant-numeric: tabular-nums on all data numerals; text-wrap: pretty on paragraphs; subtle .stat hover.\n\nVerified: 38 modules / 527 logos / 1.75 MB minimal / 3 Linux musl + 2 macOS targets / cosign+SBOM all confirmed true against source; Playwright site+docs suites green, 0 console errors, 0px overflow.",
          "timestamp": "2026-08-12T19:00:03+05:30",
          "tree_id": "11c778e551cd89c68a4c5f06a469c23c7b801c23",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/e870340333591546833c32f1c1b32d218d71f797"
        },
        "date": 1786541569487,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2709439,
            "range": "± 31351",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2732403,
            "range": "± 50009",
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
          "id": "4dacf1ecd77dec2fa28869651d9424196b14cd55",
          "message": "feat(core): real filesystem seam behind Context — modules testable with MockFs\n\nPer the codebase-design skill: Context::read_file was a fake seam (doc promised mock data, nothing injectable). Now a real one — two adapters (RealFs in prod, MockFs in tests).\n\nfs.rs: FileSystem trait (read_to_string/read_dir/exists/is_dir — the whole read surface), RealFs, cfg(test) MockFs with implicit parent-dir registration, test_ctx() helper.\n\nContext: fs field + with_fs constructor + read_dir/exists/is_dir delegates; Context::new keeps the small public interface.\n\n22 module collectors migrated from std::fs to ctx.* — /proc, /sys, /etc, package DB, and ~/.config reads (wallpaper/wm). gpu symlink + weather cache mtime stay on std::fs (documented edge cases).\n\nautotheme builds a throwaway RealFs Context for detect_wallpaper (feature-gated, off the hot path); template.rs: 4 tera-only tests gated so --no-default-features builds (pre-existing minimal-build breakage).\n\n18 new mock-fs unit tests: host, os, cpu, battery, processes, packages, health, container, fsdeep, resolution now verified against fake /proc and /sys trees.\n\nVerified: 125 workspace tests, 112 minimal-build tests, clippy -D warnings clean, all-features check clean.",
          "timestamp": "2026-08-12T19:34:45+05:30",
          "tree_id": "e55a5348c84836d6033e0d60493a72880a66bd50",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/4dacf1ecd77dec2fa28869651d9424196b14cd55"
        },
        "date": 1786543652923,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2755027,
            "range": "± 40334",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2739995,
            "range": "± 28254",
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
          "id": "2ea6645c204134eeb570c52650597161a8cffb2e",
          "message": "fix(fs): MockFs contract fidelity — missing dir is Err, ancestors are dirs\n\nnomistakes audit of the fs seam caught two adapter-drift bugs where a test could pass against MockFs while production took a different branch:\n\n1. MockFs::read_dir returned Ok(empty) for an unregistered dir; RealFs (and the trait doc) return Err. Now Err — plus a contract test locking it in.\n\n2. MockFs::file/dir only registered the immediate parent as a dir, so read_dir on a grandparent (e.g. /sys/class/power_supply) silently found nothing. Now register_ancestors() builds the whole chain; test asserts is_dir/exists/read_dir on each level.\n\n3. Documented RealFs read_dir's intentional best-effort entry filtering (permission-denied / vanishing procfs entries are dropped, not fatal).\n\nAdded 4 fs.rs contract tests (missing-dir Err, ancestor walk, direct-children-only listing, missing-file Err).\n\nVerified: 123 core tests (97 lib + integration), 125 workspace, minimal build, clippy -D warnings clean.",
          "timestamp": "2026-08-12T19:42:46+05:30",
          "tree_id": "772c2a149f77f51b983aa2758fd1b016797920d4",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/2ea6645c204134eeb570c52650597161a8cffb2e"
        },
        "date": 1786544128724,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2595277,
            "range": "± 48003",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2571620,
            "range": "± 73338",
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
          "id": "c23533414a11d788f70fe76baf4c6059d7b8eb52",
          "message": "fix(modules): nomistakes audit — saturating arithmetic + parse guards for untrusted /proc input\n\nFull panic/overflow audit of every module collector parsing /proc and /sys. Indexed accesses and len guards were already sound; five real hazards fixed:\n\ncpuusage: /proc/stat total was a plain sum() — panics in debug on near-u64::MAX counters (my new test caught it: attempt to add with overflow). Now a saturating fold + saturating_add idle; idle>total degrades to 0% not UB.\n\nswap: total/used accumulation could overflow on malformed /proc/swaps — saturating_add; percent clamped to 100 (was rendering 147% for used>size).\n\nhealth: swap probe used parse().ok()? inside the loop — one garbage meminfo line aborted the entire probe. Now if-let skips bad keys. disk_usage_percent statvfs subtraction saturating.\n\ndisk: statvfs total-avail subtraction saturating (f_bavail>f_blocks is kernel-impossible but a virtualized fs could report it).\n\n7 new regression tests covering the exact malformed-input paths: huge counters, used>size, garbage lines with/without colons.\n\nVerified: 131 workspace tests, 98 minimal, clippy -D warnings clean.",
          "timestamp": "2026-08-12T19:56:32+05:30",
          "tree_id": "e2c422a21cf9f61a3102c175e4042076c38a3a4e",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/c23533414a11d788f70fe76baf4c6059d7b8eb52"
        },
        "date": 1786544958609,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2635455,
            "range": "± 28394",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2643600,
            "range": "± 45328",
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
          "id": "3385c16b84b89b629fa84fee3c6d552a200a8dbd",
          "message": "fix(logo): recover from poisoned logo cache lock instead of panicking\n\nnomistakes audit of the CLI + remaining core files (main.rs, tools.rs, render_diff, template/export/theme/config, hardware_db, image_logo, cache, module_registry) — the diff[0]/[1] and run_selected_times[0] indexing is guarded, render_diff is safe, and publicip already handles poison.\n\nThe one real hazard: cached_fastfetch_logo used lock().unwrap() on the global FF_LOGO_CACHE. A thread panic while holding the lock (e.g. during a hashmap op in --live mode) would poison it and panic on every subsequent detect() call, bricking the whole fetch. Now lock().unwrap_or_else(|e| e.into_inner()) recovers the still-valid data.\n\nRegression test: deliberately poisons the mutex via catch_unwind, asserts is_poisoned(), then assert detect('arch') still returns a logo. Fails on the old code (PoisonError panic), passes on the new.",
          "timestamp": "2026-08-12T20:06:09+05:30",
          "tree_id": "072a3263ac134e5522db885b38e89531c4752126",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/3385c16b84b89b629fa84fee3c6d552a200a8dbd"
        },
        "date": 1786545529997,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2155118,
            "range": "± 41517",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2173124,
            "range": "± 43907",
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
          "id": "9d2356e5c411a1d8a8f0e1da8cb82eec016e9e89",
          "message": "harden: fail-closed checksums, exact u128 percent math, config/preset validation, parser fuzzing\n\nAll three followups from the nomistakes audit:\n\n[install.sh] set -euo pipefail; checksum verification is now fail-closed — a fetch failure or missing sha256 tool aborts the install instead of silently skipping (release.yml always publishes .sha256, so skip was a genuine integrity gap). Guarded the hash pipeline so a failing tool gets a clean message.\n\n[config.rs] migrate_config no longer truncates the schema version with  — 2^32+1 wraps to 1 and silently passed the current-version check; now try_from + refuse. Config::load warns loudly (load_layer) instead of silently swallowing corrupt configs. load_preset rejects traversal names (/ \\ . ..) before touching the fs.\n\n[overflow] memory.rs + health.rs percent math moved to exact u128 (saturating_mul was wrong: MAX*100/MAX would render 1% instead of 100%); meminfo fallback sums and macOS page sums now saturating. Regression tests lock in the exact 100% values.\n\n[proptest] new property tests fuzz the /proc,/sys parsers (since_boot_usage, swap, swap_percent, load_per_core, meminfo collect, human_size, migrate_config) — never panic and stay in-range on arbitrary input.",
          "timestamp": "2026-08-12T20:38:50+05:30",
          "tree_id": "3a74c3ec643d956aa2bd7ed282d560bccddfca84",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/9d2356e5c411a1d8a8f0e1da8cb82eec016e9e89"
        },
        "date": 1786547504244,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2784853,
            "range": "± 57724",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2759024,
            "range": "± 52644",
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
          "id": "84c69079ea524f98c7f99c449b815df0970041d0",
          "message": "site: self-host Geist fonts, fix contrast floor, trim em-dash overuse\n\nCritique-driven polish (impeccable critique of site/index.html, 30/36 -> fixes for the top issues):\n\nFonts: replace the Google Fonts dependency with self-hosted variable woff2\n(Geist + Geist Mono, latin subset, ~56K total). The page's only network\ndependency is gone; font-display: swap + preload added. The overused-font\ndetector hit also disappears.\n\nContrast: --muted raised to #8a8a94 (4.07:1 -> 5.75:1) and --faint to #7a7a85\n(2.54:1 -> 4.64:1); both now pass WCAG AA on the #0b0b0d background. Bumped\nthe two smallest mono notes (.lat-note 10.5px, .mod .mtag 9.5px) one step.\n\nCopy: trimmed em-dashes from 26 to 3 deliberate ones (title, og:title,\nterm-title); six body-copy dashes converted to colons/periods. The remaining\ndetector hit is a false positive on the raw-file path (it counts --i/--h CSS\ncustom-property syntax in inline style attrs and --export-style CLI flags).",
          "timestamp": "2026-08-12T22:13:55+05:30",
          "tree_id": "8eaeb227920e8c76f6b06fc6665f63d7f7619982",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/84c69079ea524f98c7f99c449b815df0970041d0"
        },
        "date": 1786553181275,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 1807108,
            "range": "± 105630",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 1765121,
            "range": "± 50669",
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
          "id": "8a66021cd8d9582d6711f2e4bbeb4d7459a1090b",
          "message": "site: fix remaining critique issues — dark OG image, live gauge demo, retire faint tier, self-host docs fonts\n\nSecond critique pass (re-run scored 30/36 with all previous P1/P2 items resolved; this closes the remaining backlog):\n\n- [og.png] Regenerated as a dark 1200x630 blueprint-style card (zinc ground, dot grid, amber ASCII motif, sample terminal output). Includes an explicit \"sample output\" label and deliberately generic values so it can't be mistaken for a fabricated real capture — the hero terminal remains the page's only \"real output\" claim.\n- [main.js] Added a second interactive proof: the Live TUI card's gauge + sparkline now run an organic ticker (random-walk CPU percent, scrolling sparkline) when scrolled into view. Gated on prefers-reduced-motion, pauses offscreen via IntersectionObserver, and skips ticks while the tab is hidden.\n- [style.css] Retired the --faint tier entirely (17 usages folded into --muted #8a8a94; token removed) — one less text tier, everything now AA-clean.\n- [book.css] Docs theme now self-hosts the Geist fonts from site/assets/fonts (two-level relative path from the built theme/ output) instead of Google Fonts, and matches the landing contrast tokens (--muted #8a8a94, --faint #7a7a85). Built docs verified: 2 @font-face, 0 googleapis refs.\n- [.impeccable/critique/ignore.md] Documented the em-dash-overuse finding as a false positive (CLI flags + CSS custom-property syntax in inline styles) so future critique runs stay clean.",
          "timestamp": "2026-08-13T08:28:26+05:30",
          "tree_id": "e10e08ba869437174ed50ad8d9883f8360ae9138",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/8a66021cd8d9582d6711f2e4bbeb4d7459a1090b"
        },
        "date": 1786590062451,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2538001,
            "range": "± 24729",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2541877,
            "range": "± 29483",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}