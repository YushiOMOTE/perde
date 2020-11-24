# UUID

<!--
>>> from dataclasses import dataclass, field
>>> import perde
>>> import uuid

-->


`uuid.UUID` is serialized as string of hex digits in standard form.

To serialize `UUID`,

```python
>>> perde.json.dumps(uuid.UUID('a8098c1a-f86e-11da-bd1a-00112444be1e'))
'"a8098c1a-f86e-11da-bd1a-00112444be1e"'

```

To deserialize `UUID`,

```python
>>> perde.json.loads_as(uuid.UUID, '"a8098c1a-f86e-11da-bd1a-00112444be1e"')
UUID('a8098c1a-f86e-11da-bd1a-00112444be1e')

```
