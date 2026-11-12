<p align="center">
  <img src="src-tauri/icons/128x128@2x.png" width="128" height="128" alt="Filera icon">
</p>

<h1 align="center">Filera</h1>

<p align="center">
  Rename, move, and organize files in bulk.<br>
  Simple rules. Live preview. Open source.
</p>

<p align="center">
  <a href="https://github.com/joncorv/filera/actions/workflows/test.yml"><img src="https://github.com/joncorv/filera/actions/workflows/test.yml/badge.svg" alt="Tests"></a>
  <a href="https://github.com/joncorv/filera/releases/latest"><img src="https://img.shields.io/github/v/release/joncorv/filera?display_name=release" alt="Release"></a>
  <a href="https://github.com/joncorv/filera/releases"><img src="https://img.shields.io/github/downloads/joncorv/filera/total" alt="Downloads"></a>
  <a href="https://github.com/joncorv/filera/releases/latest"><img src="https://img.shields.io/badge/platforms-macOS%20%7C%20Windows%20%7C%20Linux%20%7C%20NixOS-lightgrey" alt="Platforms"></a>
  <a href="https://tauri.app"><img src="https://img.shields.io/badge/built%20with-Tauri-24C8DB?logo=tauri" alt="Built with Tauri"></a>
</p>

![filera app ui](src/assets/app_ui.png?raw=true "Filera App UI")

## Features

### File Input
- Open individual files or folders (recursive)
- Drag & drop files onto the window
- Search / filter files by name
- Sort by name, date created, date modified, size, or type

### File Selection
- Click, Ctrl/Cmd+Click, or Shift+Click to select files
- Remove selected files from the list
- Clear all loaded files

### Transformation Tasks
Chain multiple tasks that apply in order with a live preview:

| Task | Description |
|------|-------------|
| **Find & Replace** | Replace text in filenames |
| **Number Sequence** | Append/prepend a numbered sequence with configurable padding and separator |
| **Custom Text** | Insert text at the start or end of filenames |
| **Change Case** | Convert filenames to lowercase or UPPERCASE |
| **Date** | Insert file modification date (configurable format and position) |
| **Time** | Insert file modification time (configurable separator and position) |
| **Clear All** | Strip all text from filenames, keeping only the extension |

### Filters
Narrow down which files are renamed:

| Filter | Description |
|--------|-------------|
| **Name Filter** | Include or exclude files matching a string |
| **File Type Filter** | Include or exclude files by extension |
| **Time Period Filter** | Filter by modification date range |
| **Time Filter** | Filter files older or newer than a date |
| **Size Filter** | Filter by file size (bytes through terabytes) |

### Output Options
- **Replace in place** — rename files at their current location
- **Copy to directory** — copy files with new names to a chosen output directory
- **Move to directory** — move and rename files to a chosen output directory

### Other
- Light / dark mode
- Cross-platform (Windows, macOS, Linux)

## Installation
Download from [Releases](https://github.com/joncorv/filera/releases)

## Building from Source

Requirements:
- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (v16 or higher)
- [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/)

```bash
git clone https://github.com/joncorv/filera.git
cd filera

# Install frontend dependencies
npm install

# Start development server
npm run tauri dev

# Build the app
npm run tauri build
```

### NixOS
Run `nix develop` at the repo root to drop into a development shell with all dependencies available.

## Roadmap
- [ ] Template support (save and reuse rename configurations)
- [ ] Undo functionality
- [ ] Regex support in Find & Replace
- [ ] Batch folder renaming
- [ ] Plugin system

## License
MIT
