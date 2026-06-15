# Messenger Desktop

A lightweight Tauri v2 wrapper for Facebook Messenger Web.

## What This Includes

- Main Tauri window pointed at `https://www.messenger.com`
- Native tray menu with Show, Hide, Reload Messenger, Settings, and Quit
- Close-to-tray behavior
- `Ctrl+Shift+M` global show/hide shortcut
- Local settings window for start-on-login, close-to-tray, start-minimized, shortcut display, and Messenger URL choice
- Settings persisted to the app config directory as `settings.json`

## Requirements

- Node.js and npm
- Rust and Cargo through rustup
- Microsoft C++ Build Tools
- Microsoft Edge WebView2 Runtime

This machine already has Node.js, WebView2, and MSVC available, but `rustc`, `cargo`, `rustup`, and `winget` were not found on PATH when this scaffold was created.

## Commands

Use `npm.cmd` from PowerShell if script execution policy blocks `npm.ps1`.

```powershell
npm.cmd install
npm.cmd run icons
npm.cmd run build
npm.cmd run tauri -- dev
npm.cmd run tauri -- build
```

Built installers will be under:

```text
src-tauri/target/release/bundle/
```

## Changing The App Icon

The source image for the app icon is:

```text
src-tauri/icons/app-icon.png
```

Replace that file when you want to change the desktop thumbnail, taskbar icon,
installer icon, Windows `.ico`, or macOS `.icns`.

Recommended image format:

- PNG
- Square, ideally `1024x1024`
- Transparent background
- Rounded corners should be part of the visible artwork, with transparent
  pixels outside the rounded shape
- No white square background behind the icon

If the icon shows a white box on the Windows desktop, the source PNG probably
has opaque white pixels in the corners. Remove the white background outside the
rounded shape, but keep any intentional white artwork inside the icon, such as
the chat bubble.

The previous white-background version is kept here as a backup:

```text
src-tauri/icons/app-icon-with-white-bg.png
```

After replacing `app-icon.png`, regenerate all platform icon files:

```powershell
npm.cmd run icons
```

This updates files such as:

```text
src-tauri/icons/icon.ico
src-tauri/icons/icon.icns
src-tauri/icons/32x32.png
src-tauri/icons/128x128.png
src-tauri/icons/128x128@2x.png
```

Then rebuild the installers:

```powershell
npm.cmd run tauri -- build
```

If Windows still shows the old icon after installing the new build, close the
app, unpin the old taskbar item, install the new version, and restart Windows
Explorer or reboot. Windows aggressively caches desktop and taskbar icons.

## Release Downloads

This project includes a GitHub Actions workflow at `.github/workflows/release.yml`.
It builds release artifacts on native runners:

- Windows x64 on `windows-latest`
- macOS Intel on `macos-13`
- macOS Apple Silicon on `macos-14`

To publish downloadable installers:

```powershell
git init
git add .
git commit -m "Initial Messenger Desktop wrapper"
git branch -M main
git remote add origin <your-github-repo-url>
git push -u origin main
git tag v0.1.0
git push origin v0.1.0
```

The workflow creates a draft GitHub Release containing the generated installers.
Review the draft, then publish it.

Windows users can install the generated `.msi` or `.exe` setup file.
macOS users can install the generated `.dmg` or extracted `.app` bundle.

Unsigned macOS builds may show a Gatekeeper warning. For a no-warning public
release, configure Apple Developer signing and notarization secrets in GitHub
Actions.
