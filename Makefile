OS := $(shell uname)
.PHONY: install install-wayland update upgrade dev build icon fmt desktop-local clean

install:
	yarn
	rustup component add rustfmt

install-wayland:
	git clone https://github.com/vemonet/ydotool -b no-scdoc
	mkdir ydotool/build
	cd ydotool/build && cmake .. && make -j `nproc`
	cp ydotool/build/ydotool* ~/.local/bin

upgrade:
	yarn upgrade @tauri-apps/cli @tauri-apps/api emoji-mart @emoji-mart/data
	cd src-tauri && cargo update

dev:
	ydotoold &
	yarn tauri dev

build:
	yarn tauri build

release:
	sed -i "s/\"version\": \"[0-9]*\.[0-9]*\.[0-9]*\",/\"version\": \"$(version)\",/g" ./package.json
	sed -i "s/\"version\": \"[0-9]*\.[0-9]*\.[0-9]*\"/\"version\": \"$(version)\"/g" ./src-tauri/tauri.conf.json
	sed -i "s/version = \"[0-9]*\.[0-9]*\.[0-9]*\"/version = \"$(version)\"/g" ./src-tauri/Cargo.toml

icon:
	echo "Put the icon in this repository root folder, and name it app-icon.png"
	yarn tauri icon

fmt:
	cd src-tauri && cargo fmt
	yarn fmt
	yarn lint

desktop-local:
	cp "src-tauri/target/release/bundle/appimage/emoji-mart_*_amd64.AppImage" "~/.local/bin/EmojiMart.AppImage"
	cp resources/EmojiMart.desktop ~/.local/share/applications/
	cp src-tauri/icons/icon.png ~/.local/share/applications/EmojiMart.png

clean:
	pkill ydotoold
	rm -rf .flatpak-builder build/ src-tauri/target
	rm -rf ~/.local/share/applications/EmojiMart.* ~/.local/bin/EmojiMart.AppImage
