#!/bin/sh
set -eu

REPO="mahesh-diwan/flexfetch"
BIN="flexfetch"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
LOCAL_DIR="${HOME}/.local/bin"
MAX_RETRIES=3

# Pac-Man animation frames (fallback to dots if Unicode fails)
PACMAN_FRAMES=(
    "🌕 ⬤ ⬤ ⬤ ⬤ ⬤"
    "🌗  ⬤ ⬤ ⬤ ⬤ ⬤"
    "🌘   ⬤ ⬤ ⬤ ⬤"
    "🌑    ⬤ ⬤ ⬤"
    "🌑     ⬤ ⬤"
    "🌑      ⬤"
    "🌘       "
    "🌗        "
)

# Check if terminal supports Unicode
if [ -t 1 ] && printf '\u2B24' 2>/dev/null | grep -q ''; then
    USE_UNICODE=1
else
    USE_UNICODE=0
    PACMAN_FRAMES=("." ".." "..." "...." "....." "...." "..." "..")
fi

# Pac-Man animation function
pacman_animate() {
    local msg="$1"
    local i=0
    local frames=${#PACMAN_FRAMES[@]}

    # Only animate if stdout is a terminal
    if [ ! -t 1 ]; then
        echo "$msg"
        return
    fi

    # Hide cursor
    printf '\033[?25l'

    while :; do
        printf '\r\033[K%s %s' "${PACMAN_FRAMES[i]}" "$msg"
        i=$(( (i + 1) % frames ))
        sleep 0.15
    done
}

# Stop pacman animation
pacman_stop() {
    local pid=$1
    kill "$pid" 2>/dev/null || true
    wait "$pid" 2>/dev/null || true
    # Show cursor and clear line
    printf '\033[?25h\r\033[K'
}

# Cleanup on interrupt
cleanup() {
    [ -n "${PACMAN_PID:-}" ] && pacman_stop "$PACMAN_PID"
    [ -n "${TMPDIR:-}" ] && rm -rf "$TMPDIR"
    exit 1
}
trap cleanup INT TERM

# Detect OS and arch (artifact names: flexfetch-<os>-<arch>.tar.gz)
OS_ALIAS="linux"
case "$(uname -s)" in
Linux)  OS_ALIAS="linux" ;;
Darwin) OS_ALIAS="macos" ;;
*)
    echo "Error: unsupported OS: $(uname -s) (only Linux and macOS are supported)"
    exit 1
    ;;
esac

ARCH=$(uname -m)
case "$OS_ALIAS:$ARCH" in
linux:x86_64)  ARCH_ALIAS="amd64" ;;
linux:aarch64) ARCH_ALIAS="aarch64" ;;
linux:armv7l)  ARCH_ALIAS="armv7" ;;
linux:*)
    echo "Error: unsupported architecture: $ARCH"
    exit 1
    ;;
macos:x86_64)  ARCH_ALIAS="x86_64" ;;
macos:arm64)   ARCH_ALIAS="aarch64" ;;
macos:*)
    echo "Error: unsupported architecture: $ARCH"
    exit 1
    ;;
esac

