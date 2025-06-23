#!/usr/bin/env bash
set -euo pipefail

VERSION=$(grep '^version =' Cargo.toml | head -1 | cut -d '"' -f2)
TARGETS=("x86_64-unknown-linux-musl" "x86_64-pc-windows-gnu" "x86_64-apple-darwin")
ARTIFACTS=()

for TARGET in "${TARGETS[@]}"; do
    rustup target add "$TARGET"
    cargo build --release --target "$TARGET"
    BIN_NAME=giglizip-compress
    EXT="tar.gz"
    PKG_NAME="giglizip-compress-v${VERSION}-${TARGET}.${EXT}"
    mkdir -p dist
    cp "target/${TARGET}/release/${BIN_NAME}" "dist/"
    strip "dist/${BIN_NAME}" || true
    tar -czvf "dist/${PKG_NAME}" -C dist "${BIN_NAME}"
    ARTIFACTS+=("dist/${PKG_NAME}")
    rm "dist/${BIN_NAME}"
done

cd dist
sha256sum *.tar.gz > checksums.txt
cd ..

gh release create "v${VERSION}" dist/*.tar.gz dist/checksums.txt --title "giglizip-compress v${VERSION}" --notes "Automated release."
