# Dataclass attributes

<!--
>>> from dataclasses import dataclass, field
>>> import perde

-->

The following attributes can be set with `dataclass`. For example,

```python
>>> @perde.attr(rename="B")
... @dataclass
... class A:
...     a: int
...     b: str

```

* `rename = "name"`
    * Serialize and deserialize classes with the given name instead of the name in Python.
* `rename_all = "string_case"`
    * Convert the case of all the field names in the class.
    * The possible values for `"string_case"` are:
        * `lowercase`
        * `UPPERCASE`
        * `PascalCase`
        * `camelCase`
        * `snake_case`
        * `SCREAMING_SNAKE_CASE`
        * `kebab-case`
        * `SCREAMING-KEBAB-CASE`
* `rename_all_serialize = "string_case"`
    * Convert the string case only when serialization.
* `rename_all_deserialize = "string_case"`
    * Convert the string case only when deserialization.
* `deny_unknown_fields = True`
    * Raises an error on deserialization if the input contains unknown fields.
* `default = True`
    * When deserialzing, any missing fields in the class are created by their default constructors.
