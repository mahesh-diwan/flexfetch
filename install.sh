#!/usr/bin/env bash
# flexfetch installer — interactive, colorful, single-file.
# Only install channel. curl shows real download progress.
set -euo pipefail

REPO="mahesh-diwan/flexfetch"
BIN="flexfetch"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
LOCAL_DIR="${HOME}/.local/bin"
MAX_RETRIES=3
TOTAL_STEPS=5

# ─── Colors (only when stdout is a tty) ───────────────────────────────────────
if [ -t 1 ]; then
	BOLD=$'\033[1m'
	DIM=$'\033[2m'
	RED=$'\033[31m'
	GREEN=$'\033[32m'
	YELLOW=$'\033[33m'
	CYAN=$'\033[36m'
	WHITE=$'\033[37m'
	RESET=$'\033[0m'
	BAR_DONE=$'\033[32m━\033[0m'
	BAR_TODO=$'\033[2m━\033[0m'
else
	BOLD="" DIM="" RED="" GREEN="" YELLOW="" CYAN="" WHITE="" RESET=""
	BAR_DONE="=" BAR_TODO="-"
fi

# ─── UI helpers ────────────────────────────────────────────────────────────────
banner() {
	printf '\n'
	printf '  %s%s.flexfetch%s\n' "$CYAN" "$BOLD" "$RESET"
	printf '  %s%s━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━%s\n' "$DIM" "$RESET"
	printf '\n'
}

progress() {
	local step=$1 total=$2 label=$3
	local done_count=$((step * 30 / total))
	local todo_count=$((30 - done_count))
	local bar=""
	local i=0
	while [ $i -lt $done_count ]; do
		bar="${bar}${BAR_DONE}"
		i=$((i + 1))
	done
	i=0
	while [ $i -lt $todo_count ]; do
		bar="${bar}${BAR_TODO}"
		i=$((i + 1))
	done
	printf '\r  %s %s%s/%s %s%s' "$bar" "$DIM" "$step" "$total" "$label" "$RESET"
}

ok() { printf '\r  %s%s✔%s %s\n' "$GREEN" "$BOLD" "$RESET" "$1"; }
fail() { printf '\r  %s%s✘%s %s\n' "$RED" "$BOLD" "$RESET" "$1"; }
info() { printf '  %s%sℹ%s %s\n' "$DIM" "$CYAN" "$RESET" "$1"; }

# ─── Spinner (for background tasks) ───────────────────────────────────────────
SPIN_PID=""
spin_start() {
	local msg="$1"
	local frames=('⠋' '⠙' '⠹' '⠸' '⠼' '⠴' '⠦' '⠧' '⠇' '⠏')
	printf '%s' "${CYAN}${msg} ${RESET}"
	(
		i=0
		while :; do
			printf '\r%b' "${CYAN}${msg} ${frames[$((i % 10))]}${RESET}"
			i=$((i + 1))
			sleep 0.08
		done
	) &
	SPIN_PID=$!
	disown "$SPIN_PID" 2>/dev/null || true
}

spin_stop() {
	if [ -n "${SPIN_PID:-}" ]; then
		kill "$SPIN_PID" 2>/dev/null || true
		wait "$SPIN_PID" 2>/dev/null || true
	fi
	SPIN_PID=""
}

# ─── Detect OS & arch ─────────────────────────────────────────────────────────
OS_ALIAS="linux"
case "$(uname -s)" in
Linux) OS_ALIAS="linux" ;;
Darwin) OS_ALIAS="macos" ;;
*)
	fail "unsupported OS: $(uname -s) (only Linux and macOS)"
	exit 1
	;;
esac

ARCH=$(uname -m)
case "$OS_ALIAS:$ARCH" in
linux:x86_64) ARCH_ALIAS="amd64" ;;
linux:aarch64) ARCH_ALIAS="aarch64" ;;
linux:armv7l) ARCH_ALIAS="armv7" ;;
linux:*)
	fail "unsupported architecture: $ARCH"
	exit 1
	;;
macos:x86_64) ARCH_ALIAS="x86_64" ;;
macos:arm64) ARCH_ALIAS="aarch64" ;;
macos:*)
	fail "unsupported architecture: $ARCH"
	exit 1
	;;
esac

