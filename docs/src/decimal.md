# Decimal

<!--
>>> from dataclasses import dataclass, field
>>> import perde
>>> import decimal

-->


`decimal.Decimal` is serialized as string types.

To serialize `Decimal`,

```python
>>> perde.json.dumps(decimal.Decimal('3.14159265'))
'"3.14159265"'

```

To deserialize `Decimal`,

```python
>>> perde.json.loads_as(decimal.Decimal, '"3.14159265"')
Decimal('3.14159265')

```
