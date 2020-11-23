# Date/Time

<!--
>>> from dataclasses import dataclass, field
>>> import perde
>>> import datetime

-->

All `datetime.datetime`, `datetime.date` and `datetime.time` are serialized as string types formatted in ISO 8601.

To serialize `datetime`,

```python
>>> perde.json.dumps(datetime.datetime(2020, 10, 31, 10, 30, 40, 1234))
'"2020-10-31T10:30:40.001234"'

```

To deserialize `datetime`,

```python
>>> perde.json.loads_as(datetime.datetime, '"2020-10-31T10:30:40.001234"')
datetime.datetime(2020, 10, 31, 10, 30, 40, 1234)

```

To serialize `date`,

```python
>>> perde.json.dumps(datetime.date(2020, 10, 31))
'"2020-10-31"'

```

To deserialize `date`,

```python
>>> perde.json.loads_as(datetime.date, '"2020-10-31"')
datetime.date(2020, 10, 31)

```

To serialize `time`,

```python
>>> perde.json.dumps(datetime.time(10, 30, 40, 1234))
'"10:30:40.001234"'

```

To deserialize `time`,

```python
>>> perde.json.loads_as(datetime.time, '"10:30:40.001234"')
datetime.time(10, 30, 40, 1234)

```
