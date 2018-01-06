# wasm-of-life

Game of life implemented using WASM and Rust.

Demo: https://deaz.github.io/wasm-of-life/

## Build

* Install `nightly` rust toolchain and add `wasm32-unknown-unknown` target
* Run `./build.sh` to build wasm module

To check locally:
1. `cd ./html`
2. `python3 -m http.server`
3. Go to http://localhost:8000/