# ─── Fetch latest release tag (3-tier) ────────────────────────────────────────
fetch_tag() {
	local tag=""

	# Tier 1: Authenticated GitHub API
	if [ -n "${GITHUB_TOKEN:-}" ]; then
		tag=$(curl -sfL -H "Authorization: token $GITHUB_TOKEN" \
			"https://api.github.com/repos/$REPO/releases/latest" 2>/dev/null |
			grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/' || true)
	fi

	# Tier 2: Unauthenticated GitHub API
	if [ -z "$tag" ]; then
		tag=$(curl -sfL "https://api.github.com/repos/$REPO/releases/latest" 2>/dev/null |
			grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/' || true)
	fi

	# Tier 3: git ls-remote (bypasses API rate limits)
	if [ -z "$tag" ] && command -v git >/dev/null 2>&1; then
		tag=$(git ls-remote --tags "https://github.com/$REPO.git" 2>/dev/null |
			sed 's/.*refs\/tags\///' | grep '^v[0-9]' |
			sort -t. -k1.2n -k2n -k3n | tail -1 || true)
	fi

	echo "$tag"
}

# ─── Download with retry ──────────────────────────────────────────────────────
download() {
	local url="$1" dest="$2"
	local attempt=1

	while [ $attempt -le $MAX_RETRIES ]; do
		if command -v curl >/dev/null 2>&1; then
			curl -fL --progress-bar "$url" -o "$dest" && return 0
		elif command -v wget >/dev/null 2>&1; then
			wget "$url" -O "$dest" 2>&1 && return 0
		else
			info "neither curl nor wget found"
			return 1
		fi
		attempt=$((attempt + 1))
		[ $attempt -le $MAX_RETRIES ] && sleep 1
	done
	return 1
}

# ─── Verify checksum ──────────────────────────────────────────────────────────
verify_checksum() {
	local tmpdir="$1" url="$2"
	local sum_tool=""

	if command -v sha256sum >/dev/null 2>&1; then
		sum_tool="sha256sum"
	elif command -v shasum >/dev/null 2>&1; then
		sum_tool="shasum -a 256"
	else
		fail "no sha256 tool found — refusing to install without verification"
		return 1
	fi

	if command -v curl >/dev/null 2>&1; then
		curl -sfL "$url" -o "$tmpdir/$BIN.sha256" 2>/dev/null || {
			fail "could not fetch checksum — refusing to install without verification"
			echo "    $url"
			return 1
		}
	elif command -v wget >/dev/null 2>&1; then
		wget -q "$url" -O "$tmpdir/$BIN.sha256" 2>/dev/null || {
			fail "could not fetch checksum — refusing to install without verification"
			echo "    $url"
			return 1
		}
	else
		fail "no download tool — refusing to install without verification"
		return 1
	fi

	local expected actual
	expected=$(awk '{print $1}' "$tmpdir/$BIN.sha256")
	# Guarded so a failing hash tool (unreadable archive, disk error) hits the
	# clean fail path below instead of a bare `set -e` abort under pipefail.
	if ! actual=$($sum_tool "$tmpdir/$BIN.tar.gz" | awk '{print $1}'); then
		fail "could not hash the downloaded archive"
		return 1
	fi

	if [ -n "$expected" ] && [ "$expected" = "$actual" ]; then
		ok "checksum verified (sha256)"
	elif [ -z "$expected" ]; then
		fail "checksum file is empty or unreadable"
		echo "    $url"
		return 1
	else
		fail "checksum mismatch!"
		echo "    expected: $expected"
		echo "    actual:   $actual"
		return 1
	fi
}

# ─── Main ──────────────────────────────────────────────────────────────────────
banner

# Step 1: Resolve version
progress 1 $TOTAL_STEPS "Resolving latest version..."
TAG=$(fetch_tag)
if [ -z "$TAG" ]; then
	echo ""
	fail "could not determine latest release"
	info "check your network connection or try again later"
	exit 1
fi
ok "latest version: $TAG"

# Check current version
CURRENT=""
if command -v "$BIN" >/dev/null 2>&1; then
	CURRENT=$("$BIN" --version 2>/dev/null | head -1 | awk '{print $2}' || echo "")
fi

if [ -n "$CURRENT" ] && [ "v$CURRENT" = "$TAG" ]; then
	echo ""
	info "$BIN already at latest version ($CURRENT)"
	exit 0
fi

# Step 2: Download
progress 2 $TOTAL_STEPS "Downloading $BIN $TAG..."
URL="https://github.com/$REPO/releases/download/$TAG/flexfetch-${OS_ALIAS}-${ARCH_ALIAS}.tar.gz"
CHECKSUM_URL="$URL.sha256"
TMPDIR=$(mktemp -d)

if ! download "$URL" "$TMPDIR/$BIN.tar.gz"; then
	echo ""
	fail "download failed after $MAX_RETRIES attempts"
	info "URL: $URL"
	info "try: curl -sfL $URL -o $BIN.tar.gz"
	rm -rf "$TMPDIR"
	exit 1
fi
ok "downloaded $(du -h "$TMPDIR/$BIN.tar.gz" | cut -f1)"

# Step 3: Validate
progress 3 $TOTAL_STEPS "Validating archive..."
if ! tar -tzf "$TMPDIR/$BIN.tar.gz" >/dev/null 2>&1; then
	fail "downloaded file is not a valid gzip archive"
	info "the release may not include a binary for $ARCH_ALIAS"
	rm -rf "$TMPDIR"
	exit 1
fi
ok "archive valid"

# Step 4: Verify checksum + extract
progress 4 $TOTAL_STEPS "Verifying checksum..."
verify_checksum "$TMPDIR" "$CHECKSUM_URL" || {
	rm -rf "$TMPDIR"
	exit 1
}

if ! tar xzf "$TMPDIR/$BIN.tar.gz" -C "$TMPDIR" 2>/dev/null; then
	fail "failed to extract archive"
	rm -rf "$TMPDIR"
	exit 1
fi

if [ ! -f "$TMPDIR/$BIN" ]; then
	fail "binary not found in archive"
	rm -rf "$TMPDIR"
	exit 1
fi

chmod +x "$TMPDIR/$BIN"

# Step 5: Install
progress 5 $TOTAL_STEPS "Installing $BIN..."
TARGET=""
if mkdir -p "$INSTALL_DIR" 2>/dev/null; then
	[ -f "$INSTALL_DIR/$BIN" ] && cp "$INSTALL_DIR/$BIN" "$INSTALL_DIR/$BIN.bak.$(date +%s)" 2>/dev/null || true
	if mv "$TMPDIR/$BIN" "$INSTALL_DIR/$BIN" 2>/dev/null; then
		TARGET="$INSTALL_DIR/$BIN"
	fi
fi
if [ -z "$TARGET" ] && mkdir -p "$LOCAL_DIR" 2>/dev/null; then
	[ -f "$LOCAL_DIR/$BIN" ] && cp "$LOCAL_DIR/$BIN" "$LOCAL_DIR/$BIN.bak.$(date +%s)" 2>/dev/null || true
	if mv "$TMPDIR/$BIN" "$LOCAL_DIR/$BIN" 2>/dev/null; then
		TARGET="$LOCAL_DIR/$BIN"
	fi
fi
if [ -z "$TARGET" ]; then
	fail "cannot write to $INSTALL_DIR or $LOCAL_DIR"
	info "try: INSTALL_DIR=~/mybin sh install.sh"
	rm -rf "$TMPDIR"
	exit 1
fi
ok "installed to $TARGET"

# PATH hint
if [ "$TARGET" = "$LOCAL_DIR/$BIN" ] && ! echo ":$PATH:" | grep -q ":${LOCAL_DIR}:"; then
	echo ""
	info "add $LOCAL_DIR to your PATH:"
	printf '    %sexport PATH="\$PATH:%s"%s\n' "$YELLOW" "$LOCAL_DIR" "$RESET"
fi

# Clean up
rm -rf "$TMPDIR"

# Done banner
SIZE_KB=$(($(wc -c <"$TARGET") / 1024))
echo ""
printf '  %s%s%s %s%s%s installed %s(%s KiB)%s\n' \
	"$GREEN" "$BOLD" "$BIN" "$CYAN" "$TAG" "$RESET" "$DIM" "$SIZE_KB" "$RESET"

# First-run payoff
if [ -t 1 ]; then
	echo ""
	"$TARGET" --minimal 2>/dev/null || true
	echo ""
	printf '  %sCustomize:%s  %s%s --wizard%s  (interactive config)\n' "$DIM" "$RESET" "$BOLD" "$BIN" "$RESET"
	printf '  %sShowcase:%s   %s%s --demo%s   (every module)\n' "$DIM" "$RESET" "$BOLD" "$BIN" "$RESET"
	printf '  %sLive dash:%s  %s%s --live%s   (real-time TUI)\n' "$DIM" "$RESET" "$BOLD" "$BIN" "$RESET"
	echo ""
fi
