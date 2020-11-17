# Testing status

## How this works?



## Types

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
    * [ ] `datetime.datetime`
    * [ ] `datetime.date`
    * [ ] `datetime.time`
    * [ ] `decimal.Decimal`
    * [ ] `uuid.UUID`

## Attribute testing

#### Class attributes

* [ ] `rename`
* [x] `rename_all`
* [x] `rename_all_serialize`
* [x] `rename_all_deserialize`
* [x] `deny_unknown_fields`
* [x] `default`

#### Class fields attributes

* [x] `perde_rename: "name"`
* [x] `perde_default: True`
* [x] `default`
* [x] `default_factory`
* [x] `perde_flatten: True`
* [x] `perde_skip: True`
* [x] `perde_skip_serializing: True`
* [x] `perde_skip_deserialzing: True`

#### Enum attributes

* [ ] `rename`
* [x] `rename_all`
* [x] `rename_all_serialize`
* [x] `rename_all_deserialize`
* [x] `as_value`

#### Enum member attributes

* [x] `perde_rename`
* [x] `perde_skip`
* [x] `perde_skip_serializing`
* [x] `perde_skip_deserialzing`
* [x] `perde_other`

## Known issues / Constraints

* Flatten for msgpack doesn't work due to [the issue](https://github.com/3Hren/msgpack-rust/issues/196) in `rmp-serde`.
