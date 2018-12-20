## wasm-canvas

The following repo is an experiment on getting wasm running the browser.

## Requirements

Run `yarn install`.

wasm-bindgen must be on your path, and the same version as the dependency of this project.

`cargo install wasm-bindgen-cli`

If the verisons don't match, it might be best to upgraded both the Cargo.toml file and the CLI tool to the current version.

Update the CLI tool: `cargo install -f wasm-bindgen-cli`

## Running

Run `yarn start` for a live reload server. Run `yarn build` to build a release version.
