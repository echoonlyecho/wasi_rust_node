# run_wasi.py
import os
import subprocess
import sys

WASM_PATH = "./target/wasm32-wasip1/debug/wasi_hello_world.wasm"

def main() -> None:
    if not os.path.exists(WASM_PATH):
        raise FileNotFoundError(f"wasm not found: {WASM_PATH}")

    # Use the wasmtime CLI to avoid Python embedding crashes on macOS.
    cmd = [
        "wasmtime",
        "run",
        "--dir",
        ".",
        WASM_PATH,
    ]
    subprocess.run(cmd, check=True)

if __name__ == "__main__":
    main()
