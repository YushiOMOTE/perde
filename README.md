# perde: python-wrapped serde

*Work-in-progress towards 0.1.0 🎅*

[![Project Status: WIP – Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![PyPi](https://img.shields.io/pypi/v/perde.svg)](https://pypi.python.org/pypi/perde)
[![Supported python versions](https://img.shields.io/pypi/pyversions/perde.svg)](https://pypi.org/project/perde/)
[![Actions Status](https://github.com/YushiOMOTE/perde/workflows/Rust/badge.svg)](https://github.com/YushiOMOTE/perde/actions)

![](https://github.com/YushiOMOTE/perde/blob/master/assets/logo.png)

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
    * [ ] 3.8
    * [ ] 3.9
* Platforms
    * [x] Linux
    * [ ] MacOS
    * [ ] Windows

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
    * [ ] `IntEnum`
    * [ ] `Flag`
    * [ ] `IntFlag`
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
* Chrono types
    * [ ] `datetime`
    * [ ] `date`
    * [ ] `time`
    * [ ] `timedelta`

### Supported formats

* [x] JSON
* [x] YAML
* [x] MessagePack
* [ ] CBOR
* [ ] Toml
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
class E(Enum):
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
  bar_bar: int = field(metadata = {"skip": True})

perde_json.dumps(A(1, 2))
# -> {"FooBar": 1}
```

```python
@perde.attr(rename_all = "snake_case")
enum A(enum.Enum):
   FooBar: 1
   BarBar: 2

perde_json.dumps(A.BarBar)
# -> "bar_bar"
```

To set attributes for the members of `Enum` or `IntEnum`. Use `perde.Enum` or `perde.IntEnum`.

```python
enum A(perde.Enum):
   FooBar: 1, {"rename": "BooBoo"}
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
    * When deserialzing, any missing non-optional fields are set by its `default` or `default_factory`.
    * Raises an error if non-optional fields are missing `default` and `default_factory`.

#### Class fields attributes

* `perde_rename: "name"`
    * Serialize and deserialize the field with the given name instead of the name in Python.
* `perde_default: True`
    * When deserialzing, if the field is missing, the field is set by `default` or `default_factory`.
* `perde_flatten: True`
    * Flatten the content of this field.
    * The type of the field can be either `dataclass` or dictionary.
    * If the type is dictionary, all the remaining fields at that point of deserialization are consumed.
* `perde_skip: True`
    * Skip serializing or deserializing this field.
    * The field must have `default` or `default_factory`.
* `perde_skip_serializing: True`
    * Skip serialzing this field.
* `perde_skip_deserialzing: True`
    * Skip deserializing this field.
    * The field must have `default` or `default_factory`.

#### Enum attributes

* `rename = "name"`
    * Serialize and deserialize enums with the given name instead of the name in Python.
* `rename_all = "string_case"`
    * Convert the case of all the members in the enum.
    * The possible values are the same as ones for `class`.
    * This option is ignored when `as_value` is set.
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

```
json(de) -------------------
json      = [0.1892565581947565, 0.1789955347776413, 0.19771194644272327, 0.17869805544614792, 0.1817416027188301]
perde as  = [0.07256896048784256, 0.06387559697031975, 0.06289006397128105, 0.06492204032838345, 0.06444761715829372]
perde     = [0.03787072375416756, 0.03849206678569317, 0.03701256774365902, 0.03784210607409477, 0.03712223842740059]
ujson     = [0.03571947664022446, 0.0350071769207716, 0.035524480044841766, 0.03537473827600479, 0.03500896133482456]
orjson    = [0.024663090705871582, 0.026005828753113747, 0.025051748380064964, 0.0264505036175251, 0.024867044761776924]

yaml(de) -------------------
yaml      = [1.8657512124627829, 1.8705988600850105, 1.8599027246236801, 1.8804237693548203, 1.8527513016015291]
perde as  = [0.29090225137770176, 0.27482700906693935, 0.2708629425615072, 0.2854452319443226, 0.28280119970440865]
perde     = [0.22424191236495972, 0.2495588045567274, 0.22433684580028057, 0.22169128619134426, 0.22160297632217407]

msgpack(de) ----------------
msgpack   = [0.03487630747258663, 0.035033950582146645, 0.03426872752606869, 0.03444667346775532, 0.03443203307688236]
perde as  = [0.07079600915312767, 0.05985707975924015, 0.06260973773896694, 0.060033876448869705, 0.0608107578009367]
perde     = [0.03339817374944687, 0.033870622515678406, 0.033603109419345856, 0.034254319965839386, 0.034998660907149315]
```

```
json(ser) ------------------
json      = [0.2153916023671627, 0.20939842239022255, 0.2292985152453184, 0.20938796736299992, 0.20893244817852974]
ujson     = [0.04131609573960304, 0.04082906246185303, 0.04345548339188099, 0.040903979912400246, 0.04144351929426193]
perde     = [0.053302960470318794, 0.053485700860619545, 0.054095394909381866, 0.05770992115139961, 0.05336238816380501]
orjson    = [0.04534510709345341, 0.045184383168816566, 0.046133121475577354, 0.0456595029681921, 0.04615986533463001]

yaml(ser) ------------------
yaml      = [1.8657512124627829, 1.8705988600850105, 1.8599027246236801, 1.8804237693548203, 1.8527513016015291]
perde     = [0.01173756830394268, 0.011586908251047134, 0.011359155178070068, 0.011403439566493034, 0.013109922409057617]

msgpack(ser) ---------------
msgpack   = [0.03487630747258663, 0.035033950582146645, 0.03426872752606869, 0.03444667346775532, 0.03443203307688236]
perde     = [0.054882919415831566, 0.05104514956474304, 0.05093616619706154, 0.050708770751953125, 0.05338519997894764]
```

#### Benchmark note

* Deserialization
    * `perde`: Deserialize to `dict`. (non-dataclass)
    * `perde as`: Deserialize to `dataclass`.
    * Others: Deserialize to `dict`. (non-dataclass)
* Serialization
    * `perde`, `orjson`: Serialize `dataclasses`.
    * Others: Serialize `dict`.
