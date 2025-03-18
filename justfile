serve-web:
    cargo build --target wasm32-unknown-unknown --profile wasm-release
    mkdir -p release
    cp -r assets release/
    cp index.html release/
    wasm-bindgen --out-dir release --target web ./target/wasm32-unknown-unknown/wasm-release/reptile.wasm
    npx serve release/
