#!/usr/bin/env bash

cd "$(dirname "$0")/.."

if ! [ -x "$(command -v cross)" ]; then
  cargo install cross
fi

platforms=(linux macos)
targets=(x86_64-unknown-linux-musl x86_64-apple-darwin)

set -e

version=$(grep ^version Cargo.toml | head -n 1 | sed -e 's/^version = "//' -e 's/"$//')
for ((i = 0; i < ${#platforms[@]}; ++i)); do
  platform=${platforms[i]}
  target=${targets[i]}
  echo "Packaging for $platform with target $target"
  echo "Building..."
  cross build --release --target=$target
  echo "Creating tar file..."
  mkdir pkg-node-manager
  cp target/${target}/release/node-manager pkg-node-manager/
  cp docker-compose.yml pkg-node-manager/
  cp -r config pkg-node-manager/
  cp README.md pkg-node-manager/
  tar czf pkg-$platform-$version.tar.gz pkg-node-manager
  rm -rf pkg-node-manager
  echo "Done packaging for $platform"
  echo ""
done
