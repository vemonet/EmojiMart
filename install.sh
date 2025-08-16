#!/usr/bin/env bash

# Script to install the Emoji Mart AppImage as desktop application on linux
VERSION="0.3.1"

if [ -z $1 ]; then
    echo "⏳️ Downloading Emoji Mart AppImage and its desktop file..."
    curl -L -o ~/.local/bin/EmojiMart.AppImage https://github.com/vemonet/EmojiMart/releases/download/v$VERSION/emoji-mart_$VERSION\_amd64.AppImage
    curl -L -o ~/.local/share/applications/EmojiMart.desktop "https://github.com/vemonet/EmojiMart/blob/main/resources/EmojiMart.desktop?raw=true"
    chmod +x ~/.local/bin/EmojiMart.AppImage
    # cp ~/.local/share/applications/EmojiMart.desktop ~/.config/autostart/
else
    echo "📂 Installing from local"
    rm -f ~/.local/bin/EmojiMart.AppImage
    BUILT_APPIMAGE="src-tauri/target/release/bundle/appimage/emoji-mart_*_amd64.AppImage"
    cp $BUILT_APPIMAGE ~/.local/bin/EmojiMart.AppImage
    cp resources/EmojiMart.desktop ~/.local/share/applications/
    cp src-tauri/icons/icon.png ~/.local/share/applications/EmojiMart.png
    # cp ~/.local/share/applications/EmojiMart.desktop ~/.config/autostart/
fi
