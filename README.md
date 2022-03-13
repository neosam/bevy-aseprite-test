# WASM build


cargo build --target wasm32-unknown-unknown --release

wasm-bindgen --out-dir doc/demo --out-name bevy-aseprite-test --target web --no-typescript target/wasm32-unknown-unknown/release/bevy-aseprite-test.wasm

cargo install -f wasm-bindgen-cli --version 0.2.79
