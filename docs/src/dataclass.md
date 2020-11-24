# Dataclass

<!--
>>> from dataclasses import dataclass, field
>>> import perde

-->

`perde` supports serializing/deserializing `dataclass`. It's serialized to a map in the serialized format (e.g. JSON map).

To serialize the dataclass `A` to JSON,

```python
>>> @dataclass
... class A:
...     a: str
...     b: int

>>> perde.json.dumps(A("x", 10))
'{"a":"x","b":10}'

```

To deserialize JSON back to `A`,

```python
>>> perde.json.loads_as(A, '{"a":"x","b":10}')
A(a='x', b=10)

```

Nesting is allowed. To serialize the dataclass `B` which contains `A`,

```python
>>> @dataclass
... class B:
...     a: float
...     b: A

>>> perde.json.dumps(B(3.33, A("x", 10)))
'{"a":3.33,"b":{"a":"x","b":10}}'

```

To deserialize `B`,

```python
>>> perde.json.loads_as(B, '{"a":3.33,"b":{"a":"x","b":10}}')
B(a=3.33, b=A(a='x', b=10))

```
