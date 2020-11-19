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

### Install

```sh
pip install perde-json
pip install perde-yaml
pip install perde-msgpack
```

### Usage

```python
import perde_json
import perde_yaml
import perde_msgpack

@dataclass
class A:
    key: int
    value: str

# Serialize objects into json, yaml, msgpack
perde_json.dumps(A(300, "json"))
perde_yaml.dumps(A(300, "yaml"))
perde_msgpack.dumps(A(300, "msgpack"))

# Deserialize as dataclasses
perde_json.loads_as(A, '{"key": 300, "value": "hoge"}')
perde_yaml.loads_as(A, '''key: 300
value: hoge
''')
perde_msgpack.loads_as(A, b'\x82\xA3\x6B\x65\x79\xCD\x01\x2C\xA5\x76\x61\x6C\x75\x65\xCD\x01\x90')

# Deserialize as objects
perde_json.loads_as(A, '{"key": 300, "value": "hoge"}')
perde_yaml.loads_as(A, '''key: 300
value: hoge
''')
perde_msgpack.loads_as(A, b'\x82\xA3\x6B\x65\x79\xCD\x01\x2C\xA5\x76\x61\x6C\x75\x65\xCD\x01\x90')
```

### Supported Python

* Interpreters (CPython)
    * [x] 3.7
    * [x] 3.8
    * [x] 3.9
* Platforms
    * [x] Linux
    * [x] MacOS
    * [x] Windows

### Supported types

* `dataclass`
* Generic types (`typing`)
    * [x] `Dict`
    * [x] `List`
    * [x] `Set`
    * [x] `FrozenSet`
    * [x] `Tuple` / `Tuple[()]`
    * [x] `Optional`
    * [x] `Union`
    * [x] `Any`
* Enum types
    * [x] `Enum`
    * [x] `IntEnum`
    * [x] `Flag`
    * [x] `IntFlag`
* Built-in types
    * [x] `int`
    * [x] `str`
    * [x] `float`
    * [x] `bool`
    * [x] `bytes`
    * [x] `bytearray`
    * [x] `dict`
    * [x] `list`
    * [x] `set`
    * [x] `frozenset`
    * [x] `tuple`
* More built-in types
    * [x] `datetime.datetime`
    * [x] `datetime.date`
    * [x] `datetime.time`
    * [x] `decimal.Decimal`
    * [x] `uuid.UUID`

### Supported formats

* [x] JSON
* [x] YAML
* [x] MessagePack
* [ ] CBOR
* [x] TOML
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

### Enums

Provides two ways of serialization.

1. Serialize and deserialize as `name`. This is the default.
2. Serialize and deserialize as `value`. Enabled by `as_value` attribute.

```python
@perde.attr(as_value = True)
class E(enum.Enum):
   A = 1
   B = 2

# This emits `1` instead of `"A"`.
perde_json.dumps(E.A)
```

### Attributes

Attributes can configure the behavior of serialization/deserialization.

```python
@perde.attr(rename_all = "camelCase")
class A:
  foo_bar: int
  bar_bar: int = field(metadata = {"perde_skip": True})

perde_json.dumps(A(1, 2))
# -> {"FooBar": 1}
```

```python
@perde.attr(rename_all = "snake_case")
class A(enum.Enum):
   FooBar: 1
   BarBar: 2

perde_json.dumps(A.BarBar)
# -> "bar_bar"
```

To set attributes for the members of `Enum` or `IntEnum`. Use `perde.Enum` or `perde.IntEnum` and add dictionaries after enum members.

```python
class A(perde.Enum):
   FooBar: 1, {"perde_rename": "BooBoo"}
   BarBar: 2

perde_json.dumps(A.FooBar)
# -> "BooBoo"
```

#### Class attributes

* `rename = "name"`
    * Serialize and deserialize classes with the given name instead of the name in Python.
* `rename_all = "string_case"`
    * Convert the case of all the field names in the class.
    * The possible values for `"string_case"` are:
        * `lowercase`
        * `UPPERCASE`
        * `PascalCase`
        * `camelCase`
        * `snake_case`
        * `SCREAMING_SNAKE_CASE`
        * `kebab-case`
        * `SCREAMING-KEBAB-CASE`
* `rename_all_serialize = "string_case"`
    * Convert the string case only when serialization.
* `rename_all_deserialize = "string_case"`
    * Convert the string case only when deserialization.
* `deny_unknown_fields = True`
    * Raises an error on deserialization if the input contains unknown fields.
* `default = True`
    * When deserialzing, any missing fields in the class are created by their default constructors.

#### Class fields attributes

* `perde_rename: "name"`
    * Serialize and deserialize the field with the given name instead of the name in Python.
* `perde_default: True`
    * When deserialzing, if the field is missing, the field is created by its default constructor.
* `perde_flatten: True`
    * Flatten the content of this field.
    * The type of the field can be either `dataclass` or dictionary.
    * If the type is dictionary, all the remaining fields at that point of deserialization are consumed.
* `perde_skip: True`
    * Skip serializing or deserializing this field.
    * The field must have `default`/`default_factory`, or the `perde` attribute `default`/`perde_default` set.
* `perde_skip_serializing: True`
    * Skip serialzing this field.
* `perde_skip_deserialzing: True`
    * Skip deserializing this field.
    * The field must have `default`/`default_factory`, or the `perde` attribute `default`/`perde_default` set.

#### Enum attributes

* `rename = "name"`
    * Serialize and deserialize enums with the given name instead of the name in Python.
* `rename_all = "string_case"`
    * Convert the case of all the members in the enum.
    * The possible values are the same as ones for `class`.
    * This option is ignored when `as_value` is set.
* `rename_all_serialize = "string_case"`
    * Convert the string case only when serialization.
* `rename_all_deserialize = "string_case"`
    * Convert the string case only when deserialization.
* `as_value = True`
    * Serialize and deserialize enum using the enum value instead of the name.

#### Enum member attributes

* `perde_rename: "name"`
    * Serialize and deserialize the member with the given name instead of the name in Python.
    * This option is ignored when `as_value` is set.
* `perde_skip: True`
    * Never serialize or deserialize this member.
* `perde_skip_serializing: True`
    * Never serialize this member. Serializing this member raises an error.
* `perde_skip_deserialzing: True`
    * Never deserialize this member.
* `perde_other: True`
    * When deserializing, any unknown members result in this member.
    * This option is ignored when `as_value` is set.

### Benchmark

#### JSON

<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/json-pack-dict.svg?raw=true" width="480" /> <img src="https://github.com/YushiOMOTE/perde/blob/master/assets/json-unpack-dict.svg?raw=true" width="480" />

#### YAML

<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/yaml-pack-dict.svg?raw=true" width="480" /> <img src="https://github.com/YushiOMOTE/perde/blob/master/assets/yaml-unpack-dict.svg?raw=true" width="480" />

#### TOML

<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/toml-pack-dict.svg?raw=true" width="480" /> <img src="https://github.com/YushiOMOTE/perde/blob/master/assets/toml-unpack-dict.svg?raw=true" width="480" />

#### MessagePack

<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/msgpack-pack-dict.svg?raw=true" width="480" /> <img src="https://github.com/YushiOMOTE/perde/blob/master/assets/msgpack-unpack-dict.svg?raw=true" width="480" />
