# Modules

All modules detect from your system automatically. `--list-modules` shows the
full set; `-m "a:b:c"` or `modules = [...]` in config selects them.

## System

| Module   | Shows                           | Static? |
| -------- | ------------------------------- | ------- |
| `os`     | Distro name, ID, version, logo  | Yes     |
| `host`   | Machine / product name          | Yes     |
| `kernel` | Kernel release                  | Yes     |
| `uptime` | How long the system has been up | No      |
| `locale` | Language / locale settings      | Yes     |
| `datetime` | Current date and time        | No      |
| `loadavg`  | 1/5/15-minute load average   | No      |
| `keyboard` | Keyboard layout (XKB)        | Yes     |

## Hardware

| Module        | Shows                             | Static? |
| ------------- | --------------------------------- | ------- |
| `bios`        | BIOS vendor + version (DMI)       | Yes     |
| `board`       | Motherboard vendor + name (DMI)   | Yes     |
| `chassis`     | Chassis type (DMI)                | Yes     |
| `brightness`  | Backlight brightness %            | No      |
| `tpm`         | TPM presence + version            | Yes     |
| `cpu`         | Model, cores, frequency           | Yes     |
| `cpucache`    | L1d/L1i/L2/L3 cache sizes         | Yes     |
| `cpuusage`    | Current CPU usage %               | No      |
| `gpu`         | Graphics devices                  | Yes     |
| `memory`      | Used / total RAM + percentage     | No      |
| `swap`        | Swap usage                        | No      |
| `disk`        | Mount usage (progress bar option) | No      |
| `battery`     | Charge percentage + status        | No      |
| `temperature` | Sensor temperatures               | Yes     |
| `display`     | Display / resolution info         | Yes     |
| `resolution`  | Screen resolution                 | Yes     |
| `colors`      | Terminal color palette            | Yes     |

## Network

| Module      | Shows                                                | Static? |
| ----------- | ---------------------------------------------------- | ------- |
| `network`   | Interface addresses + speeds                         | No      |
| `localip`   | Local interface addresses                           | Yes     |
| `wifi`      | Wireless SSID / signal                               | Yes     |
| `publicip`  | Public IP (via curl)                                 | Yes     |
| `bluetooth` | Paired devices                                       | Yes     |
| `media`     | Now-playing via MPRIS (or `nowplaying-cli` on macOS) | No      |
| `dns`       | Configured DNS servers                               | Yes     |

## Software

| Module      | Shows                                                      | Static? |
| ----------- | ---------------------------------------------------------- | ------- |
| `packages`  | Installed packages (apt/rpm/pacman/flatpak/snap breakdown) | Yes     |
| `shell`     | User's shell                                               | Yes     |
| `editor`    | Default editor (`$VISUAL`/`$EDITOR`)                       | Yes     |
| `initsystem`| Init system (systemd/OpenRC/…)                             | Yes     |
| `version`   | flexfetch version                                          | Yes     |
| `terminal`  | Terminal emulator                                          | Yes     |
| `de`        | Desktop environment                                        | Yes     |
| `wm`        | Window manager                                             | Yes     |
| `processes` | Process count                                              | No      |
| `custom`    | Your `[custom]` shell-command modules                      | No      |

## Context & extras

These modules are activated with `--smart`:

| Module      | Shows                                                                   | Static? |
| ----------- | ----------------------------------------------------------------------- | ------- |
| `health`    | 0–100 system health score (disk/swap/load/battery)                      | Yes     |
| `git`       | Branch, ahead/behind, dirty count                                       | Yes     |
| `project`   | Project type from manifests (`Cargo.toml`, `package.json`, `go.mod`, …) | Yes     |
| `context`   | Container / venv / SSH session                                          | Yes     |
| `wallpaper` | Current wallpaper path                                                  | Yes     |
| `weather`   | Current weather (via API)                                               | Yes     |
| `container` | Container/distrobox detection                                           | Yes     |
| `fsdeep`    | Filesystem depth info                                                   | Yes     |

## Layout directives

| Name        | Purpose             |
| ----------- | ------------------- |
| `title`     | Gradient page title |
| `separator` | Horizontal rule     |

These are template-only and are skipped by the plain renderer.

## Static vs dynamic

**Static modules** are collected once per session and reused in watch/live
mode (their values don't change mid-session). **Dynamic modules** are
re-collected every tick in watch/live mode.

Dynamic: `uptime`, `datetime`, `loadavg`, `cpuusage`, `memory`, `swap`,
`disk`, `battery`, `brightness`, `network`, `media`, `processes`, `custom`.
