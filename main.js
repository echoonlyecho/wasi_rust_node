const fs = require('fs');
const {WASI} = require('wasi');
const wasi = new WASI({
    args: process.argv,
    env: process.env,
    version: 'preview1',
    preopens: {
        '.': '.'
    }
});

const importObject = {
    wasi_snapshot_preview1: wasi.wasiImport
};

(async () => {
    const wasm = await WebAssembly.compile(
        fs.readFileSync('./target/wasm32-wasip1/debug/wasi_hello_world.wasm')
    );
    const instance = await WebAssembly.instantiate(wasm, importObject);

    wasi.start(instance);
})();
