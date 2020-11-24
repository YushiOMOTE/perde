# Enum

<!--
>>> from dataclasses import dataclass, field
>>> import perde, enum

-->

Enum types are serialized as the member names by default.

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
