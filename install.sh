#!/bin/sh
set -eu

REPO="mahesh-diwan/flexfetch"
BIN="flexfetch"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

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

# Download
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

if command -v curl >/dev/null 2>&1; then
	curl -sL "$URL" -o "$TMPDIR/$BIN.tar.gz"
else
	wget -q "$URL" -O "$TMPDIR/$BIN.tar.gz"
fi

# Extract
tar xzf "$TMPDIR/$BIN.tar.gz" -C "$TMPDIR"
chmod +x "$TMPDIR/$BIN"

# Install
if command -v sudo >/dev/null 2>&1; then
	sudo mv "$TMPDIR/$BIN" "$INSTALL_DIR/$BIN"
else
	mv "$TMPDIR/$BIN" "$INSTALL_DIR/$BIN"
fi

echo "Done. $BIN $TAG installed to $INSTALL_DIR/$BIN"
