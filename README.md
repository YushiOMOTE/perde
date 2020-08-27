# serde-pyobj

Experimentally connecting Rust and Python for serialization/deserialization.

### Try it

```sh
# Build
cargo build --release --features "json,toml,yaml,msgpack"

# On macOS,
cp target/release/libserde_pyobj.dylib serde_pyobj.so
# On Linux,
cp target/release/libserde_pyobj.so serde_pyobj.so
# On Windows,
cp target/release/libserde_pyobj.dll serde_pyobj.pyd

# Use in python,
python
>>> import serde_pyobj
>>> serde_pyobj.msgpack_load(b'\x81\xA1\x61\x21')
{'a': 33}
>>> serde_pyobj.json_load('{"a": "bbb"}')
{'a': 'bbb'}
>>> serde_pyobj.yaml_load('a: 3232')
{'a': 3232}
>>> serde_pyobj.toml_load('a = "unko"')
{'a': 'unko'}
>>> quit()
```
