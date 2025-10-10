# Filera 🗂️

A powerful, cross-platform batch file renaming tool written in Rust. Transform messy, disorganized filenames into clean, organized ones with ease.

![filera app ui](src/assets/app_ui.png?raw=true "Filera App UI")

> **⚠️ WORK IN PROGRESS**: This project is under active development. Features, documentation, and functionality are subject to change without notice. Use at your own discretion.

> **Note**: Filera is currently in **Beta**. While functional, there are known issues being addressed before the first stable release.

## 🚀 Features

- **Batch Rename**: Process thousands of files at once
- **Cross-Platform**: Works seamlessly on Windows, macOS, and Linux
- **Fast & Lightweight**: Built with Rust backend for optimal performance
- **Modern UI**: Clean, intuitive interface built with Vue 3 and TypeScript
- **Safe Operations**: Preview changes before applying them

## 🎯 Why Filera?

Have you ever found yourself with a folder full of files with messy names like:
- `IMG_20231205_143022.jpg`
- `Document (1) copy.pdf`
- `untitled folder/New Text Document (3).txt`

Instead of spending hours renaming files manually, Filera lets you batch process them all at once with powerful renaming patterns and rules.

## 🛠️ Tech Stack

- **Backend**: Rust with Tauri
- **Frontend**: Vue 3 + TypeScript
- **Cross-Platform**: Windows, macOS, Linux

## 📦 Installation

### Pre-built Releases
Download the latest beta release from the [Releases](https://github.com/joncorv/filera/releases) page.

## 🤝 Contributing

This is a **free and open-source** project - my way of giving back to the amazing open-source community that has provided so many incredible tools over the years.

Contributions are welcome! Whether you're:
- 🐛 Reporting bugs
- 💡 Suggesting features  
- 🔧 Fixing issues
- 📚 Improving documentation
- 🎨 Designing UI improvements

### How to Contribute
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 🔧 Development Setup

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (v16 or higher)
- [yarn](https://yarnpkg.com/)

### Getting Started
```bash
# Clone the repopsitory 
git clone https://github.com/joncorv/filera.git
cd filera

# Install frontend dependencies
yarn install

# Start development server
yarn run tauri dev

# Optionally, if you are on linux, run the custom dev script
./tauri_dev.sh
```

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

This project is licensed under an open-source Apache license.

## 🙏 Acknowledgments

Thank you to the open-source community for all the amazing tools and libraries that made this project possible. This is my small way of giving back.

---

**Made with ❤️ for the community**

*Have suggestions or found a bug? [Open an issue](https://github.com/joncorv/filera/issues) - all feedback helps make Filera better!*
