#!/bin/sh

# set -e exits the bash file if a non-zero error code is returned by a command
# set -x lists out every command run.
set -ex
cd "$(dirname $0)"

# Build the `canvas.wasm` file using Cargo/rustc
cargo +nightly build --target wasm32-unknown-unknown

# Run the `wasm-bindgen` CLI tool to postprocess the wasm file emitted by the
# Rust compiler to emit the JS support glue that's necessary
mkdir -p ./target/js
wasm-bindgen ./target/wasm32-unknown-unknown/debug/canvas.wasm --out-dir ./target/js

# Finally, package everything up using Webpack and start a server so we can
# browse the result
yarn serve
