# Features

## Live Dashboard

Real-time system monitor — CPU and memory gauges with 60-sample sparklines, top
processes by CPU, and per-interface network throughput:

```bash
flexfetch --live
```

- `q` / `Esc` — quit
- `Space` — refresh immediately (rates and CPU% are computed from actual elapsed
  time, so a manual refresh is always accurate)
- Editing `~/.config/flexfetch/config.toml` hot-reloads custom modules on the
  next tick (mtime-based, no external watcher)

Data sources are Linux `/proc` + `/sys` (CPU ticks, per-process `stat`/`statm`,
interface byte counters); the memory gauge reuses the existing `memory`
collector. Gated behind the `live` feature (default on).

## Config Wizard

Interactive 4-step wizard (ratatui): module checklist, theme picker with live
preview, layout (box style + frame), then writes
`~/.config/flexfetch/config.toml`:

```bash
flexfetch --wizard
```

- `↑`/`↓` move · `space` toggle · `a` select all · `enter` next · `q`/`Esc` quit

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

## Smart Fetch

Context-relevant info based on the current directory:

```bash
flexfetch --smart
```

| Module    | Shows                                                                  |
| --------- | ---------------------------------------------------------------------- |
| `git`     | Branch, ahead/behind vs upstream, dirty file count                     |
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
