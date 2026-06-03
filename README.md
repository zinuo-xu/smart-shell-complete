# 🧠 smart-shell-complete

> Shell autocomplete that learns from your history and predicts your next command

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![CI](https://github.com/zinuo-xu/smart-shell-complete/actions/workflows/ci.yml/badge.svg)](https://github.com/zinuo-xu/smart-shell-complete/actions/workflows/ci.yml)

## ✨ Features

- 📚 Learns from bash, zsh, and fish history
- 🎯 Context-aware predictions (directory, time, command chains)
- 💾 SQLite-backed local storage
- 🔌 Fish and Zsh plugin generators
- 🔒 All data stays local

## 🚀 Quickstart

```bash
cargo install smart-shell-complete
smart-shell-complete learn
smart-shell-complete install fish  # or zsh
```

## 📄 License

MIT (c) [zinuo-xu](https://github.com/zinuo-xu)
