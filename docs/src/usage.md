# Usage

<!--
>>> from dataclasses import dataclass, field
>>> import enum

-->

Install and import `perde`.

```sh
pip install perde
```

```python
>>> import perde

```

Assume you have a dataclass,

```python
>>> @dataclass
... class A:
...     a: int
...     b: str

```

To serialize class `A` to JSON,

```python
>>> perde.json.dumps(A(a=10, b='x'))
'{"a":10,"b":"x"}'

```

To deserialize JSON to class `A`,

```python
>>> perde.json.loads_as(A, '{"a":10,"b":"x"}')
A(a=10, b='x')

```

To deserialize JSON to a dictionary,

```python
>>> perde.json.loads('{"a":10,"b":"x"}')
{'a': 10, 'b': 'x'}

```

More formats are supported.

```python
>>> perde.yaml.dumps(A(10, "x"))
'---\na: 10\nb: x'
>>> perde.yaml.loads_as(A, '---\na: 10\nb: x')
A(a=10, b='x')

```

```python
>>> perde.msgpack.dumps(A(10, "x"))
b'\x82\xa1a\n\xa1b\xa1x'
>>> perde.msgpack.loads_as(A, b'\x82\xa1a\n\xa1b\xa1x')
A(a=10, b='x')

```

### Supported formats

* [x] JSON (`perde.json`)
* [x] YAML (`perde.yaml`)
* [x] MessagePack (`perde.msgpack`)
* [x] TOML (`perde.toml`)
* [ ] CBOR
* [ ] Pickle
* [ ] RON
* [ ] BSON
* [ ] Avro
* [ ] JSON5
* [ ] Postcard
* [ ] URL
* [ ] Environment variables
* [ ] AWS Parameter Store
* [ ] S-expressions
* [ ] D-Bus
* [ ] FlexBuffer
* [ ] XML

All the formats provide the three methods:

* `dumps(objects)`: Serialize `objects` in the format.
* `loads(data)`: Deserialize `data` to python objects.
* `loads_as(type, input)`: Deserialize `data` to python objects as specified `type`.
