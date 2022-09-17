rustup target add wasm32-unknown-unknown
cargo install trunk --force
cargo install wasm-bindgen-cli --force
cargo clean
trunk clean
trunk build