# Fetch latest release tag (3-tier: API → API → git ls-remote)
fetch_tag() {
    local tag=""

    # Tier 1: Authenticated GitHub API
    if [ -n "${GITHUB_TOKEN:-}" ]; then
        tag=$(curl -sfL -H "Authorization: token $GITHUB_TOKEN" \
            "https://api.github.com/repos/$REPO/releases/latest" 2>/dev/null \
            | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/' || true)
    fi

    # Tier 2: Unauthenticated GitHub API
    if [ -z "$tag" ]; then
        tag=$(curl -sfL "https://api.github.com/repos/$REPO/releases/latest" 2>/dev/null \
            | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/' || true)
    fi

    # Tier 3: git ls-remote (bypasses API rate limits)
    if [ -z "$tag" ] && command -v git >/dev/null 2>&1; then
        tag=$(git ls-remote --tags "https://github.com/$REPO.git" 2>/dev/null \
            | sed 's/.*refs\/tags\///' | grep '^v[0-9]' \
            | sort -V | tail -1 || true)
    fi

    echo "$tag"
}

TAG=$(fetch_tag)

if [ -z "$TAG" ]; then
    echo "Error: could not determine latest release"
    echo "  Check your network connection or try again later"
    exit 1
fi

# Check current version
CURRENT=""
if command -v "$BIN" >/dev/null 2>&1; then
    CURRENT=$("$BIN" --version 2>/dev/null | head -1 | awk '{print $2}' || echo "")
fi

if [ -n "$CURRENT" ] && [ "v$CURRENT" = "$TAG" ]; then
    echo "$BIN already at latest version ($CURRENT)"
    exit 0
fi

if [ -n "$CURRENT" ]; then
    echo "Upgrading $BIN v$CURRENT -> $TAG..."
else
    echo "Installing $BIN $TAG..."
fi

URL="https://github.com/$REPO/releases/download/$TAG/flexfetch-${OS_ALIAS}-${ARCH_ALIAS}.tar.gz"
CHECKSUM_URL="$URL.sha256"

TMPDIR=$(mktemp -d)

# Download with retry
download() {
    local url="$1"
    local dest="$2"
    local attempt=1

    while [ $attempt -le $MAX_RETRIES ]; do
        if command -v curl >/dev/null 2>&1; then
            curl -sfL "$url" -o "$dest" 2>/dev/null && return 0
        elif command -v wget >/dev/null 2>&1; then
            wget -q "$url" -O "$dest" 2>/dev/null && return 0
        else
            echo "Error: neither curl nor wget found"
            return 1
        fi
        attempt=$((attempt + 1))
        [ $attempt -le $MAX_RETRIES ] && sleep 1
    done
    return 1
}

# Start Pac-Man animation in background
pacman_animate "Downloading..." &
PACMAN_PID=$!

if ! download "$URL" "$TMPDIR/$BIN.tar.gz"; then
    pacman_stop "$PACMAN_PID"
    echo "Error: download failed after $MAX_RETRIES attempts"
    echo "  URL: $URL"
    echo "  Try: curl -sfL $URL -o $BIN.tar.gz"
    exit 1
fi

pacman_stop "$PACMAN_PID"

# Validate download (check if it's a valid gzip file)
if ! file "$TMPDIR/$BIN.tar.gz" | grep -qi gzip; then
    echo "Error: downloaded file is not a valid gzip archive"
    echo "  The release may not include a binary for $ARCH_ALIAS"
    exit 1
fi

# Verify checksum (fail closed on mismatch; skip only if no sha tool exists)
verify_checksum() {
    if command -v sha256sum >/dev/null 2>&1; then
        SUM_TOOL="sha256sum"
    elif command -v shasum >/dev/null 2>&1; then
        SUM_TOOL="shasum -a 256"
    else
        echo "Warning: no sha256 tool found — skipping checksum verification"
        return 0
    fi

    if command -v curl >/dev/null 2>&1; then
        if ! curl -sfL "$CHECKSUM_URL" -o "$TMPDIR/$BIN.sha256" 2>/dev/null; then
            echo "Warning: could not fetch checksum ($CHECKSUM_URL) — skipping verification"
            return 0
        fi
    elif command -v wget >/dev/null 2>&1; then
        if ! wget -q "$CHECKSUM_URL" -O "$TMPDIR/$BIN.sha256" 2>/dev/null; then
            echo "Warning: could not fetch checksum ($CHECKSUM_URL) — skipping verification"
            return 0
        fi
    else
        echo "Warning: neither curl nor wget found — skipping checksum verification"
        return 0
    fi

    EXPECTED=$(awk '{print $1}' "$TMPDIR/$BIN.sha256")
    ACTUAL=$($SUM_TOOL "$TMPDIR/$BIN.tar.gz" | awk '{print $1}')

    if [ -n "$EXPECTED" ] && [ "$EXPECTED" = "$ACTUAL" ]; then
        echo "Checksum verified (sha256)."
    elif [ -z "$EXPECTED" ]; then
        echo "Error: checksum file is empty or unreadable"
        echo "  Refusing to install. Re-run to retry, or download manually:"
        echo "    $URL"
        rm -rf "$TMPDIR"
        exit 1
    else
        echo "Error: checksum mismatch!"
        echo "  Expected: $EXPECTED"
        echo "  Actual:   $ACTUAL"
        echo "  Refusing to install. Re-run to retry, or download manually:"
        echo "    $URL"
        rm -rf "$TMPDIR"
        exit 1
    fi
}
verify_checksum

# Extract
if ! tar xzf "$TMPDIR/$BIN.tar.gz" -C "$TMPDIR" 2>/dev/null; then
    echo "Error: failed to extract archive"
    exit 1
fi

if [ ! -f "$TMPDIR/$BIN" ]; then
    echo "Error: binary not found in archive"
    exit 1
fi

chmod +x "$TMPDIR/$BIN"

# Backup an existing binary before overwriting (idempotent updates)
backup_existing() {
    local dest="$1"
    if [ -f "$dest" ]; then
        cp "$dest" "$dest.bak.$(date +%s)" 2>/dev/null || true
    fi
}

# Install (try target dir, fall back to ~/.local/bin)
TARGET=""
if mkdir -p "$INSTALL_DIR" 2>/dev/null; then
    backup_existing "$INSTALL_DIR/$BIN"
    if mv "$TMPDIR/$BIN" "$INSTALL_DIR/$BIN" 2>/dev/null; then
        TARGET="$INSTALL_DIR/$BIN"
    fi
fi
if [ -z "$TARGET" ] && mkdir -p "$LOCAL_DIR" 2>/dev/null; then
    backup_existing "$LOCAL_DIR/$BIN"
    if mv "$TMPDIR/$BIN" "$LOCAL_DIR/$BIN" 2>/dev/null; then
        TARGET="$LOCAL_DIR/$BIN"
    fi
fi
if [ -z "$TARGET" ]; then
    echo "Error: cannot write to $INSTALL_DIR or $LOCAL_DIR"
    echo "  Try: INSTALL_DIR=~/mybin sh install.sh"
    exit 1
fi

# If we fell back to ~/.local/bin, remind the user to add it to PATH
if [ "$TARGET" = "$LOCAL_DIR/$BIN" ] && ! echo ":$PATH:" | grep -q ":${LOCAL_DIR}:"; then
    echo "  Hint: add $LOCAL_DIR to PATH"
    echo "    export PATH=\"\$PATH:$LOCAL_DIR\""
fi

# Clean up the temp dir on the happy path too (the INT/TERM trap only fires on
# signals, and cleanup() exits 1 so it must not run on a successful install).
rm -rf "$TMPDIR"

echo "Done. $BIN $TAG installed to $TARGET"

# Phase 8.8: first-run payoff — show a live fetch immediately when the install
# happens in an interactive terminal (curl | sh always has stdout piped, so this
# only triggers for real ttys).
if [ -t 1 ]; then
    echo ""
    "$TARGET" --minimal 2>/dev/null || true
    echo ""
    echo "Customize with: $BIN --wizard   (interactive config)"
    echo "Showcase every module: $BIN --demo"
fi
