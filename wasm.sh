#!/bin/sh

set -ex

CARGO_INCREMENTAL=0 cargo build -p gluon_wasm --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/gluon_wasm.wasm --out-dir target
wasm2es6js target/gluon_wasm_wasm.wasm -o src/client/gluon_wasm_wasm.js --base64
parcel build src/client/index.html
