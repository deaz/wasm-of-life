#!/usr/bin/env bash

cargo +nightly build --target wasm32-unknown-unknown --release

wasm-gc target/wasm32-unknown-unknown/release/wasm_test.wasm html/test.wasm
