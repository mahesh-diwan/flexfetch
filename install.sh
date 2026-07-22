#!/bin/sh
set -eu

REPO="mahesh-diwan/flexfetch"
BIN="flexfetch"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
LOCAL_DIR="${HOME}/.local/bin"

# Pac-Man animation frames
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

# Pac-Man animation function
pacman_animate() {
    local msg="$1"
    local i=0
    local frames=${#PACMAN_FRAMES[@]}
    
    # Hide cursor
    printf '\033[?25l'
    
    while :; do
        # Clear line and print frame
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

# Detect arch
ARCH=$(uname -m)
case "$ARCH" in
x86_64) ARCH_ALIAS="amd64" ;;
aarch64) ARCH_ALIAS="aarch64" ;;
*)
	echo "Unsupported architecture: $ARCH"
	exit 1
	;;
esac

# Fetch latest release tag
TAG=""
if [ -n "${GITHUB_TOKEN:-}" ]; then
	# Authenticated API call (5000 req/hr)
	TAG=$(curl -s -H "Authorization: token $GITHUB_TOKEN" "https://api.github.com/repos/$REPO/releases/latest" | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p' || true)
fi

# Unauthenticated API
if [ -z "$TAG" ]; then
	TAG=$(curl -sf "https://api.github.com/repos/$REPO/releases/latest" 2>/dev/null | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p' || true)
fi

# Fallback: git ls-remote (bypasses API rate limits)
if [ -z "$TAG" ] && command -v git >/dev/null 2>&1; then
	TAG=$(git ls-remote --tags "https://github.com/$REPO.git" 2>/dev/null | sed 's/.*refs\/tags\///' | grep '^v[0-9]' | sort -t. -k1,1n -k2,2n -k3,3n | tail -1)
fi

if [ -z "$TAG" ]; then
	echo "Error: could not determine latest release"
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

echo "Downloading $URL ..."

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

# Start Pac-Man animation in background
pacman_animate "Downloading..." &
PACMAN_PID=$!

# Download with progress suppressed (pacman shows activity)
if command -v curl >/dev/null 2>&1; then
	curl -sL "$URL" -o "$TMPDIR/$BIN.tar.gz"
else
	wget -q "$URL" -O "$TMPDIR/$BIN.tar.gz"
fi

# Stop Pac-Man animation
pacman_stop "$PACMAN_PID"

# Extract
tar xzf "$TMPDIR/$BIN.tar.gz" -C "$TMPDIR"
chmod +x "$TMPDIR/$BIN"

# Install (no sudo — try target dir, fall back to ~/.local/bin)
TARGET=""
if mkdir -p "$INSTALL_DIR" 2>/dev/null && mv "$TMPDIR/$BIN" "$INSTALL_DIR/$BIN" 2>/dev/null; then
	TARGET="$INSTALL_DIR/$BIN"
elif mkdir -p "$LOCAL_DIR" && mv "$TMPDIR/$BIN" "$LOCAL_DIR/$BIN" 2>/dev/null; then
	TARGET="$LOCAL_DIR/$BIN"
	! echo ":$PATH:" | grep -q ":${LOCAL_DIR}:" && echo "  Hint: add $LOCAL_DIR to PATH (export PATH=\"\$PATH:$LOCAL_DIR\")"
else
	echo "Error: cannot write to $INSTALL_DIR or $LOCAL_DIR"
	echo "  Try: INSTALL_DIR=~/mybin sh install.sh"
	exit 1
fi

echo "Done. $BIN $TAG installed to $TARGET"
