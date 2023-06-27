# 🏪 Emoji Mart desktop popup

Modern emoji picker popup app for desktop, based on the [Emoji Mart](https://github.com/missive/emoji-mart) web component.

* 🍾 Built as a popup: quick invocation with built-in shortcut `Alt+Space`, and disappears when not needed, does not stay as a standalone window
* 🔎 Search text box automatically focused and ready to type when invoked
* ⌨️ Can use the keyboard to navigate and select emojis
* 💽 Runs in the background, with a system tray icon to invoke or stop the app
* ⚔️ Cross-platform, can be installed natively on Linux, MacOS, or Windows
* 🧑‍🚀 Uses modern and flexible technologies (JSX for UI, Rust for cross-platform compilation, what else?), making it easier to maintain and build upon in the future

Built with [Tauri](https://tauri.app/), [Solid JS](https://www.solidjs.com/), [TypeScript](https://www.typescriptlang.org/), and [Vite](https://vitejs.dev/).

![Emoji Mart screenshot](https://raw.githubusercontent.com/vemonet/EmojiMart/main/public/screenshot.png)

> Inspired by [github.com/Simon-Laux/tauri-emoji-mart-app](https://github.com/Simon-Laux/tauri-emoji-mart-app), difference are that we use Solid JS instead of React, and the popup design and performances have been improved.

## 📥️ Installation

### 🐧 Linux

Run this command to download the `.AppImage`, and create a desktop file for it:

```bash
curl -Ls https://raw.github.com/vemonet/EmojiMart/main/install.sh | bash
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

See pre-requesites to run tauri: https://tauri.app/v1/guides/getting-started/prerequisites

Additional dependencies for Linux:

```bash
sudo dnf install libX11-devel libxdo-devel libappindicator-gtk3
```

Install npm packages:

```bash
yarn
```

On Linux you need to install additional dependencies for the system tray:

```bash
sudo dnf install libappindicator
```

### 🛩️ Run

In development mode, with automatic reload when the code changes:

```bash
yarn tauri dev
```

> To see logs and debug: right click on the app window, and select `Inspect Element`

### 📦️ Build

Build `.AppImage` and `.deb` packages, or `.dmg`/`.exe` depending on your OS:

```bash
yarn tauri build
```

On Linux, install the previously built `EmojiMart.AppImage` as desktop app:

```bash
./install.sh local
```

### 🔄 Change icon

Put your icon file named `app-icon.png` (ideally size 512 or 1024) at the root of the repo, and run (cf. [official docs](https://tauri.app/fr/v1/guides/features/icons/)):

```bash
yarn tauri icon
```

## ⏫ Upgrade dependencies

Check latest: 

* EmojiMart web component: [npmjs.com/package/emoji-mart](https://www.npmjs.com/package/emoji-mart) and [npmjs.com/package/@emoji-mart/data](https://www.npmjs.com/package/@emoji-mart/data)
* Tauri app: [tauri.app](https://tauri.app)

```bash
yarn up @tauri-apps/cli @tauri-apps/api emoji-mart @emoji-mart/data
cargo update
```

### 🏷️ New release

Repository to build the Flatpak package: [github.com/vemonet/flathub/tree/io.github.vemonet.EmojiMart](https://github.com/vemonet/flathub/tree/io.github.vemonet.EmojiMart)

To publish a new release, follow this process:

1. Make sure you have changed the version in: `package.json`, `src-tauri/Cargo.toml` and `src-tauri/tauri.conf.json`

2. Update the `yarn.lock` and `package-lock.json` (required for flatpak build):

   ```bash
   yarn
   npm i --save-exact
   ```

3. Merge the `main` branch to the `release` branch, and push the `release` branch to GitHub
4. A [GitHub Action workflow](https://github.com/vemonet/EmojiMart/actions/workflows/release.yml) will automatically build the artefacts for the different platforms, and create a draft release
5. Finally you can review the draft on the [**Releases** page](https://github.com/vemonet/EmojiMart/releases), generate the Release note, and publish it in 2 clicks

## 📋️ Todo

- [ ] When running in the background and invoked using the built-in shortcut the window is not properly focused. Alternatively when installed with flatpak starting it without letting it run in the background is fast enough for using it like this directly, and does not have any issue with focusing.

- [ ] Build Flatpak:
  - [ ] Example: https://github.com/hadess/flathub/tree/d4b53ff829e0917c5129294132f619e5f12d337c
  
  - [ ] Autostart: use dbus portals https://github.com/diwic/dbus-rs/blob/master/dbus/examples/properties.rs
  
    ```python
    bus = dbus.SessionBus()
    obj = bus.get_object("org.freedesktop.portal.Desktop", "/org/freedesktop/portal/desktop")
    inter = dbus.Interface(obj, "org.freedesktop.portal.Background")
    res = inter.RequestBackground('', {
        'reason': 'Emote autostart',
        'autostart': True, 'background': True,
        'commandline': dbus.Array(['emote'])
    })
    ```
  
    
