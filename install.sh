#!/usr/bin/env bash


APPIMAGE_PATH="/home/vemonet/develop/perso/EmojiMart/src-tauri/target/release/bundle/appimage/emoji-mart-app_0.1.0_amd64.AppImage"

cp $APPIMAGE_PATH ~/.local/bin/EmojiMart.AppImage
cp public/EmojiMart.desktop ~/.local/share/applications/


# flatpak run io.github.vemonet.EmojiMart

# flatpak-builder --ccache --force-clean _build io.github.vemonet.EmojiMart.yml --repo=_repo
# flatpak build-bundle _repo io.github.vemonet.EmojiMart.flatpak io.github.vemonet.EmojiMart