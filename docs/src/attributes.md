# Attributes

<!--
>>> from dataclasses import dataclass, field
>>> import perde
>>> import enum

-->

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
