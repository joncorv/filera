# Filera 🗂️

> **Note**: Filera is currently in **Alpha**. While functional, there are known issues being addressed before the first stable release.

A powerful, cross-platform batch file renaming tool built with Tauri. The backend is rust, and frontend in Vue/Typescript. Transform messy, disorganized filenames into clean, organized ones with ease.

## 🚀 Features

- **Batch Rename**: Process hundreds or thousands of files at once
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

## 🚧 Alpha Status

Filera is currently in alpha development. It's functional and ready for testing, but please be aware of these known issues:

- **File Handling**: Issues with completely blank/empty files
- **Light Mode**: UI currently optimized for dark mode only
- **Output Options**: Only in-place replacement available (no copy/move to directory options yet)
- **Branding**: Temporary logo and name (Filera is the planned final name)

## 🛠️ Tech Stack

- **Backend**: Rust with Tauri
- **Frontend**: Vue 3 + TypeScript
- **Cross-Platform**: Windows, macOS, Linux

## 📦 Installation

### Pre-built Releases

Download the latest alpha release from the [Releases](https://github.com/yourusername/filera/releases) page.

## 🔧 Development Setup

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (v16 or higher)
- [npm](https://www.npmjs.com)
- [yarn](https://yarnpkg.com/)

### Getting Started

```bash
# Install frontend dependencies
yarn install

# Start development server
yarn tauri dev
```

## 🤝 Contributing

This is a **free and open-source** project - my way of giving back to the amazing open-source community that has provided so many incredible tools over the years.

Contributions are welcome! Whether you're:

- 🐛 Reporting bugs
- 💡 Suggesting features
- 🔧 Fixing issues
- 📚 Improving documentation
- 🎨 Designing UI improvements

### Current Priority Issues

Help is especially appreciated with these alpha issues:

1. **File Handling**: Robust handling of empty/blank files
2. **Light Mode**: Making the UI beautiful in light theme
3. **Output Options**: Adding copy/move to directory functionality
4. **UI Polish**: General interface improvements

### How to Contribute

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📋 Roadmap

- [ ] Fix blank file handling
- [ ] Implement light mode
- [ ] Add output directory options
- [ ] Custom renaming patterns
- [ ] Undo functionality
- [ ] Regex support
- [ ] Batch folder renaming
- [ ] Templates system

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](https://claude.ai/chat/LICENSE) file for details.

## 🙏 Acknowledgments

Thank you to the open-source community for all the amazing tools and libraries that made this project possible. This is my small way of giving back.

---

**Made with ❤️ for the community**

_Have suggestions or found a bug? [Open an issue](https://github.com/yourusername/filera/issues) - all feedback helps make Filera better!_
