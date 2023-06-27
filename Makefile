OS := $(shell uname)
.PHONY: install update upgrade dev build icon fmt desktop-local clean

install:
	yarn
	rustup component add rustfmt

update:
	yarn
	npm i --save-exact

upgrade:
	yarn up @tauri-apps/cli @tauri-apps/api emoji-mart @emoji-mart/data
	cd src-tauri && cargo update

dev:
	yarn tauri dev

build:
	yarn tauri build

icon:
	echo "Put the icon in this repository root folder, and name it app-icon.png"
	yarn tauri icon

fmt:
	cd src-tauri && cargo fmt
	yarn fmt

desktop-local:
	cp "src-tauri/target/release/bundle/appimage/emoji-mart_*_amd64.AppImage" "~/.local/bin/EmojiMart.AppImage"
	cp public/EmojiMart.desktop ~/.local/share/applications/
	cp src-tauri/icons/icon.png ~/.local/share/applications/EmojiMart.png

clean:
	rm -rf .flatpak-builder build/
	rm -rf ~/.local/share/applications/EmojiMart.* ~/.local/bin/EmojiMart.AppImage
