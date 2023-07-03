# <span><img width="35" height="35" src="https://github.com/vemonet/EmojiMart/blob/main/src-tauri/icons/128x128.png"></span> Emoji Mart desktop popup

Modern emoji picker popup app for desktop, based on the amazing [Emoji Mart](https://github.com/missive/emoji-mart) web component.

- 🍾 Built as a popup: quick invocation through your system custom shortcuts, and disappears when not needed, does not stay as a standalone window, does not run in the background
- 🔎 Search text box automatically focused and ready to type when invoked
- ⌨️ Can use the keyboard to navigate and select emojis
- 🧠 Remember your favorite emojis
- ⚔️ Cross-platform, can be installed natively on Linux, MacOS, or Windows (although only tested on Linux at the moment)
- 🧑‍🚀 Uses modern and flexible technologies (TSX for the UI, Rust for the cross-platform compilation, what else?), making it easier to maintain and build upon in the future
- ✒️ On x11 the selected emoji is automatically pasted to your currently focused app, instead of being added to the clipboard! (it allows to uses emojis without losing what was copied before).
- ⚠️ Auto-paste can be enabled also on Wayland, but require to open permissions of `/dev/uinput`, which is not recommended for security.

🆕 This project just had its **first release**, it should be already usable, but you might face bugs as it is not yet battle tested (especially regarding auto-paste). Please report any weird behavior in the GitHub issues! And feel free to contribute, the codebase is quite small and understandable.

![Emoji Mart screenshot](https://raw.githubusercontent.com/vemonet/EmojiMart/main/public/screenshot.png)

Built with [Tauri](https://tauri.app/), [Svelte](https://svelte.dev), [TypeScript](https://www.typescriptlang.org/), and [Vite](https://vitejs.dev/).

## 📥️ Installation

### 🐧 Linux

<a href='https://flathub.org/apps/io.github.vemonet.EmojiMart'><img width='240' alt='Download on Flathub' src='https://dl.flathub.org/assets/badges/flathub-badge-en.png'/></a>

or

```bash
flatpak install io.github.vemonet.EmojiMart
```

⌨️ The easiest way to use Emoji Mart is to invoke it with a **custom system keyboard shortcut**, create one to register the command that starts the emoji picker:

```bash
flatpak run io.github.vemonet.EmojiMart
```

<details><summary>👣 More info on defining shortcuts in GNOME Shell here</summary>

Go to **Settings** > **Keyboard** > **View and Customize Shortcuts** > **Custom Shortcuts**

Or you can do it from the terminal, but you will need to adapt it if you already have existing custom shortcuts registered

```bash
# Check existing shortcuts list:
gsettings get org.gnome.settings-daemon.plugins.media-keys custom-keybindings

# Create shortcut 0 triggered with Super+E
gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/]"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ name 'Emoji Mart'
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ command 'flatpak run io.github.vemonet.EmojiMart'
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ binding '<Super>e'
```

</details>

🌓 Emoji Mart will try to use your system theme, but you can also set a **specific theme** using the `--theme` argument when starting the app, possible values are `dark`, `light`, and `auto`:

```bash
flatpak run io.github.vemonet.EmojiMart --theme light
```

👣 If you are using **Wayland on GNOME** we recommend to enable new windows to be **centered**, otherwise the popup will appear on the top left corner. If you are using Mutter, the default window composer for GNOME, you can do so by running the following command:

```bash
gsettings set org.gnome.mutter center-new-windows true
```

##### ⚠️ Enable auto-paste on Wayland

To enable auto-paste to work on Wayland you will need to give your user permission to read/write to the user input. It is not recommended in regard of security, and Emoji Mart will still work by copying the emoji to your clipboard if you don't make this change. But it is currently the only way we found to automatically paste on Wayland, please let us know if you know of a better way in the issues!

<!-- First you will need to install the flatpak from the GitHub releases, instead of Flathub:  -->

Add this udev rule which will enable your user to access `/dev/uinput`:

```bash
echo "KERNEL==\"uinput\", MODE=\"0660\", GROUP=\"$(id -gn)\", TAG+=\"uaccess\"" | sudo tee -a /etc/udev/rules.d/99-uinput.rules
# Then reload the rules (or reboot):
sudo udevadm control --reload-rules
```

<details><summary>Alternatively, but not recommended, you can also use the <code>.AppImage</code> file</summary>

This is not recommended, as the apps takes longer to startup than with the flatpak, and not all dependencies are included to auto-paste.

Run this command to download the `.AppImage`, and create a desktop file for it:

```bash
curl -Ls https://raw.github.com/vemonet/EmojiMart/main/install.sh | bash
```

Or manually download the `.AppImage` file from the [latest release](https://github.com/vemonet/EmojiMart/releases/latest), and install it.

If you are using x11 you will need to make sure `xdotool` is installed on the system, e.g. for fedora:

```bash
sudo dnf install libxdo-devel
```

</details>

### 🍎 MacOS

Download the `.dmg` for the [latest release](https://github.com/vemonet/EmojiMart/releases/latest), and install it.

Then create a custom shortcut for the command: `flatpak run io.github.vemonet.EmojiMart` to invoke it quickly.

### 🪟 Windows

Download the `.exe` for the [latest release](https://github.com/vemonet/EmojiMart/releases/latest), and install it.

Then create a custom shortcut for the command: `flatpak run io.github.vemonet.EmojiMart` to invoke it quickly.

## 📋️ Notes

The app currently does not run in the background. It could be invoked slightly faster with a built-in shortcut, and running in the background, but that increases the chances of people starting many phantom processes without knowing. When running in the background and invoked using the built-in shortcut the window is not properly focused. Alternatively when installed with flatpak starting it without letting it run in the background is fast enough for using it like this directly, and does not have any issue with focusing.

Letting the user register custom system shortcuts, instead of having the app registering the shortcut for the user, prevents bugs and conflicts with other shortcuts. Which enable the users to choose from a larger amount of shortcuts.

Inspired by:

- [github.com/tom-james-watson/Emote](https://github.com/tom-james-watson/Emote) my favorite GTK emoji picker
- [github.com/Simon-Laux/tauri-emoji-mart-app](https://github.com/Simon-Laux/tauri-emoji-mart-app) who combined tauri with emoji-mart, difference are that we use Svelte instead of React, and the popup design has been improved.

[Icon](https://www.vecteezy.com/vector-art/5726169-cardboard-box-funny-box-box-character-delivery-box-box-emoji) credits: <a href="https://www.vecteezy.com/members/duniaonme653898">duniaonme653898 on Vecteezy</a>

### ☑️ Todo

- [x] Auto-paste on Wayland: currently using `ydotool` requires too many permissions (`--device=all` and user r/w access to `/dev/uinput`). 
  - [ ] Use [libei](https://gitlab.freedesktop.org/libinput/libei): once it has  been implemented by mutter: https://gitlab.gnome.org/GNOME/mutter/-/merge_requests/2628
  - [ ] And merged to flatpak https://github.com/flatpak/xdg-desktop-portal/pull/762

- [ ] On x11: improve the process to add the emoji to the clipboard > paste > close the app. Currently there is an issue with `xdotool` clearing the clipboard when called from tauri
- [ ] Select multiple emoji when pressing a specific key, e.g. when pressing shift?
- [ ] Check if working properly on MacOS
- [ ] Check if working properly on Windows
- [ ] Add auto-paste on Windows and MacOS when the compatibility between Enigo and Tauri is resolved (cf. https://github.com/enigo-rs/enigo/issues/15 and https://github.com/tauri-apps/tauri/issues/6421)

## 🛠️ Development

[![Latest release](https://shields.io/github/v/release/vemonet/EmojiMart)](https://github.com/vemonet/EmojiMart/releases/latest) [![Build app](https://github.com/vemonet/EmojiMart/actions/workflows/build.yml/badge.svg)](https://github.com/vemonet/EmojiMart/actions/workflows/build.yml) [![Publish new release](https://github.com/vemonet/EmojiMart/actions/workflows/release.yml/badge.svg)](https://github.com/vemonet/EmojiMart/actions/workflows/release.yml)

Recommended IDE Setup: [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

Requirements: [yarn v1](https://classic.yarnpkg.com) (v3 is not supported by flatpak to generate npm sources)

See pre-requesites to run tauri: https://tauri.app/v1/guides/getting-started/prerequisites

### 🧶 Install

Additional dependencies for Linux to enable auto-paste on x11:

```bash
sudo dnf install libX11-devel libxdo-devel
```

Install dependencies:

```bash
make install
```

If you are developing on Wayland you will need to install extra dependencies (e.g. `ydotool` to auto-paste):

```bash
make install-wayland
```

### 🛩️ Run

In development mode, with automatic reload when the code changes:

```bash
make dev
```

> To see logs and debug: right click on the app window, and select `Inspect Element`

### 📦️ Build

To build the **Flatpak** package checkout this repository: [github.com/flathub/io.github.vemonet.EmojiMart](https://github.com/flathub/io.github.vemonet.EmojiMart)

Build `.AppImage` and `.deb` packages, or `.dmg`/`.exe` depending on your OS:

```bash
make build
```

On Linux, you can install the previously built `EmojiMart.AppImage` as desktop app:

```bash
make desktop-local
```

### ✅ Format

Run automatic formatting and linting of the codebase:

```bash
make fmt
```

### ⏫ Upgrade dependencies

You might want to upgrade the latest versions of:

- Tauri app: [tauri.app](https://tauri.app)
- EmojiMart web component: [npmjs.com/package/emoji-mart](https://www.npmjs.com/package/emoji-mart) and [npmjs.com/package/@emoji-mart/data](https://www.npmjs.com/package/@emoji-mart/data)

To automatically upgrade dependencies with `yarn` and `cargo` you can run:

```bash
make upgrade
```

### 🏷️ New release

To publish a new release, follow this process:

1. Changed the version in: `package.json`, `src-tauri/Cargo.toml` and `src-tauri/tauri.conf.json`, you can use this script to do it automatically for a specific new version:

   ```bash
   make version=0.1.2 bump
   ```

2. Merge the `main` branch to the `release` branch, and push the `release` branch to GitHub.

3. A [GitHub Action workflow](https://github.com/vemonet/EmojiMart/actions/workflows/release.yml) will automatically build the artefacts for the different platforms, and create a [new release on GitHub](https://github.com/vemonet/EmojiMart/releases).

4. Finally send a pull request to update the tag in the [flathub repository](https://github.com/flathub/io.github.vemonet.EmojiMart) to trigger a new release on flathub.

### 🖼️ Change icon

Put the new icon file named `app-icon.png` (ideally size 512 or 1024) at the root of the repo, and run (for more details see the [official docs](https://tauri.app/fr/v1/guides/features/icons/)):

```bash
make icon
```
