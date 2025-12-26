# wasi_hello_world

A minimal Rust + WASI demo that writes "Hello world!" into a local file.

## Requirements

- Rust toolchain (stable)
- Node.js 22+

## Local Run

This uses a local relative path and writes to `helloworld/helloworld.txt`.

```bash
cargo run
```

## WASI Build (wasip1)

```bash
rustup target add wasm32-wasip1
cargo build --target wasm32-wasip1
```

## WASI Run (Node.js)

The WASI sandbox requires a pre-opened directory for file writes. This project
pre-opens the current directory in `main.js`.

```bash
node main.js
```

## Output

- `helloworld/helloworld.txt`

## Notes

- For local runs, the `helloworld/` directory must exist.
- For WASI runs, the directory must be pre-opened (see `main.js`).
