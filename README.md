# Filera
A powerful, cross-platform batch file renaming tool written in Rust.

![filera app ui](src/assets/app_ui.png?raw=true "Filera App UI")

> ** Beta: Functional but still in development. Code signing certificates pending from Apple and Microsoft.

## Features

- Batch rename files with pattern-based rules
- Preview changes before applying
- Recursive folder support
- Light/dark mode
- Cross-platform (Windows, macOS, Linux)

## Installation
Download from [Releases](https://github.com/joncorv/filera/releases) 

### Building from Source
- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (v16 or higher)
- [Tauri Dependencies](https://v2.tauri.app/start/prerequisites/)
```bash
git clone https://github.com/joncorv/filera.git
cd filera
npm install
npm run tauri dev

```
## Nixos Dev
If you're using nixos, run nix develop at the repo root and all dependencies will be met in a development shell.

## Planned
- Template Support
- Undo functionality
- Regex support
- Batch folder renaming
