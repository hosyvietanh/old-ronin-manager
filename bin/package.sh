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
  pkg_dir="pkg-ronin-manager-${version}"
  mkdir ${pkg_dir}
  cp target/${target}/release/ronin-manager ${pkg_dir}/
  cp docker-compose.yml ${pkg_dir}/
  cp -r config ${pkg_dir}/
  cp README.md ${pkg_dir}/
  tar czf pkg-$platform-$version.tar.gz ${pkg_dir}
  rm -rf ${pkg_dir}
  echo "Done packaging for $platform"
  echo ""
done
