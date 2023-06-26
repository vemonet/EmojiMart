#!/usr/bin/env bash

# Script to install the Emoji Mart AppImage as desktop application on linux
APPIMAGE_PATH="/home/vemonet/develop/perso/EmojiMart/src-tauri/target/release/bundle/appimage/emoji-mart_*_amd64.AppImage"
VERSION="0.1.0"

# Download:
# wget -O ~/.local/bin/EmojiMart.AppImage https://github.com/vemonet/EmojiMart/releases/download/$VERSION/emoji-mart_$VERSION_amd64.AppImage
# wget -O ~/.local/share/applications/EmojiMart.desktop https://github.com/vemonet/EmojiMart/blob/main/public/EmojiMart.desktop?raw=true

# Copy from local:
rm -f ~/.local/bin/EmojiMart.AppImage
cp $APPIMAGE_PATH ~/.local/bin/EmojiMart.AppImage
cp public/EmojiMart.desktop ~/.local/share/applications/

# Copy icon
cp src-tauri/icons/icon.png ~/.local/share/applications/EmojiMart.png
