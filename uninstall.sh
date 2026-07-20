#!/bin/sh
set -eu

BIN="flexfetch"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
LOCAL_DIR="${HOME}/.local/bin"

FOUND=""
for dir in "$INSTALL_DIR" "$LOCAL_DIR"; do
	if [ -f "$dir/$BIN" ]; then
		FOUND="$dir/$BIN"
		break
	fi
done

if [ -z "$FOUND" ]; then
	FOUND=$(command -v "$BIN" 2>/dev/null || true)
fi

if [ -z "$FOUND" ]; then
	echo "$BIN not found on system."
	exit 0
fi

echo "Found $BIN at: $FOUND"
printf "Remove $BIN? [Y/n] "
read -r CONFIRM
case "$CONFIRM" in
n | N | no | NO)
	echo "Aborted."
	exit 0
	;;
*) ;;
esac

if rm -f "$FOUND"; then
	echo "Done. $BIN removed from $FOUND"
else
	echo "Error: could not remove $FOUND (permission?)"
	echo "  Try: sudo rm $FOUND"
	exit 1
fi
