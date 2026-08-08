# Features

## Live Dashboard

Real-time system monitor — CPU and memory gauges with sparklines, top processes,
and per-interface network throughput:

```bash
flexfetch --live
```

### Controls

| Key       | Action              |
| --------- | ------------------- |
| `q`/`Esc` | Quit                |
| `Space`   | Refresh immediately |

### Data sources

Linux `/proc` + `/sys` (CPU ticks, per-process `stat`/`statm`, interface byte
counters). The memory gauge reuses the existing `memory` collector.

### Hot reload

Editing `~/.config/flexfetch/config.toml` hot-reloads custom modules on the
next tick (mtime-based, no external watcher).

Gated behind the `live` feature (default on).

## Config Wizard

Interactive 4-step wizard (ratatui): module checklist, theme picker with live
preview, layout (box style + frame), then writes
`~/.config/flexfetch/config.toml`:

```bash
flexfetch --wizard
```

### Controls

| Key       | Action     |
| --------- | ---------- |
| `↑`/`↓`   | Move       |
| `Space`   | Toggle     |
| `a`       | Select all |
| `Enter`   | Next step  |
| `q`/`Esc` | Quit       |

## Remote Fetch over SSH

```bash
flexfetch --ssh server1 --ssh server2        # parallel, per-host headers
```

Runs `flexfetch --format json` on each remote host (parallel scoped threads)
and renders it locally. If a host lacks flexfetch, the current binary is scp'd
to `/tmp/flexfetch-<pid>` and run from there.

## Watch Mode

Refresh output periodically (useful for dashboards), hot-reloading the config
file when it changes:

```bash
flexfetch --watch                          # every 2 seconds
flexfetch --watch --watch-interval 5       # every 5 seconds
```

Press `Ctrl+C` to stop.

Static modules (os/host/kernel/…) are collected once and reused. Dynamic
modules (cpuusage/memory/disk/network/battery/…) are re-collected every tick.

## Smart Fetch

Context-relevant info based on the current directory:

```bash
flexfetch --smart
```

| Module    | Shows                                                                   |
| --------- | ----------------------------------------------------------------------- |
| `git`     | Branch, ahead/behind vs upstream, dirty file count                      |
| `project` | Project type from manifests (`Cargo.toml`, `package.json`, `go.mod`, …) |
| `context` | Container, Python virtualenv, SSH session                               |

Empty results are omitted.

## Health Score

A 0–100 system health score from disk usage, swap, load, and battery:

```bash
flexfetch --health
```

```
├─ Health: 92/100 (Excellent) — disk 87%
```

Score starts at 100 and deducts for disk >90%, swap >50%, load >1/core, or
battery <80%. It's also a regular module — add `health` to your module list.

## Demo Mode

Showcase mode — every module + every visual feature, for screenshots and
social previews:

```bash
flexfetch --demo
```

## Diff Mode

Compare two systems side-by-side:

```bash
flexfetch --diff local server1
flexfetch --diff local export.json
```

Each target can be `local`, `host@remote`, or a path to a flexfetch JSON
export file.

## Environment Doctor

Validate terminal, color, config, and collectors:

```bash
flexfetch --doctor
```

## Self-update

Check the latest GitHub release and re-run the install script:

```bash
flexfetch --update
```
