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

1. [Install](#install)
2. [Usage](#usage)
3. [Supported formats](#formats)
4. [Supported types](#types)
    1. [Enum](#enum)
5. [Attributes](#attrs)
    1. [Dataclass attributes](#dataclass_attrs)
    2. [Dataclass field attributes](#dataclass_field_attrs)
    3. [Enum attributes](#enum_attrs)
    4. [Enum member attributes](#enum_mem_attrs)
6. [Benchmark](#benchmark)


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

### <a name="formats" /> Supported formats

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

### <a name="types" /> Supported types

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

### Enum

Enums are serialized as the member names.

```python
>>> class E(enum.Enum):
...     X = 10
...     Y = 'a'

>>> perde.json.dumps(E.X)
'"X"'
>>> perde.json.loads_as(E, '"Y"')
<E.Y: 'a'>

```

By using `as_value` attribute, they are serialized as the member values.

```python
>>> @perde.attr(as_value=True)
... class F(enum.Enum):
...     X = 10
...     Y = 'a'

>>> perde.json.dumps(F.X)
'10'
>>> perde.json.loads_as(F, '"a"')
<F.Y: 'a'>

```

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

For another example, to skip serializing the field,

```python
>>> @dataclass
... class A:
...     foo_bar: int
...     bar_bar: int = field(metadata = {"perde_skip": True})

>>> perde.json.dumps(A(foo_bar=1, bar_bar=2))
'{"foo_bar":1}'

```

Attributes can be used with enum as well.
 
```python
>>> @perde.attr(rename_all = "snake_case")
... class A(enum.Enum):
...     FooBar = 1
...     BarBar = 2

>>> perde.json.dumps(A.BarBar)
'"bar_bar"'
>>> perde.json.loads_as(A, '"foo_bar"')
<A.FooBar: 1>

```

To use attributes for enum members, inherit `perde.Enum`/`perde.IntEnum` instead of `enum.Enum`/`enum.IntEnum`.

```python
>>> class A(perde.Enum):
...     FooBar = 1, {"perde_rename": "BooBoo"}
...     BarBar = 2

>>> perde.json.dumps(A.FooBar)
'"BooBoo"'
>>> perde.json.loads_as(A, '"BooBoo"')
<A.FooBar: 1>

```

#### Dataclass attributes

The following attributes can be set with `dataclass`. For example,

```python
>>> @perde.attr(rename="B")
... @dataclass
... class A:
...     a: int
...     b: str

```

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

#### Dataclass field attributes

The following attributes can be set with fields in `dataclass`. For example,

```python
>>> @dataclass
... class A:
...     a: int
...     b: str = field(metadata = {"perde_skip": True})

```

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

The following attributes can be set with enum. For example,

```python
>>> @perde.attr(rename="B")
... class A(enum.Enum):
...     X = 1
...     Y = 2

```

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

The following attributes can be set with enum members. For example,

```python
>>> class A(perde.Enum):
...     X = 1, {"rename": "Z"}
...     Y = 2

```

Note that `perde.Enum`/`perde.IntEnum` needs to be used instead of `enum.Enum`/`enum.IntEnum`.

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

### Supported Python

* Interpreters (CPython)
    * 3.7
    * 3.8
    * 3.9
* Platforms
    * Linux
    * MacOS
    * Windows

### Benchmark

#### JSON

<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/json-pack-dict.svg?raw=true" width="480" /> <img src="https://github.com/YushiOMOTE/perde/blob/master/assets/json-unpack-dict.svg?raw=true" width="480" />

#### YAML

<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/yaml-pack-dict.svg?raw=true" width="480" /> <img src="https://github.com/YushiOMOTE/perde/blob/master/assets/yaml-unpack-dict.svg?raw=true" width="480" />

#### TOML

<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/toml-pack-dict.svg?raw=true" width="480" /> <img src="https://github.com/YushiOMOTE/perde/blob/master/assets/toml-unpack-dict.svg?raw=true" width="480" />

#### MessagePack

<img src="https://github.com/YushiOMOTE/perde/blob/master/assets/msgpack-pack-dict.svg?raw=true" width="480" /> <img src="https://github.com/YushiOMOTE/perde/blob/master/assets/msgpack-unpack-dict.svg?raw=true" width="480" />
