# Dataclass field attributes

<!--
>>> from dataclasses import dataclass, field
>>> import perde

-->

The following attributes can be set with fields in `dataclass`. For example,

```python
>>> @dataclass
... class A:
...     a: int
...     b: str = field(metadata = {"perde_skip": True})

```

* `perde_rename: "name"`
    * Serialize and deserialize the field with the given name instead of the name in Python.
* `perde_default: True`
    * When deserialzing, if the field is missing, the field is created by its default constructor.
* `perde_flatten: True`
    * Flatten the content of this field.
    * The type of the field can be either `dataclass` or dictionary.
    * If the type is dictionary, all the remaining fields at that point of deserialization are consumed.
* `perde_skip: True`
    * Skip serializing or deserializing this field.
    * The field must have `default`/`default_factory`, or the `perde` attribute `default`/`perde_default` set.
* `perde_skip_serializing: True`
    * Skip serialzing this field.
* `perde_skip_deserialzing: True`
    * Skip deserializing this field.
    * The field must have `default`/`default_factory`, or the `perde` attribute `default`/`perde_default` set.

