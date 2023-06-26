# 🏪 Emoji Mart app

Modern emoji picker popup for desktop, based on [Emoji Mart](https://github.com/missive/emoji-mart).

Built with Tauri, Solid JS, Typescript, and Vite. 

Inspired from https://github.com/Simon-Laux/tauri-emoji-mart-app, difference are that we use SolidJS instead of React, the title bar is gone, windows size has been improved, and the picker pops in the middle of the screen now.

![Emoji Mart screenshot](https://raw.githubusercontent.com/vemonet/EmojiMart/main/public/screenshot.png)

## 🛠️ Development

[![Build app](https://github.com/vemonet/EmojiMart/actions/workflows/build.yml/badge.svg)](https://github.com/vemonet/EmojiMart/actions/workflows/build.yml)

Recommended IDE Setup: [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

Install:

```bash
yarn
```

Run in development:

```bash
yarn tauri dev
```

> To see logs and debug: right click on the app window, and select `Inspect Element`

Build:

```bash
yarn tauri build
```

Install the previously built `EmojiMart.AppImage` as desktop app:

```bash
./install.sh
```

## 📋️ Todo

- [ ] Improve popup speed:
  - [x] Checkout tauri docs to run in the background: https://tauri.app/fr/v1/guides/features/system-tray/#preventing-the-app-from-closing
  - [ ] Or does not close the window, and bring it back to the front instead of creating a new window when the AppImage is called
  - [ ] Maybe look into tauri global shortcut to hide/show: https://tauri.app/fr/v1/api/js/globalshortcut/
- [ ] Flatpak: https://github.com/hadess/flathub/tree/d4b53ff829e0917c5129294132f619e5f12d337c
- [ ] Closing the window when clickout with the `WINDOW_BLUR` event is also triggered when running in development, which is not convenient for inspecting the element
