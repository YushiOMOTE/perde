# How to develop

This assumes both Python and Rust are already setup.

The project uses [`maturin`](https://pypi.org/project/maturin/) to build Rust/Python mixed packages.

To build and install those packages locally for development,

```sh
# Use `pipenv` for dependency management.
pip install pipenv
make
```

All the packages will be installed in the environment:

```sh
$ pipenv run pip list | grep perde
perde              0.0.2
```

# Projects

* `perde-core`: The Rust library to bind any Rust serializers with Python code.
    * Implements `Serialize` and `Deserialize` for Python objects.
    * Provides some helper functions for FFI.
* `perde`: The Python package for serialization.
    * Provides common serialization formats `json`, `yaml`, `toml`, `msgpack` etc.
