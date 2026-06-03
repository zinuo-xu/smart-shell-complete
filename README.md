# smart-shell-complete

Shell autocomplete that learns from your history and predicts your next command.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![CI](https://github.com/zinuo-xu/smart-shell-complete/actions/workflows/ci.yml/badge.svg)](https://github.com/zinuo-xu/smart-shell-complete/actions/workflows/ci.yml)

## Features

- **Learns from shell history** - Parses bash and zsh history files
- **Context-aware predictions** - Factors in current directory and command frequency
- **SQLite-backed storage** - All data stays local in a portable database
- **Command completion** - Tab-complete partial commands from learned history
- **Shell plugins** - Generate Fish and Zsh plugin configurations
- **Statistics** - View learning stats and top commands

## Quickstart

```bash
cargo install smart-shell-complete
smart-shell-complete learn
smart-shell-complete predict
```

## Commands

### learn

Parse shell history files and build the command database.

```bash
smart-shell-complete learn
```

Parses `~/.bash_history` and `~/.zsh_history` and stores commands in a local SQLite database at `~/.local/share/smart-shell-complete/commands.db`.

### predict

Show top command predictions based on current directory context.

```bash
smart-shell-complete predict
```

### complete

Complete a partial command prefix.

```bash
smart-shell-complete complete --prefix "git com"
```

### stats

Show learning statistics.

```bash
smart-shell-complete stats
```

### install

Generate and print shell plugin configuration.

```bash
smart-shell-complete install fish
smart-shell-complete install zsh
```

## Shell Integration

After running `smart-shell-complete learn`, you can integrate predictions into your shell:

### Fish

Add to `~/.config/fish/config.fish`:

```fish
function _smart_complete
    smart-shell-complete complete (commandline -ct)
end
```

### Zsh

Add to `~/.zshrc`:

```zsh
_smart_complete() {
    compadd $(smart-shell-complete complete "${(Q)words[CURRENT]}")
}
```

Or source the generated plugin scripts in `shells/`:

```bash
source shells/smart-complete.zsh   # for zsh
source shells/smart-complete.fish  # for fish
```

## Installation

### From source

```bash
git clone https://github.com/zinuo-xu/smart-shell-complete.git
cd smart-shell-complete
cargo install --path .
```

### From crates.io

```bash
cargo install smart-shell-complete
```

### Using install script

```bash
curl -fsSL https://raw.githubusercontent.com/zinuo-xu/smart-shell-complete/main/install.sh | bash
```

## Development

```bash
git clone https://github.com/zinuo-xu/smart-shell-complete.git
cd smart-shell-complete
cargo build
cargo test
```

## Project Structure

```
src/
  main.rs           # CLI entry point with clap
  engine/
    learner.rs      # Shell history parsing
    predictor.rs    # Command prediction and completion
    context.rs      # Context tracking (directory, time)
    chains.rs       # Command chain detection
  db/
    schema.rs       # SQLite schema and connection
    query.rs        # Database queries and stats
  plugins/
    mod.rs          # Plugin dispatcher
    fish.rs         # Fish shell plugin
    zsh.rs          # Zsh shell plugin
shells/             # Shell integration scripts
tests/              # Integration tests
```

## How It Works

1. **Learn**: Scans `~/.bash_history` and `~/.zsh_history`, extracts commands and their directory context, and stores them in a local SQLite database.
2. **Predict**: When you run `predict`, it queries the database for the most frequent commands used in your current directory.
3. **Complete**: When you run `complete --prefix "foo"`, it searches for learned commands matching the prefix.

## Database

The SQLite database is stored at `~/.local/share/smart-shell-complete/commands.db` on Linux/macOS.

Tables:
- `commands` - Stores command text, directory, timestamp, and frequency count
- `chains` - Stores command-to-command transition chains

## Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT (c) [zinuo-xu](https://github.com/zinuo-xu)
