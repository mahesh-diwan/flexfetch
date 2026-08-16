#!/usr/bin/env bash
# flexfetch installer — interactive, colorful, single-file.
# Only install channel. curl shows real download progress.
#
# Flags (all optional; no flags = same behavior as before):
#   --help              show this help and exit
#   --dry-run           resolve the version + print the plan, write nothing
#   --version <tag>     pin a specific release tag (e.g. v1.2.3) instead of latest
#   --dir <path>        install to exactly <path> (no fallback chain)
#   --check             compare installed vs latest; exit 0=current 1=outdated
#                       2=not installed 3=unknown/network
#   --no-confirm        non-interactive (no first-run demo output)
#   --quiet             only errors and the final "installed" line
set -euo pipefail

REPO="mahesh-diwan/flexfetch"
BIN="flexfetch"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
LOCAL_DIR="${HOME}/.local/bin"
MAX_RETRIES=3
TOTAL_STEPS=5

# ─── Flag state ───────────────────────────────────────────────────────────────
DRY_RUN=0
PINNED_TAG=""
DIR_OVERRIDE=""
DO_CHECK=0
NO_CONFIRM=0
QUIET=0

usage() {
	cat <<EOF
Usage: install.sh [options]

Installs flexfetch from the latest GitHub release (checksum-verified).

Options:
  --help              Show this help and exit
  --dry-run           Resolve the version and print the plan; write nothing
  --version <tag>     Install a specific release tag (e.g. --version v1.2.3)
  --dir <path>        Install to exactly <path> (no /usr/local/bin fallback)
  --check             Compare installed version to latest:
                      exit 0 = current, 1 = outdated, 2 = not installed,
                      3 = could not determine latest
  --no-confirm        Non-interactive (suppress the first-run demo output)
  --quiet             Only errors and the final "installed" line

Environment:
  INSTALL_DIR         Preferred install dir (default: /usr/local/bin)
  GITHUB_TOKEN        Optional, for authenticated GitHub API lookups
EOF
}

# ─── Argument parsing ─────────────────────────────────────────────────────────
while [ $# -gt 0 ]; do
	case "$1" in
	--help | -h)
		usage
		exit 0
		;;
	--dry-run) DRY_RUN=1 ;;
	--version)
		[ $# -ge 2 ] || { echo "error: --version needs a tag argument" >&2; exit 2; }
		[ -n "$2" ] || { echo "error: --version tag cannot be empty" >&2; exit 2; }
		# Guard at the boundary: the tag is interpolated into a download URL and
		# a shell command, so only accept the release convention (v + digits,
		# optionally with dots/dashes/letters for pre-releases) — never slashes,
		# spaces, or URL/shell metacharacters.
		case "$2" in
		*[!a-zA-Z0-9._-]*)
			echo "error: invalid tag '$2' (expected e.g. v1.2.3 or v1.2.3-rc1)" >&2
			exit 2
			;;
		v[0-9]*)
			PINNED_TAG="$2"
			;;
		*)
			echo "error: invalid tag '$2' (must start with v followed by a digit)" >&2
			exit 2
			;;
		esac
		shift
		;;
	--dir)
		[ $# -ge 2 ] || { echo "error: --dir needs a path argument" >&2; exit 2; }
		[ -n "$2" ] || { echo "error: --dir path cannot be empty" >&2; exit 2; }
		DIR_OVERRIDE="$2"
		shift
		;;
	--check) DO_CHECK=1 ;;
	--no-confirm) NO_CONFIRM=1 ;;
	--quiet) QUIET=1 ;;
	--)
		shift
		[ $# -eq 0 ] || { echo "error: unexpected argument: $1" >&2; exit 2; }
		;;
	-*)
		echo "error: unknown option: $1" >&2
		usage >&2
		exit 2
		;;
	*)
		echo "error: unexpected argument: $1" >&2
		usage >&2
		exit 2
		;;
	esac
	shift
done

# ─── Colors (only when stdout is a tty) ───────────────────────────────────────
if [ -t 1 ]; then
	BOLD=$'\033[1m'
	DIM=$'\033[2m'
	RED=$'\033[31m'
	GREEN=$'\033[32m'
	YELLOW=$'\033[33m'
	CYAN=$'\033[36m'
	RESET=$'\033[0m'
	BAR_DONE=$'\033[32m━\033[0m'
	BAR_TODO=$'\033[2m━\033[0m'
