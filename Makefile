OS := $(shell uname)
.PHONY: install update upgrade dev build icon fmt desktop-local clean

install:
	yarn
	rustup component add rustfmt
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

icon:
	echo "Put the icon in this repository root folder, and name it app-icon.png"
	yarn tauri icon

fmt:
	cd src-tauri && cargo fmt
	yarn fmt

desktop-local:
	cp "src-tauri/target/release/bundle/appimage/emoji-mart_*_amd64.AppImage" "~/.local/bin/EmojiMart.AppImage"
	cp resources/EmojiMart.desktop ~/.local/share/applications/
	cp src-tauri/icons/icon.png ~/.local/share/applications/EmojiMart.png

clean:
	pkill ydotoold
	rm -rf .flatpak-builder build/ src-tauri/target
	rm -rf ~/.local/share/applications/EmojiMart.* ~/.local/bin/EmojiMart.AppImage
