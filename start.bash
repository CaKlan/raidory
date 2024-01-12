rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "test_bevy" \
    ./target/wasm32-unknown-unknown/release/test_bevy.wasm
cp out/test_bevy_bg.wasm assets/test_bevy_bg.wasm
cp out/test_bevy.js assets/test_bevy.js
basic-http-server ./assets -a 127.0.0.1:4000

# cargo build --target wasm32-unknown-unknown
# cp target/wasm32-unknown-unknown/debug/test_bevy.wasm js/test_bevy.wasm
# cd js
# cargo install basic-http-server # optional, any http server will works
# basic-http-server . # or any other http server to server static files with wasm MIME