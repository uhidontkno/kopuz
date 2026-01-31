# Rusic

Rusic is a modern, lightweight music player application built with Rust and the Dioxus framework. It provides a clean and responsive interface for managing and enjoying your local music collection.

![Rusic Interface](https://github.com/user-attachments/assets/8366b1ea-021f-4631-a97b-5ed6e5bf1562)

## Overview

Rusic allows you to scan your local directories for audio files, automatically organizing them into a browsable library. You can navigate by artists, albums, or explore your custom playlists. The application is built for performance and desktop integration, utilizing the power of Rust (well most of it gets demolished by webview .d).

## Features

- **Library Management**: Automatically scans your music folder to populate your library with artist and album metadata.
- **Playback Control**: Full suite of media controls including play, pause, skip, volume, and seeking.
- **Fullscreen Player**: An immersive mode that focuses on album artwork and playback details.
- **Theming**: Includes dynamic theming support to customize the visual appearance.
- **Native Integration**: Integrates with system media controls and "Now Playing" displays (only macOS for now).

## Installation

### NixOS / Nix

```bash
nix run github:temidaradev/rusic
```

Or add to your flake:
```nix
inputs.rusic.url = "github:temidaradev/rusic";
# Then use: inputs.rusic.packages.${system}.default
```

### Ubuntu / Debian

```bash
# Install dependencies
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libasound2-dev libxdo-dev libssl-dev pkg-config nodejs npm

# Install Dioxus CLI
cargo install dioxus-cli

# Clone and install
git clone https://github.com/temidaradev/rusic
cd rusic
npm install
make install
```

### Arch Linux

```bash
# Install dependencies
sudo pacman -S webkit2gtk-4.1 gtk3 alsa-lib xdotool openssl pkg-config nodejs npm rust

# Install Dioxus CLI
cargo install dioxus-cli

# Clone and install
git clone https://github.com/temidaradev/rusic
cd rusic
npm install
make install
```

### Fedora

```bash
# Install dependencies
sudo dnf install webkit2gtk4.1-devel gtk3-devel alsa-lib-devel libxdo-devel openssl-devel pkg-config nodejs npm rust cargo

# Install Dioxus CLI
cargo install dioxus-cli

# Clone and install
git clone https://github.com/temidaradev/rusic
cd rusic
npm install
make install
```

After installation, you can:
- Run `rusic` from terminal (if `~/.local/bin` is in your PATH)
- Find "Rusic" in your app launcher

To uninstall: `make uninstall`

## Development

```bash
# Clone
git clone https://github.com/temidaradev/rusic
cd rusic

# NixOS: Enter dev shell
nix develop

# Install npm deps
npm install

# Run in dev mode
dx serve
```

## Cache

Rusic stores its local database, configuration files, and cached album artwork in your system's cache directory (typically `~/.cache/rusic`).

## Built With

- **Dioxus**: Cross-platform UI framework
- **Rodio**: Audio playback
- **Lofty**: Audio metadata parsing
- **TailwindCSS**: Styling
