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

# Detect arch
ARCH=$(uname -m)
case "$ARCH" in
x86_64)  ARCH_ALIAS="amd64" ;;
aarch64) ARCH_ALIAS="aarch64" ;;
armv7l)  ARCH_ALIAS="armv7" ;;
*)
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
            | sort -t. -k1,1n -k2,2n -k3,3n | tail -1 || true)
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

URL="https://github.com/$REPO/releases/download/$TAG/flexfetch-linux-${ARCH_ALIAS}.tar.gz"

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

# Install (try target dir, fall back to ~/.local/bin)
TARGET=""
if mkdir -p "$INSTALL_DIR" 2>/dev/null && mv "$TMPDIR/$BIN" "$INSTALL_DIR/$BIN" 2>/dev/null; then
    TARGET="$INSTALL_DIR/$BIN"
elif mkdir -p "$LOCAL_DIR" && mv "$TMPDIR/$BIN" "$LOCAL_DIR/$BIN" 2>/dev/null; then
    TARGET="$LOCAL_DIR/$BIN"
    if ! echo ":$PATH:" | grep -q ":${LOCAL_DIR}:"; then
        echo "  Hint: add $LOCAL_DIR to PATH"
        echo "    export PATH=\"\$PATH:$LOCAL_DIR\""
    fi
else
    echo "Error: cannot write to $INSTALL_DIR or $LOCAL_DIR"
    echo "  Try: INSTALL_DIR=~/mybin sh install.sh"
    exit 1
fi

echo "Done. $BIN $TAG installed to $TARGET"
