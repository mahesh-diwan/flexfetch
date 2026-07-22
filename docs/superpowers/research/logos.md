# Logo Systems Research for flexfetch

## Current State

flexfetch has 7 ASCII logos in `flexfetch-core/src/logo.rs`: arch, debian, ubuntu, fedora, nixos, macos, generic. Logos are static `&[&str]` padded to 22 lines. Detection matches `/etc/os-release` ID with a simple `match` statement. No color support.

---

## Tool Analysis

### fastfetch (500+ logos)

**Storage:** Each logo is an `FFlogo` struct with fields: `lines` (ASCII text), `names[9]` (aliases for matching), `colors[9]` (ANSI color codes), `colorKeys`, `colorTitle`, `type` (normal/small bitmask). ASCII art stored in `.inc` files per letter (a.inc, b.inc, ...) via `FASTFETCH_DATATEXT_LOGO_*` macros.

**Detection:** Reads `/etc/os-release`, extracts `ID` field, does case-insensitive name matching against logo `names[]` array. Falls back to OS name. Also supports `ID_LIKE` fallback.

**Sizes:** Supports normal and small variants via `FF_LOGO_LINE_TYPE_SMALL_BIT`. Small logos have names ending `_small`. User can request `--logo-type small`.

**Color system:** `$1`-`$9` placeholders in ASCII art replaced with ANSI escape codes from `colors[]` array. Supports carry-color across lines (color persists until reset). Up to 9 colors per logo.

**Logo count:** 500+ built-in logos covering mainstream and obscure distros.

### neofetch (~150 logos)

**Storage:** Single monolithic bash script. Each distro has a case in `get_distro_ascii()` with inline heredoc ASCII art using `${c1}`-`${c6}` color placeholders. Colors set via `set_colors` function before each logo.

**Detection:** Complex chain: WSL detection → `lsb_release` → `/etc/*-release` glob → `awk` parsing. Distro name matched via case statement with glob patterns.

**Sizes:** Some distros have `_small` and `_old` variants. Selected via `--ascii_distro` flag.

**Color system:** `${c1}`-`${c6}` placeholders replaced with ANSI codes. Colors defined per-logo via `set_colors N N N N N N`.

**Logo count:** ~150 distros. Includes legacy/niche systems (IRIX, Haiku, MINIX, etc).

### pfetch (minimal, ~30 logos)

**Storage:** Single POSIX shell script. Each logo inline in `get_ascii()` function using heredoc with `${c1}`-`${c4}` color codes.

**Detection:** Reads `/etc/os-release` ID, matches via case statement with glob patterns.

**Sizes:** Single size per logo.

**Color system:** `${c1}`-`${c4}` ANSI color codes embedded directly.

**Logo count:** ~30 distros. Generic penguin fallback.

---

## Distro Detection Strategy

### Priority chain (recommended)

1. **`/etc/os-release`** — Standard (freedesktop.org/systemd). Present on all modern distros. Fields: `ID`, `ID_LIKE`, `PRETTY_NAME`, `VERSION_ID`.
2. **`lsb_release -si`** — Fallback if os-release missing. Requires `lsb-release` package (not always installed).
3. **`/etc/lsb-release`** — Debian/Ubuntu fallback.
4. **Distro-specific files:** `/etc/redhat-release`, `/etc/debian_version`, `/etc/SuSE-release`, `/etc/alpine-release`, `/etc/gentoo-release`, `/etc/arch-release`.
5. **`hostnamectl`** — systemd systems. Outputs `Operating System:` line.
6. **`/etc/issue`** — Last resort (login banner). Unreliable formatting.

### Key fields from `/etc/os-release`

- `ID` — Lowercase identifier (e.g., `ubuntu`, `fedora`, `arch`)
- `ID_LIKE` — Space-separated list of parent distros (e.g., `ID_LIKE="rhel fedora"`)
- `PRETTY_NAME` — Human-readable (e.g., `Ubuntu 22.04.4 LTS`)
- `VERSION_ID` — Version number (e.g., `22.04`)

### Rust implementation approach

```rust
// Read /etc/os-release, parse KEY=VALUE lines
// Match ID first, then ID_LIKE as fallback
fn detect_distro() -> DistroId {
    let os_release = std::fs::read_to_string("/etc/os-release")
        .or_else(|_| std::fs::read_to_string("/usr/lib/os-release"));
    // Parse, extract ID, match against logo registry
}
```

---

## Distro Catalog (25 distros)

