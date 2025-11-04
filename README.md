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
<<<<<<< HEAD
- [Tauri Dependencies](https://v2.tauri.app/start/prerequisites/)
=======
- [npm](https://www.npmjs.com/)

### Getting Started
>>>>>>> 2e55abd (updated readme for yarn --> npm)
```bash
git clone https://github.com/joncorv/filera.git
cd filera
npm install
npm run tauri dev

<<<<<<< HEAD
=======
# Install frontend dependencies
npm install

# Start development server
npm run tauri dev

>>>>>>> 2e55abd (updated readme for yarn --> npm)
```
## Nixos Dev
If you're using nixos, run nix develop at the repo root and all dependencies will be met in a development shell.

## Planned
- Template Support
- Undo functionality
- Regex support
- Batch folder renaming

## 📋 Roadmap
- [x] Fix blank file handling
- [x] Implement light mode
- [x] Add output directory options
- [x] Impliment Open Folders with recursion
- [ ] Template Support
- [ ] Undo functionality
- [ ] Regex support
- [ ] Batch folder renaming
- [ ] Plugin system

## 📄 License

This project is licensed under an open-souce MIT License.

## 🙏 Acknowledgments

Thank you to the open-source community for all the amazing tools and libraries that made this project possible. This is my small way of giving back.

---

**Made with ❤️ for the community**

*Have suggestions or found a bug? [Open an issue](https://github.com/joncorv/filera/issues) - all feedback helps make Filera better!*
