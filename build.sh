#!/bin/sh

PROFILE="release"
TARGET="wasm32-unknown-unknown"
WWW_DIR="docs"

mkdir -p $WWW_DIR

cargo build --target $TARGET --$PROFILE

wasm-bindgen \
    --out-dir $WWW_DIR \
    --no-typescript \
    --target web \
    target/$TARGET/$PROFILE/einstein_tiling.wasm
