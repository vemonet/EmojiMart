OS := $(shell uname)
.PHONY: install install-wayland upgrade dev build i18n bump release icon fmt desktop-local clean

install:
	npm ci
	rustup component add rustfmt
# sudo apt-get install xdotool

# Install ydotool to auto-paste on wayland
install-wayland:
	git clone https://github.com/vemonet/ydotool -b no-scdoc
	mkdir ydotool/build
	cd ydotool/build && cmake .. && make -j `nproc`
	cp ydotool/build/ydotool* ~/.local/bin
	ydotoold &

upgrade:
	npm run upgrade
	cd src-tauri && cargo update

dev:
	npm run tauri dev

build:
	npm run tauri build

translate:
	git -C "cldr-json" pull || git clone https://github.com/unicode-org/cldr-json.git "cldr-json"
	git -C "emoji-mart" pull || git clone https://github.com/missive/emoji-mart.git "emoji-mart"
	node resources/internationalize.js $(version)

bump:
	sed -i "s/\"version\": \"[0-9]*\.[0-9]*\.[0-9]*\",/\"version\": \"$(version)\",/g" ./package.json
	sed -i "s/\"version\": \"[0-9]*\.[0-9]*\.[0-9]*\"/\"version\": \"$(version)\"/g" ./src-tauri/tauri.conf.json
	sed -i "s/version = \"[0-9]*\.[0-9]*\.[0-9]*\"/version = \"$(version)\"/g" ./src-tauri/Cargo.toml
	rm -rf src-tauri/target/release/bundle
# git add package.json src-tauri/tauri.conf.json src-tauri/Cargo.toml
# git commit -m "⏫ Bump to version $(version)"

release:
	git checkout release
	git merge main
	git push
	git checkout main

fmt:
	cd src-tauri && cargo fmt
	npm run fmt
	npm run lint

desktop-local:
	cp "src-tauri/target/release/bundle/appimage/emoji-mart_*_amd64.AppImage" "~/.local/bin/EmojiMart.AppImage"
	cp resources/EmojiMart.desktop ~/.local/share/applications/
	cp src-tauri/icons/icon.png ~/.local/share/applications/EmojiMart.png

clean:
	rm -rf ~/.local/share/applications/EmojiMart.* ~/.local/bin/EmojiMart.AppImage
	rm -rf .svelte-kit
	rm -rf src-tauri/target
	pkill ydotoold

icon:
	echo "Put the icon in this repository root folder, and name it app-icon.png"
	npm run tauri icon
