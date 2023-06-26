# 🏪 Emoji Mart app

Modern emoji picker popup for desktop, based on [Emoji Mart](https://github.com/missive/emoji-mart).

Built with Tauri, Solid JS, Typescript, and Vite. 

Inspired from https://github.com/Simon-Laux/tauri-emoji-mart-app, difference are that we use SolidJS instead of React, the title bar is gone, windows size has been improved, and the picker pops in the middle of the screen now.

## 🛠️ Development

Install:

```bash
yarn
```

Run in development:

```bash
yarn tauri dev
```

Build:

```bash
yarn tauri build
```

## 🕹️ Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 📋️ Notes

Creating a system tray and running in the background: https://tauri.app/fr/v1/guides/features/system-tray/#preventing-the-app-from-closing
