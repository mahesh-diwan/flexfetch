# Modules

All modules detect from your system automatically. `--list-modules` shows the
full set; `-m "a:b:c"` or `modules = [...]` in config selects them.

## System

| Module            | Shows                                                        |
| ----------------- | ------------------------------------------------------------ |
| `os`              | Distro name, ID, version, logo                               |
| `host`            | Machine / product name                                       |
| `kernel`          | Kernel release                                               |
| `uptime`          | How long the system has been up                              |
| `locale`          | Language / locale settings                                   |
| `shell`           | User's shell                                                 |
| `terminal`        | Terminal emulator                                            |
| `de`              | Desktop environment                                          |
| `wm`              | Window manager                                               |

## Hardware

| Module            | Shows                                                        |
| ----------------- | ------------------------------------------------------------ |
| `cpu`             | Model, cores, frequency                                      |
| `cpucache`        | L1d/L1i/L2/L3 cache sizes                                    |
| `cpuusage`        | Current CPU usage %                                          |
| `gpu`             | Graphics devices                                             |
| `memory`          | Used / total RAM + percentage                                |
| `swap`            | Swap usage                                                   |
| `disk`            | Mount usage (progress bar option)                            |
| `battery`         | Charge percentage + status                                   |
| `temperature`     | Sensor temperatures                                          |
| `display`         | Display / resolution info                                    |
| `resolution`      | Screen resolution                                            |
| `bluetooth`       | Paired devices                                               |
| `media`           | Now-playing via MPRIS (or `nowplaying-cli` on macOS)         |

## Network

| Module            | Shows                                                        |
| ----------------- | ------------------------------------------------------------ |
| `network`         | Interface addresses + speeds                                 |
| `wifi`            | Wireless SSID / signal                                       |
| `publicip`        | Public IP (via curl)                                         |
| `dns`             | Configured DNS servers                                       |

## Software

| Module            | Shows                                                        |
| ----------------- | ------------------------------------------------------------ |
| `packages`        | Installed packages (apt/rpm/pacman/flatpak/snap breakdown)   |
| `processes`       | Process count                                                |
| `colors`          | Terminal color palette                                       |

## Context & extras

| Module            | Shows                                                        |
| ----------------- | ------------------------------------------------------------ |
| `health`          | 0–100 system health score (disk/swap/load/battery)           |
| `git`             | Branch, ahead/behind, dirty count (via `--smart`)            |
| `project`         | Project type from manifests (via `--smart`)                  |
| `context`         | Container / venv / SSH session (via `--smart`)               |
| `custom`          | Your `[custom]` shell-command modules                        |

## Layout directives

| Name        | Purpose                    |
| ----------- | -------------------------- |
| `title`     | Gradient page title        |
| `separator` | Horizontal rule            |

These are template-only and are skipped by the plain renderer.
