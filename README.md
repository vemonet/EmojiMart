# <span><img width="35" height="35" src="https://github.com/vemonet/EmojiMart/blob/main/src-tauri/icons/128x128.png"></span> Emoji Mart desktop popup

[![FlatHub release](https://img.shields.io/flathub/v/io.github.vemonet.EmojiMart)](https://flathub.org/apps/io.github.vemonet.EmojiMart)
[![Latest release](https://shields.io/github/v/release/vemonet/EmojiMart?label=download)](https://github.com/vemonet/EmojiMart/releases/latest)
[![Build](https://github.com/vemonet/EmojiMart/actions/workflows/build.yml/badge.svg)](https://github.com/vemonet/EmojiMart/actions/workflows/build.yml)

Modern emoji picker popup for desktop, based on the amazing [Emoji Mart](https://github.com/missive/emoji-mart) web component, built with [Tauri](https://tauri.app/) and [Svelte](https://svelte.dev).

- 🍾 Built as a popup: quick invocation through your system custom shortcuts, and disappears when not needed, does not stay as a standalone window, does not run in the background
- 🔎 Search text box automatically focused and ready to type when invoked
- ⌨️ Use the keyboard to navigate and select emojis
- ✒️ On x11 the selected emoji is automatically pasted to your currently focused app
- 🌍 Complete translation in [22 languages](https://github.com/vemonet/EmojiMart/tree/main/src/data), it will use your system language automatically
- 🧠 Remembers your frequently used emojis
- ⚔️ Cross-platform, it can be installed natively on Linux, MacOS, or Windows (although only tested on Linux at the moment)
- 🧑‍🚀 Built with Tauri, TypeScript, and Svelte

> Please report any weird behavior in the GitHub issues! And feel free to contribute, the codebase is quite small and understandable.

![Emoji Mart screenshot](https://raw.github.com/vemonet/EmojiMart/main/resources/screenshot.png)

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

🌍 Emoji Mart will try to use your system language, but you can also set a **specific language** using the `--lang` argument when starting the app:

```bash
flatpak run io.github.vemonet.EmojiMart --lang fr
```

♻️ If you want to only rely on auto-paste and keep what is in your clipboard use the `--keep` argument:

```bash
flatpak run io.github.vemonet.EmojiMart --keep
```

👣 If you are using **Wayland on GNOME** we recommend to enable new windows to be **centered**, otherwise the popup will appear on the top left corner. If you are using Mutter, the default window composer for GNOME, you can do so by running the following command:

```bash
gsettings set org.gnome.mutter center-new-windows true
```

> [!CAUTION]
>
> Tauri v2 crashes when used with flatpak on Wayland (as usual Wayland breaks everything). So you will need to use the `.appimage` or the flatpak `v0.2.4`

<!--

##### ⚠️ Enable auto-paste on Wayland

 To enable auto-paste to work on Wayland you will need to give your user permission to read/write to the user input. It is not recommended in regard of security, and Emoji Mart will still work by copying the emoji to your clipboard if you don't make this change. But it is currently the only way we found to automatically paste on Wayland, please let us know if you know of a better way in the issues!

First you will need to download and install the `.flatpak` file from the GitHub releases, instead of Flathub:

```bash
curl -Lo io.github.vemonet.EmojiMart.flatpak https://github.com/vemonet/EmojiMart/releases/latest/download/io.github.vemonet.EmojiMart.flatpak
flatpak install io.github.vemonet.EmojiMart.flatpak
```

And add this udev rule which will enable your user to access `/dev/uinput`:

```bash
echo "KERNEL==\"uinput\", MODE=\"0660\", GROUP=\"$(id -gn)\", TAG+=\"uaccess\"" | sudo tee -a /etc/udev/rules.d/99-uinput.rules
# Or this command should work too but I did not tested it yet:
sudo usermod -a -G input $(whoami)
# Then reboot to make sure changes are loaded
sudo reboot
```

-->

<details><summary>Alternatively, you can use the <code>.AppImage</code> file</summary>

This is not recommended, as the app takes slightly longer to startup than with flatpak, and not all dependencies are included to auto-paste.

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

The app currently does not run in the background. It could be invoked slightly faster with a built-in shortcut, and running in the background, but that increases the chances of people starting many phantom processes without knowing. Alternatively when installed with flatpak starting it without letting it run in the background is fast enough for using it like this directly, and does not have any issue with focusing.

Letting the user register custom system shortcuts, instead of having the app registering the shortcut for the user, prevents bugs and conflicts with other shortcuts. Which enable the users to choose from a larger amount of shortcuts.

Inspired by:

- [tom-james-watson/Emote](https://github.com/tom-james-watson/Emote) my favorite GTK emoji picker
- [Simon-Laux/tauri-emoji-mart-app](https://github.com/Simon-Laux/tauri-emoji-mart-app) who combined tauri with emoji-mart, difference are that we use Svelte instead of React, and the popup design has been improved.

[Icon](https://www.vecteezy.com/vector-art/5726169-cardboard-box-funny-box-box-character-delivery-box-box-emoji) credits: <a href="https://www.vecteezy.com/members/duniaonme653898">duniaonme653898 on Vecteezy</a>

### ☑️ Todo

- [ ] Auto-paste on Wayland: currently using `ydotool` but requires too many permissions (`--device=all` and user r/w access to `/dev/uinput`). And it tries to install files at the wrong place in flatpak, so it fails

  - [ ] Use [libei](https://gitlab.freedesktop.org/libinput/libei): once it has been implemented by mutter: https://gitlab.gnome.org/GNOME/mutter/-/merge_requests/2628 and merged to flatpak https://github.com/flatpak/xdg-desktop-portal/pull/762
  - [ ] Recent rust [crate for libei](https://crates.io/crates/reis) protocol: https://github.com/ids1024/reis/blob/master/examples/type-text.rs

- [ ] Select multiple emoji when pressing a specific key, e.g. when pressing shift
- [ ] Add auto-paste on Windows and MacOS when the compatibility between Enigo and Tauri is resolved (cf. https://github.com/enigo-rs/enigo/issues/15 and https://github.com/tauri-apps/tauri/issues/6421)
- [ ] Improve persistence https://aptabase.com/blog/persistent-state-tauri-apps

## 🛠️ Development

Built with [Tauri](https://tauri.app/), [Svelte](https://svelte.dev), [TypeScript](https://www.typescriptlang.org/), and [Vite](https://vitejs.dev/).

Recommended IDE Setup: [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

Requirements: NodeJS (20 ideally)

See pre-requesites to run tauri: https://tauri.app/v1/guides/getting-started/prerequisites

### 🧶 Install

Additional dependencies for Linux to enable auto-paste on x11:

```bash
# On debian/ubuntu
sudo apt install -y libwebkit2gtk-4.1-dev librsvg2-dev libx11-dev libxdo-dev build-essential libssl-dev libayatana-appindicator3-dev
# On fedora
sudo dnf install -y libX11-devel libxdo-devel
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

To build the **Flatpak** package checkout this repository: [flathub/io.github.vemonet.EmojiMart](https://github.com/flathub/io.github.vemonet.EmojiMart)

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
   make bump version=0.1.2
   ```

2. Merge the `main` branch to the `release` branch, and push the `release` branch to GitHub:

   ```bash
   make release
   ```

3. A [GitHub Action workflow](https://github.com/vemonet/EmojiMart/actions/workflows/release.yml) will automatically build the artefacts for the different platforms, and create a [new release on GitHub](https://github.com/vemonet/EmojiMart/releases).

4. Finally send a pull request to update the tag in the [flathub repository](https://github.com/flathub/io.github.vemonet.EmojiMart) to trigger a new release on flathub.

### 🖼️ Change icon

Put the new icon file named `app-icon.png` (ideally size 512 or 1024) at the root of the repo, and run (for more details see the [official docs](https://tauri.app/fr/v1/guides/features/icons/)):

```bash
make icon
```
