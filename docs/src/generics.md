# Generic types

<!--
>>> from dataclasses import dataclass, field
>>> import perde
>>> import typing

-->

## Dictionary

The dictonary type like `dict`, `typing.Dict` corresponds to a map pattern in the serialized format (e.g. JSON map). `perde` supports the following form of dictionary types:

* Built-in `dict`
* `typing.Dict` without subscription
* `typing.Dict` with subscription

Using built-in `dict`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: dict

>>> perde.json.loads_as(A, '{"a": "x", "b": {"x": 3, "y": "hey", "z": true}}')
A(a='x', b={'x': 3, 'y': 'hey', 'z': True})

```

Using `typing.Dict` without subscription,

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.Dict

>>> perde.json.loads_as(A, '{"a": "x", "b": {"x": 3, "y": "hey", "z": true}}')
A(a='x', b={'x': 3, 'y': 'hey', 'z': True})

```

Using `typing.Dict` with subscription,

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.Dict[str, float]

>>> perde.json.loads_as(A, '{"a": "x", "b": {"x": 3.0, "y": 1.4, "z": 1.5}}')
A(a='x', b={'x': 3.0, 'y': 1.4, 'z': 1.5})

```

## List

The list type like `list`, `typing.List` corresponds to a list or array pattern in the serialized format (e.g. JSON array). `perde` supports the following form of list types:

* Built-in `list`
* `typing.List` without subscription
* `typing.List` with subscription

Using built-in `list`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: list

>>> perde.json.loads_as(A, '{"a": "x", "b": [1, 2, 3]}')
A(a='x', b=[1, 2, 3])

```

Using `typing.List` without subscription,

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.List

>>> perde.json.loads_as(A, '{"a": "x", "b": [1, 2, 3]}')
A(a='x', b=[1, 2, 3])

```

Using `typing.List` with subscription,

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.List[int]

>>> perde.json.loads_as(A, '{"a": "x", "b": [1, 2, 3]}')
A(a='x', b=[1, 2, 3])

```

## Set

The set type like `set`, `typing.Set` corresponds to a list or array pattern in the serialized format (e.g. JSON array). `perde` supports the following form of set types:

* Built-in `set`
* Built-in `frozenset`
* `typing.Set` with/without subscription
* `typing.FrozenSet` with/without subscription

Using built-in `set`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: set

>>> perde.json.loads_as(A, '{"a": "x", "b": [1, 2, 3]}')
A(a='x', b={1, 2, 3})

```

Using `typing.Set` without subscription,

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.Set

>>> perde.json.loads_as(A, '{"a": "x", "b": [1, 2, 3]}')
A(a='x', b={1, 2, 3})

```

Using `typing.Set` with subscription,

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.Set[int]

>>> perde.json.loads_as(A, '{"a": "x", "b": [1, 2, 3]}')
A(a='x', b={1, 2, 3})

```

`frozenset` and `typing.FrozenSet` work the same as `set` and `typing.Set`.

## Tuple

The tuple type like `tuple`, `typing.Tuple` corresponds to a list or array pattern in the serialized format (e.g. JSON array). `perde` supports the following form of set types:

* Built-in `tuple`
* `typing.Tuple` with/without subscription

Using built-in `tuple`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: tuple

>>> perde.json.loads_as(A, '{"a": "x", "b": [1, true, "hello"]}')
A(a='x', b=(1, True, 'hello'))

```

Using `typing.Tuple` without subscription,

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.Tuple

>>> perde.json.loads_as(A, '{"a": "x", "b": [1, true, "hello"]}')
A(a='x', b=(1, True, 'hello'))

```

Using `typing.Tuple` with subscription,

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.Tuple[int, bool, str]

>>> perde.json.loads_as(A, '{"a": "x", "b": [1, true, "hello"]}')
A(a='x', b=(1, True, 'hello'))

```

### Empty tuple

Use `typing.Tuple[()]` to explicitly specify the empty tuple.

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.Tuple[()]

>>> perde.json.loads_as(A, '{"a": "x", "b": []}')
A(a='x', b=())

```

## Optional

`typing.Optional` allows to parse the field optionally.

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.Optional[str]

>>> perde.json.loads_as(A, '{"a": "x"}')
A(a='x', b=None)

```

If the format supports `None` value (e.g. `null` in JSON), the `None` value is accepted.

```python
>>> perde.json.loads_as(A, '{"a": "x", "b": null}')
A(a='x', b=None)

```

Note that serialization results include `null` explicitly.

```python
>>> perde.json.dumps(A(a='x', b=None))
'{"a":"x","b":null}'

```

As the other generic types, `typing.Optional` without subscription is supported,

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.Optional

>>> perde.json.loads_as(A, '{"a": "x"}')
A(a='x', b=None)

```

## Union

`typing.Union` is used when there're multiple possible types for one field.

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.Union[str, int]

>>> perde.json.loads_as(A, '{"a": "x", "b": 3}')
A(a='x', b=3)

>>> perde.json.loads_as(A, '{"a": "x", "b": "three"}')
A(a='x', b='three')

```

As the other generic types, `typing.Union` without subscription is allowed but it accepts anything including `None` value, also making the field optional.

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.Union

>>> perde.json.loads_as(A, '{"a": "x", "b": "anything"}')
A(a='x', b='anything')

```

The field is optional.

```python
>>> perde.json.loads_as(A, '{"a": "x"}')
A(a='x', b=None)

```

The field accepts `None` value.

```python
>>> perde.json.loads_as(A, '{"a": "x", "b": null}')
A(a='x', b=None)

```

`typing.Union` cannot be used in schema-less formats which itself don't have type information.

## Any

`typing.Any` accepts any types including `None`, also making the field optional.

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.Any

>>> perde.json.loads_as(A, '{"a": "x", "b": "anything"}')
A(a='x', b='anything')

```

The field is optional.

```python
>>> perde.json.loads_as(A, '{"a": "x"}')
A(a='x', b=None)

```

The field accepts `None` value.

```python
>>> perde.json.loads_as(A, '{"a": "x", "b": null}')
A(a='x', b=None)

```

`typing.Optional` and `typing.Any` without subscription behave exactly same as `typing.Any`.
`typing.Any` cannot be used in schema-less formats which itself don't have type information.
