window.BENCHMARK_DATA = {
  "lastUpdate": 1787308157853,
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
          "id": "9bfbbf43708516d99650d9b8d8d3300c1377dad6",
          "message": "site: polish pass + hero terminal fetch-cascade animation\n\nTwo impeccable commands in sequence: polish (final quality pass) then animate (motion thesis).\n\nPOLISH:\n- Fixed a real defect: the hero terminal clipped the ASCII logo (420px viewport vs 1071px content — only 20 of ~49 lines visible, cutting the logo mid-art). Set .term-body to 11px/1.55 mono with min/max-height 394/432px so the full 22-line logo + user line always fit; the fold now lands on the \"── System ──\" header, with a 44px bottom-fade hint (pointer-events: none) that there's more output below.\n- Verified via headless browser: logo fully visible, no horizontal overflow at 390px, 0 console errors, all interactions pass.\n\nANIMATE (motion thesis: the hero terminal fetches itself on load):\n- The product is a fetch tool whose hero IS its real output. The focal moment: the ASCII logo and system lines print top-to-bottom in a fast cascade (~0.8s, 16ms/line stagger), echoing \"fetched in milliseconds\" — a bounded one-shot, not a loop.\n- main.js now splits the fetched hero.html into per-line spans (filtered to drop the trailing empty line); CSS .hero-line uses lineIn 0.26s ease with explicit from/to keyframes and base state visible (fill-mode both hides each line only during its stagger delay — fixed a first-pass bug where lines animated 0 -> 0 and never appeared).\n- Reduced-motion: the media query now also zeroes .hero-line animation-delay, so lines appear instantly (verified via emulation: all lines opacity 1, delay 0s).",
          "timestamp": "2026-08-13T08:54:44+05:30",
          "tree_id": "32b4281f6a94d9d082f85523b1a09f2da49751bd",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/9bfbbf43708516d99650d9b8d8d3300c1377dad6"
        },
        "date": 1786591646841,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2706185,
            "range": "± 107903",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2704931,
            "range": "± 34817",
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
          "id": "421da38ef146144579fc1e3384e489632f364696",
          "message": "fix: correct stale/fabricated claims after full truthfulness audit\n\n- latency bars: all modules 33->26 ms, template render 0.37->0.33 ms, cpu 0.7->0.65 ms (real --benchmark values on author machine)\n\n- checksum behavior: installer refuses to run without a sha256 tool; site + installation.md previously claimed it 'skipped with a warning' (install.sh actually fails closed)\n\n- building.md: benchmark table to real measured ranges (collection 23-28 ms, render 0.27-0.41 ms, full pipeline cold ~130 ms / warm ~57 ms), os ~30 us, cpu ~0.65 ms\n\n- test count 103 -> 153 (cargo test: 8+119+6+6+7+7)\n\n- faq.md: 5th catalog section is Processes, not 'Context & extras'",
          "timestamp": "2026-08-13T09:20:28+05:30",
          "tree_id": "aff02a61ea46083ff9700f31f4e7929eee9f0ff5",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/421da38ef146144579fc1e3384e489632f364696"
        },
        "date": 1786593195010,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 1976666,
            "range": "± 52751",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 1997074,
            "range": "± 45607",
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
          "id": "464243f2526970bdb785c1154f593dd871af5dea",
          "message": "perf: cache slow modules (wifi/display/packages) to cut startup 45ms → 7ms\n\nDiagnosis (--benchmark): wall time was dominated by module collection —\nwifi spawned nmcli (~35ms), display spawned xrandr (~4.5ms), packages\nread the 69k-entry pacman DB (~5-11ms) on every run, even though their\nvalues barely change. The 60s TTL Cache already existed in Context\n(used by publicip) but these three never touched it.\n\nReuse the established cache pattern: check ctx.cache first, collect +\nstore on miss. First run unchanged; subsequent runs within 60s skip the\nspawns entirely. Verified 45ms → 7ms median warm on the default build.\n\nAlso fix the nondeterministic plain renderer exposed by the cache diff:\nwifi/terminal fell through to a catch-all that pulled arbitrary\nHashMap order (output varied between runs). Added deterministic arms\nmirroring default.tera and sorted keys in the catch-all, with the\nplain-vs-Tera byte-parity test extended to cover both modules.\n\n🤖 Generated with Codebuff\nCo-Authored-By: Codebuff <noreply@codebuff.com>",
          "timestamp": "2026-08-13T14:25:47+05:30",
          "tree_id": "1a3be34dc2562f8f3c9c6b0f44fd32789063de42",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/464243f2526970bdb785c1154f593dd871af5dea"
        },
        "date": 1786611504704,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2733660,
            "range": "± 134045",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2708216,
            "range": "± 169212",
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
          "id": "6311ae9aba67df0345c24d0eac13d63c1899fd05",
          "message": "perf: add `iw` fast path for wifi (35ms→3ms cold) + cache bluetooth/media\n\nContinuing the startup audit: wifi's nmcli fallback was the last big cold\ncost (~35ms every run on systems without iwgetid). `iw dev <iface> link`\nreturns the SSID in ~3ms, so add it as the middle tier (proc → iwgetid →\niw → nmcli) with a parser + 4 unit tests.\n\nAlso apply the established 60s TTL cache to the remaining subprocess\nmodules: bluetooth (2× bluetoothctl, ~15ms) and media (dbus-send,\n~14ms) — both opt-in but paid the spawn cost on every run.\n\nVerified: cold module collection with cache cleared is ~15ms (was ~45ms);\nsteady-state median 8.9ms; cached/uncached output identical; 157/157\ntests pass; clippy clean.\n\n🤖 Generated with Codebuff\nCo-Authored-By: Codebuff <noreply@codebuff.com>",
          "timestamp": "2026-08-13T14:36:19+05:30",
          "tree_id": "bf2d67daf09dde4f8f57df063bba87ef050258d8",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/6311ae9aba67df0345c24d0eac13d63c1899fd05"
        },
        "date": 1786612137743,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2525948,
            "range": "± 22624",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2536789,
            "range": "± 28986",
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
          "id": "b13de4111f9de6d57c9db440aa12a668f97d907d",
          "message": "feat: configurable cache_ttl + refresh perf docs to measured ms values\n\nAll three followups from the startup audit:\n\n1. cache_ttl config key (default 60s, 0 = always re-collect): added to\n   Config with merge/default handling, wired through Context via a\n   set_ttl on Cache that keeps loaded entries, and applied at the three\n   production load sites (config_load, watch hot-reload, live reload).\n   Verified: ttl=1 expires and re-collects after ~2s; hit runs ~10ms.\n\n2. Docs/site refreshed with the new measured numbers (all ms):\n   warm run ~9ms (6-11), cold run ~14ms (13-16), run_selected ~3ms,\n   template render ~0.5ms, cpu ~1.1ms — replacing the stale 26ms/\n   0.65ms/0.33ms figures from before the caching + iw work.\n\n3. live/watch audit: both already optimal — live samples /proc directly\n   on a 1s tick (never touches cached modules); watch serves static\n   modules from its snapshot and re-collects only dynamic ones (~13ms\n   tick on a 1s cadence). No changes needed; documented.\n\n157/157 tests pass, clippy clean.\n\n🤖 Generated with Codebuff\nCo-Authored-By: Codebuff <noreply@codebuff.com>",
          "timestamp": "2026-08-13T14:48:28+05:30",
          "tree_id": "4f3a7c787ec7fe214d6d5c934308b831623f46f2",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/b13de4111f9de6d57c9db440aa12a668f97d907d"
        },
        "date": 1786612874596,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2732598,
            "range": "± 38532",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2749696,
            "range": "± 36804",
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
          "id": "20c41b8c8bc36300eaadbd7cee39108fcf4221d5",
          "message": "fix: green CI on Windows — cfg-gate Linux-only helpers, benchmark cache state\n\nCI was red on every commit (all 11 runs failed, pre-existing): the\nWindows job builds with --no-default-features --all-targets and runs\nclippy with -D warnings, and multiple modules reference Linux-only\ncode paths unconditionally.\n\nFixes:\n- wifi.rs: cfg-gate iw_link_ssid/parse_iw_link/parse_wireless/\n  quality_percent to Linux (they were dead code on Windows) and gate\n  the test module to Linux; allow unused ctx on non-Linux.\n- 11 modules (battery, cpucache, cpu, disk, dns, gpu, memory, network,\n  os, resolution, swap): allow unused_variables on collect (ctx is only\n  read in Linux-gated blocks).\n- cpuusage.rs: gate its tests to Linux (they reference the Linux-only\n  since_boot_usage).\n- live.rs: allow dead_code on ProcInfo.cpu_pct for non-Linux builds.\n- cargo fmt applied to the module cache edits from the perf work.\n\nAlso: --benchmark now prints a `cache: warm|cold` line sampled before\ncollection, so the per-module (cold) vs run_selected (warm) split is\nvisible instead of ambiguous.\n\nVerified locally: Linux tests 157/157, clippy 0, fmt clean; Windows\nclippy 0/0 on both crates (the Windows test link step can't run here —\nno Windows linker — but CI has the full toolchain).\n\n🤖 Generated with Codebuff\nCo-Authored-By: Codebuff <noreply@codebuff.com>",
          "timestamp": "2026-08-13T15:12:25+05:30",
          "tree_id": "64ec0e69f979ff0c86424c86a87a356e1f6803e2",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/20c41b8c8bc36300eaadbd7cee39108fcf4221d5"
        },
        "date": 1786614294061,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 1765216,
            "range": "± 103022",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 1710717,
            "range": "± 55852",
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
          "id": "64b0298222ecb0e4035a6e3585c04fccdf6bf418",
          "message": "fix: security audit workflow — cargo-deny config + crate licensing\n\nThe Security Audit workflow (cargo-deny) was failing on every run with\n\"failed to validate configuration file ./deny.toml\" and, once the config\nwas fixed, on real license/ban errors that had been masked by the broken\nconfig:\n\n- deny.toml: `copyleft = \"deny\"` is a removed key (cargo-deny >= 0.16,\n  PR #611); with licenses version 2 every license not in `allow` is\n  denied by default, so the key was redundant. Also swap the stale\n  `Unicode-DFS-2016` allowance for `Unicode-3.0` (unicode-ident now\n  declares the v3 license), keeping Zlib (foldhash) and BSL-1.0.\n- flexfetch-core/Cargo.toml + flexfetch-cli/Cargo.toml: inherit\n  `license.workspace = true` (the crates were \"unlicensed\" to cargo-deny\n  despite the workspace declaring MIT).\n- flexfetch-cli/Cargo.toml: add a version to the flexfetch-core path\n  dependency — the wildcard requirement failed the bans check.\n\nVerified locally with cargo-deny v0.20.2 (same as the CI action):\nadvisories ok, bans ok, licenses ok, sources ok. Build + 157 tests pass.\n\n🤖 Generated with Codebuff\nCo-Authored-By: Codebuff <noreply@codebuff.com>",
          "timestamp": "2026-08-13T16:09:24+05:30",
          "tree_id": "419dd7c8962579d293ed3b52b5b9690593f829e5",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/64b0298222ecb0e4035a6e3585c04fccdf6bf418"
        },
        "date": 1786617721775,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2727965,
            "range": "± 63690",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2666329,
            "range": "± 46957",
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
          "id": "58b2aca2511a65d3099e22e8cad012d3abb2cefe",
          "message": "fix(security): upgrade rqrr 0.3→0.10 to drop vulnerable lru 0.6.6 (RUSTSEC-2021-0130)\n\ncargo-audit flagged a use-after-free in lru 0.6.6, pulled in transitively by\nrqrr 0.3.2 via the qr feature. rqrr 0.10.1 uses lru 0.16 and keeps the same\nprepare_from_greyscale/detect_grids/decode API, so no code changes were needed.\n\n🤖 Generated with Codebuff\nCo-Authored-By: Codebuff <noreply@codebuff.com>",
          "timestamp": "2026-08-13T16:19:39+05:30",
          "tree_id": "b4392d5d6d8cb108e81845e600fa2a2e4a8f4e69",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/58b2aca2511a65d3099e22e8cad012d3abb2cefe"
        },
        "date": 1786618349042,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2889858,
            "range": "± 130959",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2940639,
            "range": "± 147266",
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
          "id": "0f0c5ce3ee607e95664e5401b3ab52569276844d",
          "message": "fix(ci): grant checks:write so security audit's check run succeeds\n\nrustsec/audit-check@v2 creates a GitHub check run to report results; the\nworkflow's permissions block only granted contents/issues, so every run failed\nwith \"Resource not accessible by integration\" after a clean audit. Also skip\nthe audit job on dependabot pushes — their GITHUB_TOKEN is read-only by design\nand can never create check runs (daily schedule + main pushes still cover it).\n\n🤖 Generated with Codebuff\nCo-Authored-By: Codebuff <noreply@codebuff.com>",
          "timestamp": "2026-08-13T16:26:37+05:30",
          "tree_id": "4f04503315a3677e9abc58735932513b402f7ed7",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/0f0c5ce3ee607e95664e5401b3ab52569276844d"
        },
        "date": 1786618762887,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2557179,
            "range": "± 26405",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2564596,
            "range": "± 28384",
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
          "id": "2213b8e6544d7f68c43cb2df163dd03f61e26b45",
          "message": "fix(security): write weather cache with 0600 perms to match cache.rs\n\nThe nomistakes audit found one real gap: weather.rs wrote its cache\n(temp + rename) with default umask permissions, while cache.rs — the\nestablished pattern — deliberately sets 0o600 on Unix. The weather cache\nholds location-identifying data (city/coordinates) that shouldn't be\nworld-readable on shared machines. Aligned with the cache.rs\nOpenOptions + mode(0o600) pattern; everything else in the audit was\nclean (unwraps on internal invariants only, guarded indexing, no shell\ninjection, PID/hash-suffixed temp files).\n\nGenerated with Codebuff 🤖\nCo-Authored-By: Codebuff <noreply@codebuff.com>",
          "timestamp": "2026-08-13T21:01:19+05:30",
          "tree_id": "d1073bd3b7cb28abf4ee08c889738d4b84739695",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/2213b8e6544d7f68c43cb2df163dd03f61e26b45"
        },
        "date": 1786635223764,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 1913540,
            "range": "± 72492",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 1910091,
            "range": "± 61285",
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
          "id": "7080e5aff4de96bf9132bc82f279285d4a947920",
          "message": "feat(site): sleek aligned hero terminal + live TUI dashboard card\n\nHero terminal: replaced the stale, ragged capture (CachyOS logo with\nwrapped info columns and a duplicated section) with a compact aligned\nrender — the FLEXFETCH brand logo on the left, real system values in a\nclean key/value column, site palette, and no lines wider than the\nterminal's ~68ch capacity at any viewport. The hero now stacks below\n1200px so the terminal keeps full width instead of clipping.\n\nLive card: upgraded the single gauge + sparkline into a mini live-TUI\ndashboard mock — three SVG ring gauges (cpu/memory/disk in site\naccent colors), per-process bars, and a network sparkline inside a\nterminal-framed panel with a pulsing 1s tick.\n\nVerified in a headless browser across 1440-480px: no hero wrapping,\nno page-level horizontal scroll, rings/percentages/bars all computed\ncorrectly, zero console errors.\n\nGenerated with Codebuff 🤖\nCo-Authored-By: Codebuff <noreply@codebuff.com>",
          "timestamp": "2026-08-13T21:26:57+05:30",
          "tree_id": "479a45718da9f24dbecb2d16e60eedd47dcf7736",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/7080e5aff4de96bf9132bc82f279285d4a947920"
        },
        "date": 1786636785021,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2677971,
            "range": "± 48466",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2625073,
            "range": "± 59415",
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
          "id": "7308495458e30d24b284aefac1209b1083c0fba2",
          "message": "fix(site): mobile responsiveness — phantom bento column, page-level scroll\n\n- .card:nth-child(7) keeps grid-column: span 2 from the ≤1024 block in the\n  1-column ≤768 layout, creating a phantom implicit column that squeezed\n  every bento card into ~54px-wide columns (1525px-tall cards) on phones.\n  Reset it to span 1 at ≤768 — verified single-column stacked layout.\n- .term-wrap::before glow (inset -10%) escaped the relative parent and\n  pushed the page wide at ≤480px; clip it with overflow: clip.\n- ≤400px media step (9px mono, tighter chrome) so 60-col hero output fits\n  the ~330px content area; trimmed Kernel/Memory/Disk values.\n- Hero presentation: ghost offset frame → soft clipped amber glow,\n  version/perf badges in the title bar, bottom status bar, blinking cursor,\n  fixed-width logo column so all key/value colons align.\n- Verified 320→1440px: no page-level horizontal scroll, bento 1-col on\n  mobile / 2-col ≤1024 / 3-col desktop, terminal fits exactly ≥390px.",
          "timestamp": "2026-08-14T18:38:16+05:30",
          "tree_id": "d64bfb6acbd77ec937881e1f475c2028b459796e",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/7308495458e30d24b284aefac1209b1083c0fba2"
        },
        "date": 1786713057111,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2746258,
            "range": "± 89348",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2641726,
            "range": "± 56475",
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
          "id": "9a1ea63651d3941fc160e7d2bc2491d8280d87bf",
          "message": "fix(site): ellipsize module names so chips never overflow on narrow phones\n\n.mod .mname could not shrink (flex item, no min-width), so a long module\nname + tag pushed content past the cell edge — clipped by .mods-grid's\noverflow:hidden at 320-390px. Add min-width:0 + nowrap + text-overflow:\nellipsis so the name truncates gracefully and the tag stays visible.",
          "timestamp": "2026-08-14T19:22:05+05:30",
          "tree_id": "efb561403173bf5632b2739a998328d461750297",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/9a1ea63651d3941fc160e7d2bc2491d8280d87bf"
        },
        "date": 1786715671251,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 1631169,
            "range": "± 74279",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 1616481,
            "range": "± 79696",
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
          "id": "8ac55c54ceb2f79e3105403903a3b6ccd925d076",
          "message": "fix(site): a11y — main landmark, skip-link target, mobile menu focus\n\n- Wrap page content in <main id=\"main\"> (hero through CTA; footer stays\n  outside) so screen-reader users can jump to content; skip-link now\n  points at #main instead of #features.\n- Hamburger: add type=\"button\" (correct default for a non-form button).\n- Mobile menu: move focus to the first menu link when it opens, so\n  keyboard/screen-reader users land inside the menu (Escape already\n  closed it and restored focus to the hamburger).\n- Verified: landmark structure, aria-expanded toggling, focus in/out,\n  and unchanged layout at 320/390/1280 (bento columns, hero height,\n  zero page scroll).",
          "timestamp": "2026-08-14T19:31:30+05:30",
          "tree_id": "836377f0a8f8b94d98a438c76e78b4bcb3d5225f",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/8ac55c54ceb2f79e3105403903a3b6ccd925d076"
        },
        "date": 1786716249536,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2593921,
            "range": "± 45222",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2584966,
            "range": "± 51966",
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
          "id": "b3d9012458d3ff11a1df0d1cf2f44ff0240bd65a",
          "message": "refactor: extract render_diff/render_prompt into render_output, benchmark into bench.rs\n\nmain.rs drops to pure orchestration: render/diff/prompt/export now live in\nrender_output.rs and the benchmark loop in bench.rs, each behind a small\ninterface. Also collapse the five identical single-info export arms in\nrender() into one shared shape.\n\nSigned-off-by: Mahesh Diwan <maheshdiwan@proton.me>\nSigned-off-by: Mahesh Diwan <diwanmahesh11@gmail.com>",
          "timestamp": "2026-08-14T19:53:57+05:30",
          "tree_id": "16001cb90e815b412a3d059fb5c49d5f144a9644",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/b3d9012458d3ff11a1df0d1cf2f44ff0240bd65a"
        },
        "date": 1786717731517,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2696311,
            "range": "± 30096",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2700032,
            "range": "± 37806",
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
          "id": "dc9913f66c45c766b3fa6c622fe71c8a9ac5fe9e",
          "message": "fix(site): real hero logo + colors, trim module grid, rework install, fix card demos\n\n- Hero terminal now shows the real fastfetch diamond logo and the real\n  16-block colors module, extracted from the tool's actual output (the old\n  hand-drawn diamond and palette chars matched nothing).\n- Marquee and module chips listed only 25 of 38 modules and the duplicate\n  loop copy was visible under prefers-reduced-motion; both now carry the\n  full set and the second copy hides when animation is off.\n- Module grid trimmed: 18 notable modules shown by default, the rest hidden\n  but searchable, with a Show-all toggle.\n- Install section reorganized: tightened copy, sudo requirement, new\n  self-update + verify card.\n- ASCII-image card shows the real colored diamond; Share & compare card now\n  has a mini QR + diff demo instead of two dots.\n\nSigned-off-by: Mahesh Diwan <maheshdiwan@proton.me>\n\nGenerated with Codebuff 🤖\nCo-Authored-By: Codebuff <noreply@codebuff.com>",
          "timestamp": "2026-08-15T17:40:47+05:30",
          "tree_id": "99ecc4522f7a492792fcc0fd17f8c353589f0487",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/dc9913f66c45c766b3fa6c622fe71c8a9ac5fe9e"
        },
        "date": 1786796065262,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2782656,
            "range": "± 37180",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2703048,
            "range": "± 68497",
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
          "id": "04fcc828473e5beddeab8475aee5fa096e4a953a",
          "message": "test: cover user-template loading (-t my_template) with unit tests\n\nrenders_user_template_from_config_dir proves the documented workflow now\nworks (config.template != \"default\" loads templates/<name>.tera from the\nconfig dir and interpolates), and missing_user_template_errors_loudly pins\nthe fail-loudly behavior that replaced the old silent default render.\n\nSigned-off-by: Mahesh Diwan <maheshdiwan@proton.me>\n\nGenerated with Codebuff 🤖\nCo-Authored-By: Codebuff <noreply@codebuff.com>",
          "timestamp": "2026-08-15T17:48:40+05:30",
          "tree_id": "4104efb1cab6dec67c1c0dda33ecbd8cafd87bc7",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/04fcc828473e5beddeab8475aee5fa096e4a953a"
        },
        "date": 1786796474105,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2287721,
            "range": "± 48497",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2326286,
            "range": "± 41486",
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
          "id": "293535e1c84217148b5e36e8561afd759a8e4393",
          "message": "feat: honest --benchmark, lazy cache, install.sh flags — installer & speed plan\n\nImplements docs/plan-installer-and-speed.md:\n- bench: report the real parallel path (collect + render) as the headline;\n  per-module timings are now labeled cold/sequential/informational, so the\n  benchmark matches the stopwatch instead of the sequential sum\n- cache: load flexfetch-cache.json lazily on first get/set (zero file IO when\n  no cached module is selected); fix timestamp underflow on future entries;\n  unique pid-suffixed temp file so concurrent processes don't clobber writes\n- install.sh: --help/--dry-run/--version <tag>/--dir/--check/--no-confirm/\n  --quiet flags with zero-prompt defaults; fail-fast dependency pre-check;\n  cleanup trap kills spinner + tmpdir on Ctrl-C; validate --version tags and\n  --dir paths at the boundary (no URL/shell injection)\n- flag-smoke.sh: assert the benchmark reporting contract; write export checks\n  to /tmp so the repo tree stays clean\n- ci: installer smoke job (syntax, shellcheck, flag exit codes, dry-run)\n\nSigned-off-by: Mahesh Diwan <maheshdiwan@proton.me>\n\nGenerated with Codebuff 🤖\nCo-Authored-By: Codebuff <noreply@codebuff.com>",
          "timestamp": "2026-08-16T15:49:49+05:30",
          "tree_id": "9b547f20021d4c29eb6ddf5f6b959d288e9eac44",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/293535e1c84217148b5e36e8561afd759a8e4393"
        },
        "date": 1786875828980,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2566540,
            "range": "± 46806",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2568006,
            "range": "± 27772",
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
          "id": "d4ab66940f5ea96747e9af0ca8800bd7e635a6f0",
          "message": "fix(site): close marquee wrapper, dedupe modules button, fix mobile term overflow, true binary claim\n\n- close the .marquee div so aria-hidden and overflow:hidden stop wrapping the\n  stats/features/modules/install sections (marquee box was 6578px tall)\n- remove the duplicate \"Show all 38 modules\" button (invalid duplicate id;\n  JS bound only the first)\n- narrow-phone term font 9px -> 8px so the full 69-col hero output fits\n  without the 29px horizontal scroll\n- \"1.75 MB\" -> \"1.7 MB\" (minimal binary measures 1.69 MB; matches PRODUCT.md)\n\nSigned-off-by: Mahesh Diwan <maheshdiwan@proton.me>\n\nGenerated with Codebuff 🤖\nCo-Authored-By: Codebuff <noreply@codebuff.com>\n\nSigned-off-by: Mahesh Diwan <diwanmahesh11@gmail.com>",
          "timestamp": "2026-08-16T16:23:21+05:30",
          "tree_id": "3390488a4f50cc1fab126ad49c7df18276c89fa2",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/d4ab66940f5ea96747e9af0ca8800bd7e635a6f0"
        },
        "date": 1786877774038,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2734923,
            "range": "± 98663",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2736228,
            "range": "± 121975",
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
          "id": "a62b1513f903f8404e37a83a63ac80e725b14a44",
          "message": "feat: 12 new fastfetch-parity modules (38→50) + fix CI failures\n\nModules added (all zero-subprocess — read /proc, /sys, DMI, or env):\n- System: datetime (chrono-free exact civil-date math), loadavg, keyboard\n- Software: editor ($VISUAL/$EDITOR), initsystem, version\n- Hardware: bios, board, chassis (DMI + SMBIOS chassis codec),\n  brightness (backlight %), tpm\n- Network: localip (one getifaddrs call, compact primary-IPs view)\n\nRegistered everywhere via MODULE_CATALOG (single source of truth):\ncatalog drives --list-modules, section headers, plain-renderer alignment,\nand template grouping; also added wizard toggles, --demo showcase,\ndefault.tera rows + plain-renderer icons, docs, CHANGELOG, and the site\n(marquee chips, +12 grid cards, all \"38 modules\" claims → 50).\n\nCI fixes (both jobs failed on the previous push):\n- installer: shellcheck exit 1 — removed unused WHITE var, fixed banner\n  printf arg count (3 specifiers vs 2 args), suppressed intentional\n  literal-\\$PATH hint (SC2016)\n- minimal-build: TempConfigDir test helper was tera-gated at usage but\n  not definition, so --no-default-features test builds flagged it as\n  dead code under -D warnings; gated struct + impls\n\nVerified: 192/192 tests pass, clippy/fmt clean (default + no-default-\nfeatures), flag-smoke green, real output shows all 12 modules in both\nTera and plain renderers, site renders 50 modules with no dup ids.\n\nSigned-off-by: Mahesh Diwan <maheshdiwan@proton.me>\n\nGenerated with Codebuff 🤖\nCo-Authored-By: Codebuff <noreply@codebuff.com>\n\nSigned-off-by: Mahesh Diwan <diwanmahesh11@gmail.com>",
          "timestamp": "2026-08-16T17:43:51+05:30",
          "tree_id": "dc994b7b2fe4ba1c5d5403d28be527b57359ad97",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/a62b1513f903f8404e37a83a63ac80e725b14a44"
        },
        "date": 1786882605399,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2636102,
            "range": "± 23468",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2651360,
            "range": "± 35936",
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
          "id": "7ed391301e7a6158d13084c60032983e4c6648d2",
          "message": "fix(ci): shellcheck SC2015 — explicit if for binary backup in install_to\n\nCI's shellcheck (newer than the local 0.11.0) flags the\n`[ -f ... ] && cp ... || true` backup line as SC2015 (\"A && B || C is\nnot if-then-else; C may run when A is true\"). Rewrote as an explicit\n`if` with the same best-effort semantics — never fail the install over\na failed backup.\n\nSigned-off-by: Mahesh Diwan <maheshdiwan@proton.me>\n\nGenerated with Codebuff 🤖\nCo-Authored-By: Codebuff <noreply@codebuff.com>\n\nSigned-off-by: Mahesh Diwan <diwanmahesh11@gmail.com>",
          "timestamp": "2026-08-16T18:25:57+05:30",
          "tree_id": "cec8d93fe54a37a9c8190017da74b687758fcb70",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/7ed391301e7a6158d13084c60032983e4c6648d2"
        },
        "date": 1786885095961,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2119723,
            "range": "± 23206",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2097948,
            "range": "± 15190",
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
          "id": "d45b12e41ec1e8a8f505e97e40e1bd6a2281fe32",
          "message": "fix(ci): installer job — capture exit codes under runner's bash -e\n\nThe \"unknown flag exits 2\" and \"--check\" steps ran the install script\nas a bare command; under GitHub's `bash -e` the step died with the\nscript's exit code before `test $? -eq 2` could run. Wrap the checks in\nset +e / set -e and capture the status explicitly.\n\nSigned-off-by: Mahesh Diwan <maheshdiwan@proton.me>\n\nGenerated with Codebuff 🤖\nCo-Authored-By: Codebuff <noreply@codebuff.com>\n\nSigned-off-by: Mahesh Diwan <diwanmahesh11@gmail.com>",
          "timestamp": "2026-08-16T18:37:45+05:30",
          "tree_id": "7b2da362da9326231959019a2243e5af2f46eb4c",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/d45b12e41ec1e8a8f505e97e40e1bd6a2281fe32"
        },
        "date": 1786885840385,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2774692,
            "range": "± 24807",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2749492,
            "range": "± 23267",
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
          "id": "0d169bf3234c23763344abbe785783e05f4f2e30",
          "message": "fix(ci): cache target dir + timeout on windows job to de-flake OOMs\n\nThe windows job recompiled everything from scratch every run (registry\ncache only, no target cache): 40-60 min of cold compilation on shared\nwindows runners, which repeatedly died with OOM / STATUS_STACK_BUFFER_\nOVERRUN / runner-shutdown — a flake that predates the recent module work\n(the same failure appears on Aug 13-15 runs). Caching `target` keyed on\nCargo.lock + rustc hash cuts rebuilds to minutes, and the 60-min timeout\nbounds a hung run instead of burning an hour.\n\nSigned-off-by: Mahesh Diwan <maheshdiwan@proton.me>\n\nGenerated with Codebuff 🤖\nCo-Authored-By: Codebuff <noreply@codebuff.com>",
          "timestamp": "2026-08-16T20:28:30+05:30",
          "tree_id": "937a299099ba31c19fb2df000599200b532e4a3e",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/0d169bf3234c23763344abbe785783e05f4f2e30"
        },
        "date": 1786892490013,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2644190,
            "range": "± 23985",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2639118,
            "range": "± 25896",
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
          "id": "9085efca40eb84900ae51b167df052b04125e600",
          "message": "fix: bump version to 1.0.1 (matches latest release) + honor --quiet contract\n\nThe v1.0.1 release was cut without a version bump, so the released binary\nself-reported 0.32.0 and install.sh --check could never report \"up to\ndate\". Bump the workspace version (both crates, dep spec, Cargo.lock) to\n1.0.1 and update the site hero badge; --check now converges.\n\nAlso make --quiet honor its documented contract (only errors + the final\n\"installed\" line): banner and step progress bars are suppressed.\n\nAnd add a rustc-only restore-key to the windows cargo cache so a\nCargo.lock change no longer forces a full cold rebuild.\n\nSigned-off-by: Mahesh Diwan <maheshdiwan@proton.me>\n\nGenerated with Codebuff 🤖\nCo-Authored-By: Codebuff <noreply@codebuff.com>",
          "timestamp": "2026-08-16T21:14:39+05:30",
          "tree_id": "86c4547e374ab9532f8fd3029704dac038637b36",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/9085efca40eb84900ae51b167df052b04125e600"
        },
        "date": 1786895253104,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2036935,
            "range": "± 46656",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2016684,
            "range": "± 40274",
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
          "id": "efcafc907e1ecf45a82789045ecf1483d675823b",
          "message": "feat(site): add GSAP ScrollTrigger choreography and creative web enhancements\n\nAdd scroll-driven animations, ambient hero effects, and interactive polish:\n\n- Hero: floating gradient orbs with parallax, scan line, glow ring, dot grid\n- Feature cards: staggered GSAP entrance, spotlight glow, scan line on hover\n- Stats: slide-up reveal with GSAP, hover glow pulse\n- Modules: cursor spotlight on grid rows\n- Install: staggered slide-in cards, step numbers\n- CTA: animated gradient accent line, scale-up heading\n- Footer: fade-in, brand underline animation\n- Cursor: subtle amber trail with mix-blend-mode\n- Marquee: gradient fade edges, speed linked to scroll position\n- Nav: tighter backdrop on scroll\n- GSAP CDN (3.12.5) + ScrollTrigger + scroll.js (10 instances)\n- All animations respect prefers-reduced-motion\n\n🤖 Generated with Codebuff\nCo-Authored-By: Codebuff <noreply@codebuff.com>",
          "timestamp": "2026-08-17T18:44:33+05:30",
          "tree_id": "7d69bc3fe6888cb07cdb2a937b2c677005a510ba",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/efcafc907e1ecf45a82789045ecf1483d675823b"
        },
        "date": 1786972648842,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 1912653,
            "range": "± 42964",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 1932826,
            "range": "± 86447",
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
          "id": "dfbf791a771cef0957369c16cfad2131dc94a3d4",
          "message": "fix(site): resolve GSAP/CSS animation conflict — elements no longer stuck invisible\n\nGSAP from() was setting inline opacity:0 on elements before ScrollTrigger\nfired, which overrode CSS .reveal.in { opacity:1 }. Elements below the\nfold stayed permanently hidden.\n\nFix:\n- Use fromTo() with immediateRender:false so elements stay visible\n  (natural CSS state) until ScrollTrigger fires\n- Add clearProps:\"all\" so inline styles are removed after animation,\n  letting CSS cascade take back control\n- Add .gsap-ready class to GSAP-animated elements so CSS .reveal\n  hiding rule skips them\n- Skip .in class for .gsap-ready elements in IntersectionObserver\n\nVerified: 7/7 cards visible, 4/4 stats visible, 0 inline styles remaining.\n\n🤖 Generated with Codebuff\nCo-Authored-By: Codebuff <noreply@codebuff.com>",
          "timestamp": "2026-08-17T18:57:27+05:30",
          "tree_id": "4ab39ae0480864504348398e000a60a6b2aef669",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/dfbf791a771cef0957369c16cfad2131dc94a3d4"
        },
        "date": 1786973412110,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 1782340,
            "range": "± 55039",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 1798509,
            "range": "± 48595",
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
          "id": "6178a9f2e8e2c5fb5d0f645e1018a022e116db8f",
          "message": "fix(site): address impeccable critique — contrast, touch targets, spacing, typography\n\nCritical fixes:\n- Body text contrast: #a1a1aa → #b4b4bc (8.5:1 ratio, was 6.8:1)\n- Muted text contrast: #8a8a94 → #9a9aa4 (6.5:1 ratio, was 5.5:1)\n- h1 line-height: 1.04 → 1.1 (prevents ascender/descender collision)\n- Touch targets: hamburger 40→44px, copy buttons 31→44px height,\n  btn-sm min-height 40px, all coarse-pointer targets min 44×44px\n\nSignificant fixes:\n- Bento grid gap: 14px → 18px (better breathing room)\n- Module grid gap: 1px → 3px (less visual noise)\n- Install section columns: 0.9fr/1.1fr → 1fr/1.05fr (balanced)\n- CTA padding: 110px → 80px (less disconnected)\n- Footer columns: 1.2fr/2fr → 1fr/1.8fr (better balance)\n\nMinor fixes:\n- Stats section: added \"// by the numbers\" eyebrow label\n- Social proof: rephrased for clarity\n- Marquee font: 12px → 13px (more legible)\n- Terminal font floor: 8px → 9px on small phones\n- Scan line frequency: 4s → 6s (less distracting)\n- Cursor trail: smaller (6px), subtler (0.2 opacity, blur)\n\n🤖 Generated with Codebuff\nCo-Authored-By: Codebuff <noreply@codebuff.com>",
          "timestamp": "2026-08-17T19:20:13+05:30",
          "tree_id": "5f26be18c3843b1e73b6f4c6d91fe47b79b345c1",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/6178a9f2e8e2c5fb5d0f645e1018a022e116db8f"
        },
        "date": 1786974776892,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2221226,
            "range": "± 26036",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2238986,
            "range": "± 22573",
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
          "id": "44a3e2e6db850a5e97d24d548e792071a24f0510",
          "message": "refactor: over-engineering audit cuts (-1029 lines, -4 deps)\n\n- site: drop GSAP/ScrollTrigger + scroll.js for CSS scroll-driven animations;\n  remove duplicate IntersectionObserver reveal engine; delete dead CSS\n  (.plugin*, .secure-grid, .hero-cur)\n- docs: delete unreferenced doc/templates.md and completed plan doc\n- core: delete dead group_sections() + tests, dead DisplayConfig::logo_mode,\n  Error::Lua; derive template placeholders/image-logo names from\n  MODULE_CATALOG (fixes drift); reuse dedup_visible() in render_tera;\n  collapse detect_nerd_font/detect_osc8 into terminal_matches(); extract\n  os_id_of(); drop unreachable migrate_config arm\n- cli: derive wizard module/theme lists from core (fixes drift); delete\n  gen_man.rs example; drop unused clap_mangen/proptest dev-deps; inline\n  one-line wrappers; share live.rs snapshot drain\n- ci: merge perf-gate into bench.yml cached build",
          "timestamp": "2026-08-21T15:40:06+05:30",
          "tree_id": "83e826d008c14333f1ee2e16e6bdab4a2b4a1efa",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/44a3e2e6db850a5e97d24d548e792071a24f0510"
        },
        "date": 1787307210048,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2751872,
            "range": "± 26359",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2762473,
            "range": "± 34976",
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
          "id": "9dd7368cea6edc9a7a3ad889ea9edbfd2d715812",
          "message": "docs: 2026 landscape research — fastfetch/neofetch deep study, feature proposals, size+speed roadmap",
          "timestamp": "2026-08-21T15:56:05+05:30",
          "tree_id": "1b43a17eaebdb9b14fc57527a3b2e996de0383c8",
          "url": "https://github.com/mahesh-diwan/flexfetch/commit/9dd7368cea6edc9a7a3ad889ea9edbfd2d715812"
        },
        "date": 1787308157360,
        "tool": "cargo",
        "benches": [
          {
            "name": "cold_start_minimal",
            "value": 2582668,
            "range": "± 21097",
            "unit": "ns/iter"
          },
          {
            "name": "cold_start_default_pipe",
            "value": 2561306,
            "range": "± 26197",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}