| #   | Distro              | os-release ID         | ID_LIKE       | Logo Name   | Primary Color    |
| --- | ------------------- | --------------------- | ------------- | ----------- | ---------------- |
| 1   | Ubuntu              | `ubuntu`              | `debian`      | ubuntu      | `#E95420` orange |
| 2   | Fedora              | `fedora`              | —             | fedora      | `#51A2DA` blue   |
| 3   | Arch Linux          | `arch`                | —             | arch        | `#1793D1` cyan   |
| 4   | Debian              | `debian`              | —             | debian      | `#A80030` red    |
| 5   | Linux Mint          | `linuxmint`           | `ubuntu`      | mint        | `#87BE33` green  |
| 6   | Pop!_OS             | `pop`                 | `ubuntu`      | pop         | `#48B9C7` teal   |
| 7   | Manjaro             | `manjaro`             | `arch`        | manjaro     | `#33BF82` green  |
| 8   | CentOS              | `centos`              | `rhel fedora` | centos      | `#932279` purple |
| 9   | Rocky Linux         | `rocky`               | `rhel fedora` | rocky       | `#10B981` green  |
| 10  | AlmaLinux           | `almalinux`           | `rhel fedora` | alma        | `#00BA88` green  |
| 11  | openSUSE Leap       | `opensuse-leap`       | `suse`        | opensuse    | `#73BA25` green  |
| 12  | openSUSE Tumbleweed | `opensuse-tumbleweed` | `suse`        | opensuse-tw | `#35B97A` green  |
| 13  | Gentoo              | `gentoo`              | —             | gentoo      | `#54487A` purple |
| 14  | Alpine              | `alpine`              | —             | alpine      | `#0D597F` blue   |
| 15  | Void Linux          | `void`                | —             | void        | `#478061` green  |
| 16  | NixOS               | `nixos`               | —             | nixos       | `#7EBAE4` blue   |
| 17  | Zorin OS            | `zorin`               | `ubuntu`      | zorin       | `#00AAEE` blue   |
| 18  | Elementary OS       | `elementary`          | `ubuntu`      | elementary  | `#64BAEF` blue   |
| 19  | Linux Lite          | `linuxlite`           | `ubuntu`      | linuxlite   | `#F6A924` gold   |
| 20  | MX Linux            | `mx`                  | `debian`      | mx          | `#61A6CF` blue   |
| 21  | Raspberry Pi OS     | `raspbian`            | `debian`      | raspbian    | `#C51C4A` red    |
| 22  | Kali Linux          | `kali`                | `debian`      | kali        | `#557C94` blue   |
| 23  | EndeavourOS         | `endeavouros`         | `arch`        | endeavouros | `#6B4EB0` purple |
| 24  | Garuda Linux        | `garuda`              | `arch`        | garuda      | `#F45D00` orange |
| 25  | CachyOS             | `cachyos`             | `arch`        | cachyos     | `#00AEEF` blue   |

**Additional IDs to map (arch derivatives):**

- `arcolinux` → arch logo
- `artix` → arch logo
- `biglinux` → generic
- `blackarch` → arch variant
- `blendos` → generic
- `bornagain` → generic

---

## Colored ASCII Art Approach

### Three approaches compared

| Approach                      | How it works                               | Pros                                | Cons                                               |
| ----------------------------- | ------------------------------------------ | ----------------------------------- | -------------------------------------------------- |
| **Color slots (neofetch)**    | `${c1}`-`${c6}` placeholders, 6 colors max | Simple, compact                     | Limited colors, requires placeholder insertion     |
| **Color indices (fastfetch)** | `$1`-`$9` placeholders, 9 colors max       | More colors, per-logo color schemes | Requires placeholder insertion                     |
| **Inline ANSI**               | Raw escape codes embedded in art           | No parsing needed                   | Ugly source, harder to edit, no user customization |

### Recommendation: Color slots (6 colors)

**Why:** Matches neofetch's proven approach. 6 colors sufficient for all distro logos. Simple to implement in Rust.

**Implementation:**

```rust
struct Logo {
    lines: &'static [&'static str],  // ASCII art with $1-$6 placeholders
    colors: &'static [&'static str], // ANSI color codes per slot
}
```

**Color replacement:**

```rust
fn render_logo(logo: &Logo) -> Vec<String> {
    logo.lines.iter().map(|line| {
        let mut result = line.to_string();
        for (i, color) in logo.colors.iter().enumerate() {
            result = result.replace(&format!("${}", i + 1), color);
        }
        result
    }).collect()
}
```

**ANSI color codes (16-color terminal):**

- `\x1b[31m` = red, `\x1b[32m` = green, `\x1b[33m` = yellow
- `\x1b[34m` = blue, `\x1b[35m` = magenta, `\x1b[36m` = cyan
- `\x1b[0m` = reset

---

## Logo Sizing Approach

### Options

1. **Fixed height (current):** All logos padded to N lines. Simple, predictable layout. Most tools do this.
2. **Dynamic height:** Logos natural height, info text positioned accordingly. Better aesthetics, more complex layout.
3. **Multiple sizes (fastfetch):** Normal + small variants per distro. User chooses.

### Recommendation: Fixed height + optional small variant

- Default: Fixed 22 lines (current approach). Simple layout calculation.
- Add `_small` suffix for compact logos (Arch, Gentoo, Alpine, etc).
- Future: Allow user `--logo-size small|normal` flag.

---

## Key Design Decisions

1. **Storage:** Static arrays like current approach. Add `colors` field to Logo struct.
2. **Detection:** Parse `/etc/os-release` ID, fallback chain for legacy systems.
3. **Color model:** 6-slot color system with `$1`-`$6` placeholders.
4. **Logo count:** Start with 25 popular distros, expand later.
5. **Sizing:** Fixed 22 lines with optional small variants.
6. **Fallback:** Generic logo when no match found.
