#!/bin/sh

set -ex

cargo build -p gluon_wasm --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/gluon_wasm.wasm --out-dir src/client/
(cd src/client && wasm2es6js gluon_wasm_wasm.wasm -o gluon_wasm_wasm.js --base64)
parcel src/client/index.html
