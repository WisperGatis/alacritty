# Alacritty Terminal Emulator

## Project Overview

Alacritty is a fast, cross-platform, OpenGL terminal emulator written in Rust. It's a modern terminal emulator with sensible defaults that allows for extensive configuration. By integrating with other applications, rather than reimplementing their functionality, it manages to provide a flexible set of features with high performance. The supported platforms currently consist of BSD, Linux, macOS and Windows.

The project is structured as a Rust workspace with the following main crates:
- `alacritty`: The main application crate.
- `alacritty_terminal`: The core terminal emulation library.
- `alacritty_config`: The configuration loading and management library.
- `alacritty_config_derive`: A derive macro for the configuration.

## Building and Running

### Building

To build the project, run the following command from the root of the repository:

```bash
cargo build --release
```

### Running

To run the application, you can use the following command:

```bash
cargo run
```

### Testing

To run the test suite, use the following command:

```bash
cargo test
```

## Development Conventions

### Code Style

The project follows the standard Rust formatting guidelines. To format the code, run:

```bash
cargo fmt
```

### Linting

The project uses `clippy` for linting. To run the linter, use the following command:

```bash
cargo clippy --all-targets
```

### Contributing

Contributions are welcome. Please follow the guidelines in the `CONTRIBUTING.md` file. Before submitting a pull request, make sure to run the tests and the linter.
