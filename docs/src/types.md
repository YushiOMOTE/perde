# Supported types

<!--
>>> from dataclasses import dataclass, field
>>> import perde
>>> import typing

-->

`perde` supports the following types.

* Primitive types
    * `int`
    * `str`
    * `float`
    * `bool`
    * `bytes`
    * `bytearray`
* Generic types
    * `dict` / `typing.Dict`
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
* `dataclass`

## Deserialization

The supported types can be used to specify as which type the input is parsed.
They can be directly set to the first argument of `loads_as` methods, or can be the member type of `dataclass`.

### Directly set to `loads_as`

To parse a JSON array as `list`,

```python
>>> perde.json.loads_as(list, '[97, 98, 99]')
[97, 98, 99]

```

To parse a JSON array as `bytes`,

```python
>>> perde.json.loads_as(bytes, '[97, 98, 99]')
b'abc'

```

To parse a JSON array as a `set`,

```python
>>> perde.json.loads_as(typing.Set[int], '[97, 98, 99]')
{97, 98, 99}

```

### As a member of `dataclass`

```python
>>> @dataclass
... class A:
...     a: str
...     b: bytes
...     c: typing.Dict[str, int]

>>> perde.json.loads_as(A, '{"a": "x", "b": [97, 98, 99], "c": {"p": 4, "q": 5}}')
A(a='x', b=b'abc', c={'p': 4, 'q': 5})

```

Deserializing incompatible types raises an exception from the format module.

```python
>>> @dataclass
... class A:
...     a: int
...     b: str

>>> perde.json.loads_as(A, '{"a": 3, "b": 4}')
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
json.JsonError: invalid type: integer `4`, expected a string at line 1 column 15

```

## Serialization

The instances of the supported types can be serialized by `dumps` methods.

To serialize `list` to a JSON array,

```python
>>> perde.json.dumps([97, 98, 99])
'[97,98,99]'

```

To serialize `bytes` to a JSON array,

```python
>>> perde.json.dumps(b'abc')
'[97,98,99]'

```

To serialize `set` to a JSON array,

```python
>> perde.json.dumps({97, 98, 99})
'[97,98,99]'

```
