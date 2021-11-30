# trace-deps

[<img alt="Rust" src="https://img.shields.io/badge/rust-2021-a72145?logo=rust&style=flat" />](https://www.rust-lang.org/)

## Setup

Install Rust with [rustup](https://rustup.rs/).

### Clippy

For helpful linting rools, install [Clippy](https://github.com/rust-lang/rust-clippy) with `rustup`:

```sh
rustup component add clippy
```

Run it with `cargo`:

```sh
cargo clippy --fix
```

Configure the `rust-analyzer` VS Code plugin to use it (in _settings.json_):

```json
{
    "rust-analyzer.checkOnSave.command": "clippy"
}
```
