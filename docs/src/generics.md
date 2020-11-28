# Generic types

<!--
>>> from dataclasses import dataclass, field
>>> import perde
>>> import typing

-->

## Dictionary

The dictionary types like `dict`, `typing.Dict` correspond to a map pattern in serialized format (e.g. JSON map). `perde` supports the following form of dictionary types:

* `dict`
* `typing.Dict`
* `typing.Dict[X, Y]`
* `dict[X]` (since Python 3.9)
* `dict[X, Y]` (since Python 3.9)

Using built-in `dict`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: dict

>>> perde.json.loads_as(A, '{"a": "x", "b": {"x": 3, "y": "hey", "z": true}}')
A(a='x', b={'x': 3, 'y': 'hey', 'z': True})

```

Using bare `typing.Dict`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.Dict

>>> perde.json.loads_as(A, '{"a": "x", "b": {"x": 3, "y": "hey", "z": true}}')
A(a='x', b={'x': 3, 'y': 'hey', 'z': True})

```

Using `typing.Dict[X, Y]`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.Dict[str, float]

>>> perde.json.loads_as(A, '{"a": "x", "b": {"x": 3.0, "y": 1.4, "z": 1.5}}')
A(a='x', b={'x': 3.0, 'y': 1.4, 'z': 1.5})

```

Using `dict[X, Y]`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: dict[str, float]

>>> perde.json.loads_as(A, '{"a": "x", "b": {"x": 3.0, "y": 1.4, "z": 1.5}}')
A(a='x', b={'x': 3.0, 'y': 1.4, 'z': 1.5})

```

## List

The list types like `list`, `typing.List` correspond to a list or array pattern in serialized format (e.g. JSON array). `perde` supports the following form of list types:

* `list`
* `typing.List`
* `typing.List[X]`
* `list[X]` (since Python 3.9)

Using built-in `list`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: list

>>> perde.json.loads_as(A, '{"a": "x", "b": [1, "a", 3.3]}')
A(a='x', b=[1, 'a', 3.3])

```

Using bare `typing.List`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.List

>>> perde.json.loads_as(A, '{"a": "x", "b": [1, 2, 3]}')
A(a='x', b=[1, 2, 3])

```

Using `typing.List[X]`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.List[int]

>>> perde.json.loads_as(A, '{"a": "x", "b": [1, 2, 3]}')
A(a='x', b=[1, 2, 3])

```

Using `list[X]`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: list[int]

>>> perde.json.loads_as(A, '{"a": "x", "b": [1, 2, 3]}')
A(a='x', b=[1, 2, 3])

```

## Set

The set types like `set`, `typing.Set` correspond to a list or array pattern in serialized format (e.g. JSON array). `perde` supports the following form of set types:

* `set` / `frozenset`
* `typing.Set` / `typing.FrozenSet`
* `typing.Set[X]` / `typing.FrozenSet[X]`
* `set[X]` / `frozenset[X]` (since Python 3.9)

Using built-in `set`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: set

>>> perde.json.loads_as(A, '{"a": "x", "b": [true, 2, 3]}')
A(a='x', b={True, 2, 3})

```

Using bare `typing.Set`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.Set

>>> perde.json.loads_as(A, '{"a": "x", "b": [true, 2, 3]}')
A(a='x', b={True, 2, 3})

```

Using `typing.Set[X]`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.Set[int]

>>> perde.json.loads_as(A, '{"a": "x", "b": [1, 2, 3]}')
A(a='x', b={1, 2, 3})

```

Using `set[X]`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: set[int]

>>> perde.json.loads_as(A, '{"a": "x", "b": [1, 2, 3]}')
A(a='x', b={1, 2, 3})

```

`frozenset` and `typing.FrozenSet` work the same as `set` and `typing.Set`.

## Tuple

The tuple types like `tuple`, `typing.Tuple` correspond to a list or array pattern in serialized format (e.g. JSON array). `perde` supports the following form of set types:

* `tuple`
* `typing.Tuple`
* `typing.Tuple[X, Y, ...]`
* `tuple[X, Y, ...]` (since Python 3.9)

Using built-in `tuple`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: tuple

>>> perde.json.loads_as(A, '{"a": "x", "b": [1, true, "hello"]}')
A(a='x', b=(1, True, 'hello'))

```

Using bare `typing.Tuple`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.Tuple

>>> perde.json.loads_as(A, '{"a": "x", "b": [1, true, "hello"]}')
A(a='x', b=(1, True, 'hello'))

```

Using `typing.Tuple[X, Y, ...]`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: typing.Tuple[int, bool, str]

>>> perde.json.loads_as(A, '{"a": "x", "b": [1, true, "hello"]}')
A(a='x', b=(1, True, 'hello'))

```

Using `tuple[X, Y, ...]`,

```python
>>> @dataclass
... class A:
...     a: str
...     b: tuple[int, bool, str]

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

`tuple[()]` is also available since Python 3.9.

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

Bare `typing.Union` accepts anything including `None` value, also making the field optional.

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

`typing.Union` cannot be used in schema-less formats which don't have type information themselves.

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

Bare `typing.Optional` and `typing.Any` behave exactly same as `typing.Any`.
`typing.Any` cannot be used in schema-less formats which don't have type information themselves.
