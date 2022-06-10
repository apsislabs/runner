#!/bin/bash
# note, this expects to be run from osx

rm -rf target

VERSION=$(cargo run -- --version | sed 's/runner //')
mkdir -p releases/$VERSION

cargo build --release
cp target/release/runner releases/$VERSION/runner-$VERSION-x86_64-apple-darwin

cargo build --target=aarch64-apple-darwin --release
cp target/release/runner releases/$VERSION/runner-$VERSION-aarch64-apple-darwin

rm -rf target
docker build --tag runner-build:latest .
docker run -v $(pwd):/app runner-build:latest cargo build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/runner releases/$VERSION/runner-$VERSION-x86_64-unknown-linux-musl
