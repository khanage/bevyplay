#!/bin/bash

COMMIT_MSG="${1:-Pushing changes}"

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ./web/ \
    --out-name "pong" \
    ./target/wasm32-unknown-unknown/release/pong.wasm

pushd web

git commit -am "$COMMIT_MSG"
git push

popd

