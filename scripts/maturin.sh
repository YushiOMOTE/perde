#!/bin/bash

set -eux

for d in "perde" "perde-json" "perde-msgpack" "perde-yaml"; do
	echo "Building/installing $d..."
	pushd $d
	maturin develop --release
	popd
done
