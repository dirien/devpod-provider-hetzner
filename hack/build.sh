#!/usr/bin/env bash
set -e

PROVIDER_ROOT=$(git rev-parse --show-toplevel)
COMMIT_HASH=$(git rev-parse --short HEAD 2>/dev/null)
DATE=$(date "+%Y-%m-%d")
BUILD_PLATFORM=$(uname -a | awk '{print tolower($1);}')

echo "Current working directory is $(pwd)"

if [[ "$(pwd)" != "${PROVIDER_ROOT}" ]]; then
  echo "you are not in the root of the repo" 1>&2
  echo "please cd to ${PROVIDER_ROOT} before running this script" 1>&2
  exit 1
fi

# generate provider.yaml
go run -mod vendor "${PROVIDER_ROOT}/hack/provider/main.go" ${RELEASE_VERSION} > "${PROVIDER_ROOT}/release/provider.yaml"
