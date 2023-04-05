#!/bin/sh

PROFILE="release"
TARGET="wasm32-unknown-unknown"
WWW_DIR="dist"

mkdir -p $WWW_DIR

cargo build --target $TARGET --$PROFILE

cp target/$TARGET/$PROFILE/einstein_tiling.wasm $WWW_DIR/
