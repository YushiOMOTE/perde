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
* Supports various types including dataclasses, generic types, enum and common built-in types.
* Supports various serialization formats. By design, `perde` can support as many format as `serde` can.
* Provides string case conversion of field names, skipping serialization/deserialization options, structure flattening.
* Precise type checking based on type hints.
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
>>> perde.msgpack.dumps(A(10, "x"))
b'\x82\xa1a\n\xa1b\xa1x'
>>> perde.msgpack.loads_as(A, b'\x82\xa1a\n\xa1b\xa1x')
A(a=10, b='x')

```

### Supported formats

* [x] JSON (`perde.json`)
* [x] YAML (`perde.yaml`)
* [x] MessagePack (`perde.msgpack`)
* [x] TOML (`perde.toml`)
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
* Primitive types
    * `int`
    * `str`
    * `float`
    * `bool`
    * `bytes`
    * `bytearray`
* Generic types
    * `dict` /`typing.Dict`
    * `list` / `typing.List`
    * `set` / `typing.Set`
    * `frozenset` / `typing.FrozenSet`
    * `tuple` / `typing.Tuple`
    * `typing.Optional`
    * `typing.Union`
    * `typing.Any`
* Enum types
    * `Enum`
    * `IntEnum`
    * `Flag`
    * `IntFlag`
* More built-in types
    * `datetime.datetime`
    * `datetime.date`
    * `datetime.time`
    * `decimal.Decimal`
    * `uuid.UUID`

### Attributes

Attributes allow to modify the way of serialization/deserialization.

For example, to serialize/deserialize the field names as `camelCase`,

```python
>>> @perde.attr(rename_all="camelCase")
... @dataclass
... class A:
...     foo_bar: int
...     bar_bar: int

>>> perde.json.dumps(A(foo_bar=1, bar_bar=2))
'{"fooBar":1,"barBar":2}'
>>> perde.json.loads_as(A, '{"fooBar":1,"barBar":2}')
A(foo_bar=1, bar_bar=2)

```

See [the book](https://yushiomote.github.io/perde/attributes.html) for more details.

### Benchmark

#### JSON

<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/json-pack-dict.svg?raw=true" width="480" /> <img src="https://github.com/YushiOMOTE/perde/blob/master/assets/json-unpack-dict.svg?raw=true" width="480" />

#### YAML

<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/yaml-pack-dict.svg?raw=true" width="480" /> <img src="https://github.com/YushiOMOTE/perde/blob/master/assets/yaml-unpack-dict.svg?raw=true" width="480" />

#### TOML

<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/toml-pack-dict.svg?raw=true" width="480" /> <img src="https://github.com/YushiOMOTE/perde/blob/master/assets/toml-unpack-dict.svg?raw=true" width="480" />

#### MessagePack

<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/msgpack-pack-dict.svg?raw=true" width="480" /> <img src="https://github.com/YushiOMOTE/perde/blob/master/assets/msgpack-unpack-dict.svg?raw=true" width="480" />
