# 🪢 Knot Forget

> Tie a knot. Don't forget.

A lightweight, efficient, and durable Rust motor for time and reminder management.

## Getting Started

### Prerequisites

- **Rust** ≥ 1.94.0 — install via [rustup](https://rustup.rs)

### Build

1. Clone the repository:
   ```bash
   git clone https://github.com/Knot-Forget/knot-forget.git
   ```
2. Navigate into the project directory:
   ```bash
   cd knot-forget
   ```

Once inside the project directory, compile the library:

```bash
cargo build
```

### Test

Run the test suite to verify everything works as expected:

```bash
cargo test
```

## Development

Before opening a pull request, make sure the code passes both the linter and the formatter check:

```bash
cargo clippy --all-targets --all-features -- -D warnings   # lint
cargo fmt                                                   # format
```

## License

This project is licensed under the MIT License.
