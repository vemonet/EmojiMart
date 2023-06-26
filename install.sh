#!/usr/bin/env bash

# Script to install the Emoji Mart AppImage as desktop application on linux
VERSION="0.1.0"

if [ -z $1 ]; then
    echo "⏳️ Downloading Emoji Mart AppImage and its desktop app..."
    curl -L -o $HOME/.local/bin/EmojiMart.AppImage https://github.com/vemonet/EmojiMart/releases/download/v$VERSION/emoji-mart_$VERSION\_amd64.AppImage
    curl -L -o $HOME/.local/share/applications/EmojiMart.desktop "https://github.com/vemonet/EmojiMart/blob/main/public/EmojiMart.desktop?raw=true"
else
    echo "📂 Installing from local"
    rm -f ~/.local/bin/EmojiMart.AppImage
    BUILT_APPIMAGE="src-tauri/target/release/bundle/appimage/emoji-mart_*_amd64.AppImage"
    cp $BUILT_APPIMAGE ~/.local/bin/EmojiMart.AppImage
    cp public/EmojiMart.desktop ~/.local/share/applications/

    # Copy icon
    cp src-tauri/icons/icon.png ~/.local/share/applications/EmojiMart.png
fi
