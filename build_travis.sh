#!/bin/sh

set -x
set -e

if [ "$GTK" = latest -o "$GTK" = "3.96" ]; then
	BUNDLE="gtk-3.96.0-1"
	FEATURES=gio/v2_44
fi

if [ -n "$BUNDLE" ]; then
	WD="$PWD"
	cd "$HOME"
	curl -LO "https://github.com/sfanxiang/gtk-bootstrap/releases/download/$BUNDLE/deps.txz"
	tar xf deps.txz
	cd "$WD"
	export PKG_CONFIG_PATH="$HOME/local/lib/pkgconfig:$HOME/local/share/pkgconfig"
fi

PKG_CONFIG_ALLOW_CROSS=1 cargo check $OTHER_TARGET --features "$FEATURES" --jobs 1 "$@"
