# 🏪 Emoji Mart desktop popup

Modern emoji picker popup app for desktop, based on the [Emoji Mart](https://github.com/missive/emoji-mart) web component.

Built with [Tauri](https://tauri.app/), [Solid JS](https://www.solidjs.com/), [TypeScript](https://www.typescriptlang.org/), and [Vite](https://vitejs.dev/). 

Inspired from [github.com/Simon-Laux/tauri-emoji-mart-app](https://github.com/Simon-Laux/tauri-emoji-mart-app), difference are that we use Solid JS instead of React, the title bar is gone, window size has been improved, and the window pops in the middle of the screen.

![Emoji Mart screenshot](https://raw.githubusercontent.com/vemonet/EmojiMart/main/public/screenshot.png)

## 📥️ Installation

### 🐧 Linux

Run this command to download the `.AppImage`, and create a desktop file for it:

```bash
curl https://raw.github.com/vemonet/EmojiMart/release/install.sh | bash
```

You can also manually download the `.AppImage` from the [latest release](https://github.com/vemonet/EmojiMart/releases/latest), and install it.

### 🍎 MacOS

Download the `.dmg` for the [latest release](https://github.com/vemonet/EmojiMart/releases/latest), and install it.

### 🪟 Windows

Download the `.exe` for the [latest release](https://github.com/vemonet/EmojiMart/releases/latest), and install it.

## 🛠️ Development

[![Latest release](https://shields.io/github/v/release/vemonet/EmojiMart)](https://github.com/vemonet/EmojiMart/releases/latest) [![Build app](https://github.com/vemonet/EmojiMart/actions/workflows/build.yml/badge.svg)](https://github.com/vemonet/EmojiMart/actions/workflows/build.yml) [![Publish new release](https://github.com/vemonet/EmojiMart/actions/workflows/release.yml/badge.svg)](https://github.com/vemonet/EmojiMart/actions/workflows/release.yml)

Recommended IDE Setup: [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### 🧶 Install

```bash
yarn
```

### 🛩️ Run

In development mode, with automatic reload when the code changes:

```bash
yarn tauri dev
```

> To see logs and debug: right click on the app window, and select `Inspect Element`

### 📦️ Package

Build an `.AppImage` and `.deb` packages (or `.dmg`/`.exe` depending on your OS):

```bash
yarn tauri build
```

On Linux, install the previously built `EmojiMart.AppImage` as desktop app:

```bash
./install.sh local
```

### 🏷️ New release

To publish a new release, follow this process:

1. Make sure you have changed the version in: `package.json`, `src-tauri/Cargo.toml` and `tauri.conf.json`

2. Update the `yarn.lock` and `package-lock.json` (required for flatpak build):

   ```bash
   yarn
   npm i --save-exact
   ```

3. Merge the `main` branch to the `release` branch, and push the `release` branch to GitHub
4. A [GitHub Action workflow](https://github.com/vemonet/EmojiMart/actions/workflows/release.yml) will automatically build the artefacts for the different platforms, and create a draft release
5. Finally you can review the draft on the [**Releases** page](https://github.com/vemonet/EmojiMart/releases), generate the Release note, and publish it in 2 clicks

## 📋️ Todo

- [ ] Improve popup speed: 
  - [ ] Calling the AppImage creates a new window, instead of reusing open processes
  - [x] Checkout tauri docs to run in the background: https://tauri.app/fr/v1/guides/features/system-tray/#preventing-the-app-from-closing
  - [ ] Try to not close the window (hide?), and bring it back to the front instead of creating a new window when the AppImage is called?
  - [ ] Look into tauri global shortcut to hide/show: https://tauri.app/fr/v1/api/js/globalshortcut/
- [ ] Closing the window when clickout with the `WINDOW_BLUR` event is also triggered when running in development, which is not convenient for inspecting the element
- [ ] Build Flatpak: 
  - [ ] Example: https://github.com/hadess/flathub/tree/d4b53ff829e0917c5129294132f619e5f12d337c