else
	BOLD="" DIM="" RED="" GREEN="" YELLOW="" CYAN="" RESET=""
	BAR_DONE="=" BAR_TODO="-"
fi

# ─── UI helpers ───────────────────────────────────────────────────────────────
banner() {
	printf '\n'
	printf '  %s%s.flexfetch%s\n' "$CYAN" "$BOLD" "$RESET"
	printf '  %s━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━ ━%s\n' "$DIM" "$RESET"
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

ok() { [ "$QUIET" -eq 1 ] || printf '\r  %s%s✔%s %s\n' "$GREEN" "$BOLD" "$RESET" "$1"; }
fail() { printf '\r  %s%s✘%s %s\n' "$RED" "$BOLD" "$RESET" "$1"; }
info() { [ "$QUIET" -eq 1 ] || printf '  %s%sℹ%s %s\n' "$DIM" "$CYAN" "$RESET" "$1"; }

# ─── Spinner (for background tasks) ───────────────────────────────────────────
SPIN_PID=""
spin_start() {
	[ "$QUIET" -eq 1 ] && return
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

# ─── Cleanup on any exit (Ctrl-C mid-download must not leak spinner/tmpdir) ──
TMPDIR="" # shadow the env var; we own it below
cleanup() {
	spin_stop
	if [ -n "${TMPDIR:-}" ] && [ -d "$TMPDIR" ]; then
		rm -rf "$TMPDIR"
	fi
}
# INT/TERM: clean up AND abort (a bare `trap cleanup` would resume the script
# after Ctrl-C). EXIT covers normal and error paths.
on_interrupt() {
	cleanup
	exit 130
}
trap cleanup EXIT
trap on_interrupt INT TERM

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

# ─── Dependency pre-check (fail fast, before any network work) ───────────────
MISSING=""
command -v tar >/dev/null 2>&1 || MISSING="${MISSING} tar"
if ! command -v curl >/dev/null 2>&1 && ! command -v wget >/dev/null 2>&1; then
	MISSING="${MISSING} curl-or-wget"
fi
if ! command -v sha256sum >/dev/null 2>&1 && ! command -v shasum >/dev/null 2>&1; then
	MISSING="${MISSING} sha256sum-or-shasum"
fi
if [ -n "$MISSING" ]; then
	fail "missing required tools:$MISSING"
	info "install them first, then re-run this script"
	exit 1
fi

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

# Resolve which tag to use: pinned flag wins, else latest.
resolve_tag() {
	if [ -n "$PINNED_TAG" ]; then
		echo "$PINNED_TAG"
	else
		fetch_tag
	fi
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

# installed_version: what `flexfetch --version` reports, or empty if absent.
installed_version() {
	if command -v "$BIN" >/dev/null 2>&1; then
		"$BIN" --version 2>/dev/null | head -1 | awk '{print $2}' || echo ""
	else
		echo ""
	fi
}

# ─── --check: compare installed vs latest, exit-code contract ────────────────
if [ "$DO_CHECK" -eq 1 ]; then
	TAG=$(resolve_tag)
	if [ -z "$TAG" ]; then
		echo "flexfetch: could not determine latest release" >&2
		exit 3
	fi
	CURRENT=$(installed_version)
	if [ -z "$CURRENT" ]; then
		echo "flexfetch: not installed (latest: $TAG)"
		exit 2
	elif [ "v$CURRENT" = "$TAG" ]; then
		echo "flexfetch: up to date ($CURRENT)"
		exit 0
	else
		echo "flexfetch: outdated (installed v$CURRENT, latest $TAG)"
		exit 1
	fi
fi

banner

# ─── --dry-run: resolve + print plan, write nothing ──────────────────────────
if [ "$DRY_RUN" -eq 1 ]; then
	TAG=$(resolve_tag)
	if [ -z "$TAG" ]; then
		echo ""
		fail "could not determine latest release"
		info "check your network connection or try again later"
		exit 3
	fi
	URL="https://github.com/$REPO/releases/download/$TAG/flexfetch-${OS_ALIAS}-${ARCH_ALIAS}.tar.gz"
	if [ -n "$DIR_OVERRIDE" ]; then
		TARGET="$DIR_OVERRIDE/$BIN"
	else
		TARGET="$INSTALL_DIR/$BIN"
	fi
	printf '\n'
	ok "would install flexfetch $TAG"
	info "download: $URL"
	info "target:   $TARGET"
	info "checksum: $URL.sha256 (verified after download)"
	info "no files were written (--dry-run)"
	printf '\n'
	exit 0
fi

# ─── Main flow ────────────────────────────────────────────────────────────────

# Step 1: Resolve version
progress 1 $TOTAL_STEPS "Resolving latest version..."
TAG=$(resolve_tag)
if [ -z "$TAG" ]; then
	echo ""
	fail "could not determine latest release"
	info "check your network connection or try again later"
	exit 1
fi
ok "latest version: $TAG"

# Check current version
CURRENT=$(installed_version)

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
	exit 1
fi
ok "downloaded $(du -h "$TMPDIR/$BIN.tar.gz" | cut -f1)"

# Step 3: Validate
progress 3 $TOTAL_STEPS "Validating archive..."
if ! tar -tzf "$TMPDIR/$BIN.tar.gz" >/dev/null 2>&1; then
	fail "downloaded file is not a valid gzip archive"
	info "the release may not include a binary for $ARCH_ALIAS"
	exit 1
fi
ok "archive valid"

# Step 4: Verify checksum + extract
progress 4 $TOTAL_STEPS "Verifying checksum..."
verify_checksum "$TMPDIR" "$CHECKSUM_URL" || exit 1

if ! tar xzf "$TMPDIR/$BIN.tar.gz" -C "$TMPDIR" 2>/dev/null; then
	fail "failed to extract archive"
	exit 1
fi

if [ ! -f "$TMPDIR/$BIN" ]; then
	fail "binary not found in archive"
	exit 1
fi

chmod +x "$TMPDIR/$BIN"

# Step 5: Install
progress 5 $TOTAL_STEPS "Installing $BIN..."
TARGET=""
install_to() {
	local dir="$1"
	mkdir -p "$dir" 2>/dev/null || return 1
	[ -f "$dir/$BIN" ] && cp "$dir/$BIN" "$dir/$BIN.bak.$(date +%s)" 2>/dev/null || true
	mv "$TMPDIR/$BIN" "$dir/$BIN" 2>/dev/null || return 1
	TARGET="$dir/$BIN"
}

if [ -n "$DIR_OVERRIDE" ]; then
	# --dir: exact location, no fallback chain.
	if ! install_to "$DIR_OVERRIDE"; then
		fail "cannot write to $DIR_OVERRIDE"
		info "try: --dir ~/mybin"
		exit 1
	fi
else
	if ! install_to "$INSTALL_DIR" && ! install_to "$LOCAL_DIR"; then
		fail "cannot write to $INSTALL_DIR or $LOCAL_DIR"
		info "try: INSTALL_DIR=~/mybin sh install.sh"
		exit 1
	fi
fi
ok "installed to $TARGET"

# PATH hint
if [ "$TARGET" = "$LOCAL_DIR/$BIN" ] && ! echo ":$PATH:" | grep -q ":${LOCAL_DIR}:"; then
	echo ""
	info "add $LOCAL_DIR to your PATH:"
	# shellcheck disable=SC2016
	# The literal \$PATH above is intentional: this is a copy-paste hint, and
	# the user's shell must expand $PATH when they run it.
	printf '    %sexport PATH="\$PATH:%s"%s\n' "$YELLOW" "$LOCAL_DIR" "$RESET"
fi

# Done banner
# Guarded: a failing `wc` (permissions, race) must not abort the install
# under `set -e` — just skip the size figure.
if SIZE_KB=$(wc -c <"$TARGET" 2>/dev/null); then
	SIZE_KB=$((SIZE_KB / 1024))
else
	SIZE_KB=0
fi
echo ""
printf '  %s%s%s %s%s%s installed %s(%s KiB)%s\n' \
	"$GREEN" "$BOLD" "$BIN" "$CYAN" "$TAG" "$RESET" "$DIM" "$SIZE_KB" "$RESET"

# First-run payoff (suppressed by --no-confirm / --quiet / non-tty)
if [ -t 1 ] && [ "$NO_CONFIRM" -eq 0 ]; then
	echo ""
	"$TARGET" --minimal 2>/dev/null || true
	echo ""
	printf '  %sCustomize:%s  %s%s --wizard%s  (interactive config)\n' "$DIM" "$RESET" "$BOLD" "$BIN" "$RESET"
	printf '  %sShowcase:%s   %s%s --demo%s   (every module)\n' "$DIM" "$RESET" "$BOLD" "$BIN" "$RESET"
	printf '  %sLive dash:%s  %s%s --live%s   (real-time TUI)\n' "$DIM" "$RESET" "$BOLD" "$BIN" "$RESET"
	echo ""
fi
