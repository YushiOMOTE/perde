#!/bin/bash

set -ex

cmd=$1

for d in "perde" "perde-json" "perde-msgpack" "perde-yaml"; do
	  echo "Building/installing $d..."
	  pushd $d
    case $1 in
        "develop")
            if [ "$2" != "" ] && [ "$2" != "$d" ]; then
                popd
                continue
            fi
	          maturin develop --release;;
        "build")
            maturin build --release;;
        "publish")
            maturin publish -u $PYPI_USER -p $PYPI_PASSWORD;;
        "test-publish")
            maturin publish -u $TEST_PYPI_USER -p $TEST_PYPI_PASSWORD -r https://test.pypi.org/legacy/;;
        "link-readme")
            ln -s ../README.md README.md
    esac
	  popd
done
