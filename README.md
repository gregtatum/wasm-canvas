## wasm-canvas

The following repo is an experiment on getting wasm running the browser.

## Requirements

wasm-bindgen must be on your path, and the same version as the dependency of this project.

`cargo install wasm-bindgen-cli`

To update run:

`cargo install -f wasm-bindgen-cli`

Probably also bump the versions in the Cargo.toml file.

## Running

There is no live re-loading, so run: `./build.sh` to run every time.
