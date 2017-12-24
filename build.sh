#!/usr/bin/env bash

cargo +nightly build --target wasm32-unknown-unknown --release

wasm-gc target/wasm32-unknown-unknown/release/wasm_of_life.wasm html/wasm_of_life.wasm
