#!/bin/sh
cargo build --target x86_64-pc-windows-msvc &&
cp target/x86_64-pc-windows-msvc/debug/pong.exe . &&
exec ./pong.exe "$@"

