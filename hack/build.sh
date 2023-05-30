#!/bin/bash

echo "Current working directory is $(pwd)"
echo "PATH is $PATH"
echo "CARGO_HOME is $CARGO_HOME"

PROVIDER_ROOT=$(git rev-parse --show-toplevel)
COMMIT_HASH=$(git rev-parse --short HEAD 2>/dev/null)
DATE=$(date "+%Y-%m-%d")
BUILD_PLATFORM=$(uname -a | awk '{print tolower($1);}')

if [[ "$(pwd)" != "${PROVIDER_ROOT}" ]]; then
  echo "you are not in the root of the repo" 1>&2
  echo "please cd to ${PROVIDER_ROOT} before running this script" 1>&2
  exit 1
fi

RUST_BUILD_CMD="cargo build"
RUST_BUILD_FLAGS="--release"

if [[ -z "${PROVIDER_BUILD_PLATFORMS}" ]]; then
    PROVIDER_BUILD_PLATFORMS="unknown-linux-gnu pc-windows-msvc" # pc-windows-msvc apple-darwin"
fi

if [[ -z "${PROVIDER_BUILD_ARCHS}" ]]; then
    PROVIDER_BUILD_ARCHS="x86_64 aarch64"
fi

# Create the release directory
mkdir -p "${PROVIDER_ROOT}/release"

cargo install cross --git https://github.com/cross-rs/cross

for OS in ${PROVIDER_BUILD_PLATFORMS[@]}; do
  for ARCH in ${PROVIDER_BUILD_ARCHS[@]}; do
    NAME="devpod-provider-hetzner-${OS}-${ARCH}"
    if [[ "${OS}" == "pc-windows-msvc" ]]; then
      NAME="${NAME}.exe"
    fi

    if [[ "${ARCH}" == "aarch64" && "${OS}" == "pc-windows-msvc" ]]; then
        echo "Building for ${OS}/${ARCH} not supported."
        continue
    fi

    echo "Building for ${OS}/${ARCH}"
    cross build --target ${ARCH}-${OS} --release
    #rustup target add ${ARCH}-${OS}
    #RUSTFLAGS="--target ${ARCH}-${OS}" ${RUST_BUILD_CMD} ${RUST_BUILD_FLAGS}
    cp target/${ARCH}-${OS}/release/devpod-provider-hetzner "${PROVIDER_ROOT}/release/${NAME}"
    shasum -a 256 "${PROVIDER_ROOT}/release/${NAME}" | cut -d ' ' -f 1 > "${PROVIDER_ROOT}/release/${NAME}".sha256
  done
done
