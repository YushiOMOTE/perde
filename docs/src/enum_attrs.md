# Enum attributes

<!--
>>> from dataclasses import dataclass, field
>>> import perde
>>> import enum

-->

The following attributes can be set with enum. For example,

```python
>>> @perde.attr(rename="B")
... class A(enum.Enum):
...     X = 1
...     Y = 2

```

* `rename = "name"`
    * Serialize and deserialize enums with the given name instead of the name in Python.
* `rename_all = "string_case"`
    * Convert the case of all the members in the enum.
    * The possible values are the same as ones for `class`.
    * This option is ignored when `as_value` is set.
* `rename_all_serialize = "string_case"`
    * Convert the string case only when serialization.
* `rename_all_deserialize = "string_case"`
    * Convert the string case only when deserialization.
* `as_value = True`
    * Serialize and deserialize enum using the enum value instead of the name.
