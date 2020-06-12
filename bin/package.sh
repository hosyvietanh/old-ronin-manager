#!/usr/bin/env bash

if ! [ -x "$(command -v cross)" ]; then
  cargo install cross
fi

targets=(x86_64-unknown-linux-musl x86_64-unknown-linux-gnu)

set -e

for target in "${targets[@]}"; do
  echo "Packaging for $target"
  echo "Building..."
  cross build --release --target=$target
  echo "Creating tar file..."
  mkdir chain-node-manager
  cp target/${target}/release/node-manager chain-node-manager/
  cp docker-compose.yml chain-node-manager/
  cp -r config chain-node-manager/
  cp README.md chain-node-manager/
  tar czf chain-node-manager-$target.tar.gz chain-node-manager
  rm -rf chain-node-manager
  echo "Done packaging for $target"
  echo ""
done
