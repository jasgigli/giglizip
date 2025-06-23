#!/usr/bin/env bash
set -e

VERSION=$(grep '^version =' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
TARGETS=(
  x86_64-unknown-linux-musl
  x86_64-pc-windows-gnu
  x86_64-apple-darwin
)

rustup target add x86_64-unknown-linux-musl

for TRG in "${TARGETS[@]}"; do
  cargo build --release --target "${TRG}"
  BIN=target/${TRG}/release/giglizip-compress
  EXT=""
  BIN_FILE=giglizip-compress
  if [[ "${TRG}" == *windows* ]]; then EXT=.exe; BIN_FILE=giglizip-compress.exe; fi
  mkdir -p dist/${TRG}
  cp "${BIN}${EXT}" dist/${TRG}/"${BIN_FILE}"
  pushd dist/${TRG}
    tar czf giglizip-compress-${VERSION}-${TRG}.tar.gz "${BIN_FILE}"
  popd
done

gh release create "v${VERSION}" dist/* --title "giglizip-compress v${VERSION}" --notes "First stable release"
