# Enum member attributes

<!--
>>> from dataclasses import dataclass, field
>>> import perde

-->

The following attributes can be set with enum members. For example,

```python
>>> class A(perde.Enum):
...     X = 1, {"rename": "Z"}
...     Y = 2

```

Note that `perde.Enum`/`perde.IntEnum` needs to be used instead of `enum.Enum`/`enum.IntEnum`.

* `perde_rename: "name"`
    * Serialize and deserialize the member with the given name instead of the name in Python.
    * This option is ignored when `as_value` is set.
* `perde_skip: True`
    * Never serialize or deserialize this member.
* `perde_skip_serializing: True`
    * Never serialize this member. Serializing this member raises an error.
* `perde_skip_deserialzing: True`
    * Never deserialize this member.
* `perde_other: True`
    * When deserializing, any unknown members result in this member.
    * This option is ignored when `as_value` is set.
