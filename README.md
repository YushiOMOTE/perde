# perde: python-wrapped serde

### Heavily under construction towards 0.1.0 🎅

[![Project Status: WIP – Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![PyPi](https://img.shields.io/pypi/v/perde.svg)](https://pypi.python.org/pypi/perde)
[![Supported python versions](https://img.shields.io/pypi/pyversions/perde.svg)](https://pypi.org/project/perde/)
[![Actions Status](https://github.com/YushiOMOTE/perde/workflows/tests/badge.svg)](https://github.com/YushiOMOTE/perde/actions)
[![codecov](https://codecov.io/gh/yushiomote/perde/branch/master/graph/badge.svg)](https://codecov.io/gh/yushiomote/perde)
[![Coding style](https://badgen.net/badge/code%20style/black/000)](https://github.com/ambv/black)

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


### Usage

Assume you have a dataclass,

```python
>>> @dataclass
... class A:
...     a: int
...     b: str

```

To serialize/deserialize the object of `A` to/from JSON,

```python
>>> from perde import json

>>> json.dumps(A(a=10, b='x'))
'{"a":10,"b":"x"}'

>>> json.loads_as(A, '{"a":10,"b":"x"}')
A(a=10, b='x')

```

Also supports (de)serialization of non-dataclass objects.

```python
>>> json.dumps({'a': 10, 'b': 'x'})
'{"a":10,"b":"x"}'

>>> json.loads('{"a":10,"b":"x"}')
{'a': 10, 'b': 'x'}

```

More formats are supported.

```python
>>> from perde import yaml

>>> yaml.dumps(A(10, "x"))
'---\na: 10\nb: x'

>>> yaml.loads_as(A, '---\na: 10\nb: x')
A(a=10, b='x')

>>> yaml.loads('---\na: 10\nb: x')
{'a': 10, 'b': 'x'}

```

```python
>>> from perde import msgpack

>>> msgpack.dumps(A(10, "x"))
b'\x82\xa1a\n\xa1b\xa1x'

>>> msgpack.loads_as(A, b'\x82\xa1a\n\xa1b\xa1x')
A(a=10, b='x')

>>> msgpack.loads(b'\x82\xa1a\n\xa1b\xa1x')
{'a': 10, 'b': 'x'}

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

### Case conversion and more

To serialize/deserialize the field names in `camelCase`,

```python
>>> import perde

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

Supports more such features to configure the way of (de)serialization:

* Skips serializing/deserializing fields.
* Sets default values on deserialization.
* Flatten the content of structures.

See [the book](https://yushiomote.github.io/perde/attributes.html) for more details.

### Benchmark

<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/serialize_json_data_a.svg?raw=true" width="320px" />
<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/deserialize_json_data_a.svg?raw=true" width="320px" />

<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/serialize_msgpack_data_a.svg?raw=true" width="320px" />
<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/deserialize_msgpack_data_a.svg?raw=true" width="320px" />

The benchmark repeats (de)serializing the data structure `A` 10000 times:

```python
class A:
    a: int
    b: str
    c: float
    d: bool
```

The libraries in the benchmark:

* `perde`: This library.
* [pyserde](https://github.com/yukinarit/pyserde): Yet another serialization library on top of dataclasses.
* [mashumaro](https://github.com/Fatal1ty/mashumaro): A fast and well tested serialization framework on top of dataclasses.
* [attrs](https://github.com/python-attrs/attrs): Python Classes Without Boilerplate.
* [cattrs](https://github.com/Tinche/cattrs): Complex custom class converters for attrs.
