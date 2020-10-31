#!/bin/bash

set -eux

cmd=$1

for d in "perde" "perde-json" "perde-msgpack" "perde-yaml"; do
	echo "Building/installing $d..."
	pushd $d
  case $1 in
      "develop")
	        maturin develop --release;;
      "build")
          maturin build --release;;
      "publish")
          maturin publish;;
      "test-publish")
          maturin publish -r https://test.pypi.org/simple/;;
  esac
	popd
done
