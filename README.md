# Unreact

A static site generation framework for Rust using Handlebars and Scss.

Submit issue [here](https://github.com/darccyy/unreact/issues/new)

# Usage

For a quick start, check out [Unreact Template](https://github.com/darccyy/unreact-template)

## Development

```toml
unreact = {version = "*"}
```

```bash
cargo run -- --dev
```

## Production

```toml
unreact = {version = "*", default-features = false}
```

```bash
cargo run
```
