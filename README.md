# Sutemi

Sutemi is a Tauri + Vue desktop GUI for [Jujutsu](https://github.com/jj-vcs/jj). It focuses on a fast native dashboard for repository status, recent history, and bookmarks, wrapped in an expressive Material-inspired UI.

## Stack

- Tauri 2
- Vue 3 + Vite
- Native Rust command bridge to `jj`
- GitHub Actions release workflow for macOS, Linux, and Windows

## Local development

1. Install Rust.
2. Install Jujutsu and ensure `jj` is on your `PATH`.
3. Install dependencies with `npm install`.
4. Start the web shell with `npm run dev`.
5. Start the desktop app with `npm run tauri dev`.

## Release flow

Pushing a tag like `v0.2.0` triggers `.github/workflows/release.yml` and builds installers for:

- macOS
- Linux
- Windows

The workflow publishes the generated artifacts to the GitHub release for that tag.
