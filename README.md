# wasm-of-life

[Game of life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) implemented using WASM and Rust.

Demo: https://deaz.github.io/wasm-of-life/

![Demo](./game-of-life.gif)

## Build

* Install `nightly` rust toolchain and add `wasm32-unknown-unknown` target
* Run `./build.sh` to build wasm module

To check locally:
1. `cd ./html`
2. `python3 -m http.server`
3. Go to http://localhost:8000/
