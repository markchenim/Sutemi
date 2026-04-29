# Sutemi

Sutemi is a Tauri + Vue desktop GUI for [Jujutsu](https://github.com/jj-vcs/jj). It focuses on a fast native dashboard for repository status, recent history, bookmarks, and quick repository orientation, wrapped in an expressive Material-inspired UI.

## Stack

- Tauri 2
- Vue 3 + Vite
- Native Rust command bridge to `jj`
- GitHub Actions release workflow for macOS, Linux, and Windows
- Generated cross-platform icon pipeline for local builds and CI

## Local development

1. Install Rust.
2. Install Jujutsu and ensure `jj` is on your `PATH`.
3. Install dependencies with `npm install`.
4. Start the web shell with `npm run dev`.
5. Start the desktop app with `npm run tauri dev`.

## Release flow

Pushing a tag like `v0.5.0` triggers `.github/workflows/release.yml` and builds installers for:

- macOS
- Linux
- Windows

The workflow publishes the generated artifacts to the GitHub release for that tag.
