# Contributing

## Getting Started

1. Fork and clone the repository
2. Build: `cargo build`
3. Test: `cargo test`
4. Submit a PR

## Guidelines

- Run `cargo fmt` before committing
- Ensure `cargo test` passes
- Add tests for new functionality
- Keep commits atomic and well-described

## Adding a New Shell Plugin

1. Create `src/plugins/<shell>.rs`
2. Add module to `src/plugins/mod.rs`
3. Add a public `install()` function
4. Add the shell to `plugins::install()` dispatch
5. Optionally add a shell script to `shells/`

## Code of Conduct

Be respectful and constructive.
