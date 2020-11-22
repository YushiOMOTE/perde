# perde: python-wrapped serde

### Heavily under construction towards 0.1.0 🎅

[![Project Status: WIP – Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![PyPi](https://img.shields.io/pypi/v/perde.svg)](https://pypi.python.org/pypi/perde)
[![Supported python versions](https://img.shields.io/pypi/pyversions/perde.svg)](https://pypi.org/project/perde/)
[![Actions Status](https://github.com/YushiOMOTE/perde/workflows/tests/badge.svg)](https://github.com/YushiOMOTE/perde/actions)
[![codecov](https://codecov.io/gh/yushiomote/perde/branch/master/graph/badge.svg)](https://codecov.io/gh/yushiomote/perde)


![](https://github.com/YushiOMOTE/perde/blob/master/assets/logo.png?raw=true)

Python wrapper around [the powerful Rust serialization framework](https://github.com/serde-rs/serde).

* Serialization & deserialization of python data structures.
* Supports dataclasses and most generic types.
* Supports various format. By design, `perde` can support as many format as `serde` can.
* Provide case conversion of field names, skipping serialization/deserialization, structure flattening.
* Strict type checking.
* Very fast.

<!--
>>> from dataclasses import dataclass, field
>>> import enum

-->


### Install

```sh
pip install perde
```

### Usage

```python
>>> import perde

```

Assume you have a dataclass,

```python
>>> @dataclass
... class A:
...     a: int
...     b: str

```

To serialize class `A` to JSON,

```python
>>> perde.json.dumps(A(a=10, b='x'))
'{"a":10,"b":"x"}'

```

To deserialize JSON to class `A`,

```python
>>> perde.json.loads_as(A, '{"a":10,"b":"x"}')
A(a=10, b='x')

```

To deserialize JSON to a dictionary,

```python
>>> perde.json.loads('{"a":10,"b":"x"}')
{'a': 10, 'b': 'x'}

```

More formats are supported.

```python
>>> perde.yaml.dumps(A(10, "x"))
'---\na: 10\nb: x'
>>> perde.yaml.loads_as(A, '---\na: 10\nb: x')
A(a=10, b='x')

```

```python
>>> perde.msgpack.dumps(A(10, "x"))
b'\x82\xa1a\n\xa1b\xa1x'
>>> perde.msgpack.loads_as(A, b'\x82\xa1a\n\xa1b\xa1x')
A(a=10, b='x')

```

### Supported formats

* [x] JSON
* [x] YAML
* [x] MessagePack
* [x] TOML
* [ ] CBOR
* [ ] Pickle
* [ ] RON
* [ ] BSON
* [ ] Avro
* [ ] JSON5
* [ ] Postcard
* [ ] URL
* [ ] Environment variables
* [ ] AWS Parameter Store
* [ ] S-expressions
* [ ] D-Bus
* [ ] FlexBuffer
* [ ] XML

### Supported types

* `dataclass`
* Generic types (`typing`)
    * `Dict`
    * `List`
    * `Set`
    * `FrozenSet`
    * `Tuple` / `Tuple[()]`
    * `Optional`
    * `Union`
    * `Any`
* Enum types
    * `Enum`
    * `IntEnum`
    * `Flag`
    * `IntFlag`
* Built-in types
    * `int`
    * `str`
    * `float`
    * `bool`
    * `bytes`
    * `bytearray`
    * `dict`
    * `list`
    * `set`
    * `frozenset`
    * `tuple`
* More built-in types
    * `datetime.datetime`
    * `datetime.date`
    * `datetime.time`
    * `decimal.Decimal`
    * `uuid.UUID`

### Benchmark

#### JSON

<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/json-pack-dict.svg?raw=true" width="480" /> <img src="https://github.com/YushiOMOTE/perde/blob/master/assets/json-unpack-dict.svg?raw=true" width="480" />

#### YAML

<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/yaml-pack-dict.svg?raw=true" width="480" /> <img src="https://github.com/YushiOMOTE/perde/blob/master/assets/yaml-unpack-dict.svg?raw=true" width="480" />

#### TOML

<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/toml-pack-dict.svg?raw=true" width="480" /> <img src="https://github.com/YushiOMOTE/perde/blob/master/assets/toml-unpack-dict.svg?raw=true" width="480" />

#### MessagePack

<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/msgpack-pack-dict.svg?raw=true" width="480" /> <img src="https://github.com/YushiOMOTE/perde/blob/master/assets/msgpack-unpack-dict.svg?raw=true" width="480" />
