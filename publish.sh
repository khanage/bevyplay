#!/bin/bash

COMMIT_MSG="${1:-Pushing changes}"

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ../khanage.github.io/games/ \
    --out-name "pong" \
    ./target/wasm32-unknown-unknown/release/pong.wasm

