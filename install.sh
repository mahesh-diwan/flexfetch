#!/bin/sh
set -eu

REPO="mahesh-diwan/flexfetch"
BIN="flexfetch"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
LOCAL_DIR="${HOME}/.local/bin"

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
if command -v curl >/dev/null 2>&1; then
	TAG=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p')
elif command -v wget >/dev/null 2>&1; then
	TAG=$(wget -q -O - "https://api.github.com/repos/$REPO/releases/latest" | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p')
else
	echo "Error: need curl or wget"
	exit 1
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

if command -v curl >/dev/null 2>&1; then
	curl -#L "$URL" -o "$TMPDIR/$BIN.tar.gz"
else
	wget "$URL" -O "$TMPDIR/$BIN.tar.gz"
fi

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
