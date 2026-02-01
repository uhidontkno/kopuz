#!/bin/bash
set -e

BINARY_NAME="Rusic-x86_64.AppImage"
INSTALL_DIR="$HOME/.local/share/applications"
ICON_DIR="$HOME/.local/share/icons/hicolor/512x512/apps"
APP_DIR="$HOME/Applications"

if [ ! -f "$BINARY_NAME" ]; then
    echo "Error: $BINARY_NAME not found."
    exit 1
fi

mkdir -p "$APP_DIR" "$INSTALL_DIR" "$ICON_DIR"

cp "$BINARY_NAME" "$APP_DIR/"
chmod +x "$APP_DIR/$BINARY_NAME"

./"$BINARY_NAME" --appimage-extract "rusic.png" > /dev/null 2>&1 || true

if [ -f "squashfs-root/rusic.png" ]; then
    mv "squashfs-root/rusic.png" "$ICON_DIR/rusic.png"
    rm -rf squashfs-root
fi

cat > "$INSTALL_DIR/rusic.desktop" <<EOF
[Desktop Entry]
Name=Rusic
Comment=Modern Music Player
Exec=$APP_DIR/$BINARY_NAME
Icon=rusic
Type=Application
Categories=AudioVideo;Audio;Player;
Terminal=false
StartupWMClass=rusic
EOF

echo "Rusic has been installed to your application menu."
