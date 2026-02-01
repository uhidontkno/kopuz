#!/bin/bash
set -e

APP_NAME="rusic"
BUILD_DIR="target/dx/rusic/release/linux/app"
OUTPUT_DIR="target/appimage"
LINUXDEPLOY_URL="https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage"
LINUXDEPLOY_PLUGIN_GTK_URL="https://raw.githubusercontent.com/linuxdeploy/linuxdeploy-plugin-gtk/master/linuxdeploy-plugin-gtk.sh"

echo "Creating AppImage for $APP_NAME..."

mkdir -p "$OUTPUT_DIR"
cd "$OUTPUT_DIR"

if [ ! -f "linuxdeploy-x86_64.AppImage" ]; then
    curl -L -o linuxdeploy-x86_64.AppImage "$LINUXDEPLOY_URL"
    chmod +x linuxdeploy-x86_64.AppImage
fi

if [ ! -f "appimagetool-x86_64.AppImage" ]; then
    curl -L -o appimagetool-x86_64.AppImage "https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage"
    chmod +x appimagetool-x86_64.AppImage
fi

if [ ! -f "linuxdeploy-plugin-gtk.sh" ]; then
    curl -L -o linuxdeploy-plugin-gtk.sh "$LINUXDEPLOY_PLUGIN_GTK_URL"
    chmod +x linuxdeploy-plugin-gtk.sh
fi

if [ ! -d "linuxdeploy-root" ]; then
    ./linuxdeploy-x86_64.AppImage --appimage-extract
    mv squashfs-root linuxdeploy-root
fi
if [ ! -d "appimagetool-root" ]; then
    ./appimagetool-x86_64.AppImage --appimage-extract
    mv squashfs-root appimagetool-root
fi

APPDIR="AppDir"
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin"
mkdir -p "$APPDIR/usr/share/applications"

PROJECT_ROOT="../.."
cp "$PROJECT_ROOT/$BUILD_DIR/$APP_NAME" "$APPDIR/usr/bin/"
cp -r "$PROJECT_ROOT/$BUILD_DIR/assets" "$APPDIR/usr/bin/"

cat > "$APPDIR/usr/share/applications/com.temidaradev.rusic.desktop" <<EOF
[Desktop Entry]
Categories=AudioVideo;Audio;Player;
Comment=A modern music player
Exec=rusic
Icon=rusic
Name=Rusic
StartupWMClass=rusic
Terminal=false
Type=Application
Version=1.0
EOF

ICON_PATH_HICOLOR="$APPDIR/usr/share/icons/hicolor/512x512/apps"
mkdir -p "$ICON_PATH_HICOLOR"
COPY_LOGO_FROM="$PROJECT_ROOT/rusic/assets/logo.png"
if [ ! -f "$COPY_LOGO_FROM" ]; then
    COPY_LOGO_FROM="$PROJECT_ROOT/assets/logo.png"
fi
cp "$COPY_LOGO_FROM" "$ICON_PATH_HICOLOR/rusic.png"
cp "$COPY_LOGO_FROM" "$APPDIR/rusic.png"
ln -sf "rusic.png" "$APPDIR/.DirIcon"

cat > "$APPDIR/AppRun" <<EOF
#!/bin/bash
HERE="\$(dirname "\$(readlink -f "\${0}")")"
export PATH="\${HERE}/usr/bin:\${PATH}"
export LD_LIBRARY_PATH="\${HERE}/usr/lib:\${LD_LIBRARY_PATH}"
export XDG_DATA_DIRS="\${HERE}/usr/share:\${XDG_DATA_DIRS:-/usr/local/share:/usr/share}"
exec "\${HERE}/usr/bin/rusic" "\$@"
EOF
chmod +x "$APPDIR/AppRun"

export NO_STRIP=1
./linuxdeploy-root/usr/bin/linuxdeploy --appdir "$APPDIR" \
    --desktop-file "$APPDIR/usr/share/applications/com.temidaradev.rusic.desktop" \
    --icon-file "$APPDIR/rusic.png" \
    --icon-filename "rusic"

rm -f "$APPDIR"/usr/lib/libwebkit2gtk* "$APPDIR"/usr/lib/libjavascriptcoregtk*
rm -f "$APPDIR"/usr/lib/libgtk-3* "$APPDIR"/usr/lib/libgdk-3*
rm -f "$APPDIR"/usr/lib/libgio-2.0* "$APPDIR"/usr/lib/libglib-2.0*
rm -f "$APPDIR"/usr/lib/libgobject-2.0* "$APPDIR"/usr/lib/libcairo*
rm -f "$APPDIR"/usr/lib/libpango* "$APPDIR"/usr/lib/libsystemd*

export ARCH=x86_64
./appimagetool-root/usr/bin/appimagetool "$APPDIR" Rusic-x86_64.AppImage

echo "Success! AppImage created: $OUTPUT_DIR/Rusic-x86_64.AppImage"
