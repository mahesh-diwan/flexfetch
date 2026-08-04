#!/bin/sh
# Phase 8.1C — verify that a release binary was built from the claimed source.
#
# Usage:
#   ./scripts/verify-repro.sh v0.18.0
#
# Rebuilds the tag with the same toolchain + feature set the release pipeline
# uses, then compares the resulting binary's sha256 to the checksum published on
# the GitHub release. Byte-identical output = reproducible build.
#
# IMPORTANT: the release builds the Linux amd64 artifact as static musl via
# cargo-zigbuild (x86_64-unknown-linux-musl). A native glibc build is NOT
# byte-identical to it, so this script uses zigbuild + the musl target by
# default, exactly like the pipeline. If cargo-zigbuild/zig are not installed,
# it falls back to a native build and prints a warning (the comparison will
# almost certainly differ — glibc vs musl) so the user knows the caveat.
#
# NOTE: the release pipeline also runs UPX compression on Linux artifacts; this
# script compares the *uncompressed* binary hash, which is what the .sha256
# files in dist/ would reflect only if UPX output is what's checksummed. If the
# published checksum is of the UPX-compressed file, run with VERIFY_COMPRESSED=1
# (requires upx installed) to compare apples to apples.
#
# Env overrides:
#   REPO                repo to fetch the published checksum from (default mahesh-diwan/flexfetch)
#   TARGET              rust target triple (default x86_64-unknown-linux-musl)
#   RELEASE_FEATURES    feature set to build with (default live,image-logos,completions,parallel)
#   VERIFY_COMPRESSED   set to 1 to UPX-compress before hashing
set -eu

REPO="${REPO:-mahesh-diwan/flexfetch}"
TAG="${1:-}"
TARGET="${TARGET:-x86_64-unknown-linux-musl}"
RELEASE_FEATURES="${RELEASE_FEATURES:-live,image-logos,completions,parallel}"
ARTIFACT="flexfetch-linux-amd64.tar.gz"
SHA_URL="https://github.com/${REPO}/releases/download/${TAG}/${ARTIFACT}.sha256"

if [ -z "$TAG" ]; then
    echo "usage: $0 <tag>   e.g. $0 v0.18.0"
    exit 2
fi

if ! command -v cargo >/dev/null 2>&1; then
    echo "error: cargo not found" >&2
    exit 1
fi
if ! command -v curl >/dev/null 2>&1 && ! command -v wget >/dev/null 2>&1; then
    echo "error: need curl or wget to fetch the published checksum" >&2
    exit 1
fi

echo "==> building ${TAG}: ${TARGET} with --no-default-features --features ${RELEASE_FEATURES}"
git fetch --tags origin 2>/dev/null || true
git checkout --detach "${TAG}" 2>/dev/null || {
    echo "error: tag ${TAG} not found" >&2
    exit 1
}
trap 'git checkout - 2>/dev/null || true' EXIT INT TERM

BUILD_CMD="cargo build --release --locked -p flexfetch-cli --no-default-features --features ${RELEASE_FEATURES}"
if command -v cargo-zigbuild >/dev/null 2>&1; then
    # Same path as release.yml (musl static via zig cross linker).
    rustup target add "${TARGET}" >/dev/null 2>&1 || true
    eval "cargo zigbuild --release --target ${TARGET} --locked -p flexfetch-cli --no-default-features --features ${RELEASE_FEATURES}"
    BIN="target/${TARGET}/release/flexfetch"
else
    # Fallback: native build (NOT byte-identical to the musl artifact).
    echo "warning: cargo-zigbuild not found — building natively; the hash will"
    echo "         almost certainly differ from the musl release artifact."
    echo "         Install it: cargo install cargo-zigbuild"
    eval "${BUILD_CMD}"
    BIN="target/release/flexfetch"
fi
if [ "${VERIFY_COMPRESSED:-0}" = "1" ]; then
    command -v upx >/dev/null 2>&1 || { echo "error: VERIFY_COMPRESSED=1 needs upx installed" >&2; exit 1; }
    TMPBIN="$(mktemp)"
    cp "$BIN" "$TMPBIN"
    upx --best --lzma "$TMPBIN" >/dev/null
    BIN="$TMPBIN"
    trap 'rm -f "$TMPBIN"; git checkout - 2>/dev/null || true' EXIT INT TERM
fi

LOCAL_SHA=$(sha256sum "$BIN" | awk '{print $1}')
echo "local hash:  ${LOCAL_SHA}"

REMOTE_SHA=""
if command -v curl >/dev/null 2>&1; then
    REMOTE_SHA=$(curl -sfL "$SHA_URL" | awk '{print $1}' || true)
else
    REMOTE_SHA=$(wget -qO- "$SHA_URL" | awk '{print $1}' || true)
fi
if [ -z "$REMOTE_SHA" ]; then
    echo "error: could not fetch published checksum from ${SHA_URL}" >&2
    echo "  (the release may predate checksums, or the tag has no linux-amd64 artifact)" >&2
    exit 1
fi
echo "release hash: ${REMOTE_SHA}"

if [ "$LOCAL_SHA" = "$REMOTE_SHA" ]; then
    echo "OK: reproducible — binary matches the published checksum"
    exit 0
else
    echo "MISMATCH: local build differs from the published artifact" >&2
    echo "  possible causes: non-reproducible build, CI environment difference," >&2
    echo "  or a tampered artifact. Report as a security issue (see SECURITY.md)." >&2
    exit 1
fi
