import enum
from dataclasses import dataclass, field
import pytest
import perde
from util import FORMATS, FORMATS_EXCEPT, FORMATS_ONLY


def test_error_perde():
    with pytest.raises(TypeError) as e:

        @perde.attr
        class X:
            pass

    assert e.value.args[0] == (
        "unsupported type "
        "`<class 'test_error.test_error_perde.<locals>.X'>`")

    with pytest.raises(TypeError) as e:

        @perde.attr()
        class Y:
            pass

    assert e.value.args[0] == (
        "unsupported type "
        "`<class 'test_error.test_error_perde.<locals>.Y'>`")


@pytest.mark.parametrize("m", FORMATS)
def test_error_no_args(m):
    with pytest.raises(TypeError) as e:
        m.dumps()
    assert e.value.args[
        0] == "dumps() requires 1 positional argument but got 0"

    with pytest.raises(TypeError) as e:
        m.loads()
    assert e.value.args[
        0] == "loads() requires 1 positional argument but got 0"

    with pytest.raises(TypeError) as e:
        m.loads_as()
    assert e.value.args[
        0] == "loads_as() requires 2 positional arguments but got 0"


@pytest.mark.parametrize("m", FORMATS)
def test_error_keyword_args(m):
    with pytest.raises(TypeError) as e:
        m.dumps(kw='a')
    assert e.value.args[0] == "dumps() takes no keyword arguments"

    with pytest.raises(TypeError) as e:
        m.loads(kw='a')
    assert e.value.args[0] == "loads() takes no keyword arguments"

    with pytest.raises(TypeError) as e:
        m.loads_as(kw='a')
    assert e.value.args[0] == "loads_as() takes no keyword arguments"


@pytest.mark.parametrize("m", FORMATS)
def test_error_dumps_wrong_number_of_args(m):
    with pytest.raises(TypeError) as e:
        m.dumps('a', 'b')
    assert e.value.args[
        0] == "dumps() requires 1 positional argument but got 2"

    with pytest.raises(TypeError) as e:
        m.loads('a', 'b')
    assert e.value.args[
        0] == "loads() requires 1 positional argument but got 2"

    with pytest.raises(TypeError) as e:
        m.loads_as('a')
    assert e.value.args[
        0] == "loads_as() requires 2 positional arguments but got 1"
    with pytest.raises(TypeError) as e:
        m.loads_as('a', 'b', 'c')
    assert e.value.args[
        0] == "loads_as() requires 2 positional arguments but got 3"


@pytest.mark.parametrize("m", FORMATS_EXCEPT("msgpack"))
def test_error_loads_invalid_argument_type(m):
    with pytest.raises(TypeError) as e:
        m.loads(b"a")
    assert e.value.args[
        0] == "invalid argument: expected `str` got `bytes`: b'a'"
    with pytest.raises(TypeError) as e:
        m.loads(any)
    assert e.value.args[0] == ("invalid argument: expected `str` "
                               "got `builtin_function_or_method`:"
                               " <built-in function any>")

    with pytest.raises(TypeError) as e:
        m.loads_as(str, b"a")
    assert e.value.args[
        0] == "invalid argument: expected `str` got `bytes`: b'a'"

    with pytest.raises(TypeError) as e:
        m.loads_as("b", "a")
    assert e.value.args[0] == "invalid argument: `b` is not a type"

    with pytest.raises(TypeError) as e:
        m.loads_as(any, "a")
    assert e.value.args[
        0] == "invalid argument: `<built-in function any>` is not a type"


@pytest.mark.parametrize("m", FORMATS_ONLY("msgpack"))
def test_error_loads_invalid_argument_type_msgpack(m):
    with pytest.raises(TypeError) as e:
        m.loads("a")
    assert e.value.args[0] == "invalid argument: expected `bytes` got `str`: a"


@pytest.mark.parametrize("m", FORMATS)
def test_error_invalid_class_attribute(m):
    for attr, ty in [("rename_all", "str"), ("rename_all_serialize", "str"),
                     ("rename_all_deserialize", "str"), ("rename", "str"),
                     ("deny_unknown_fields", "bool"), ("default", "bool")]:
        with pytest.raises(TypeError) as e:

            @perde.attr(**{attr: 3})
            @dataclass
            class A:
                pass

        assert e.value.args[
            0] == f"invalid attribute `{attr}`: expected `{ty}` got `int`: 3"

    for attr in [
            "rename_all", "rename_all_serialize", "rename_all_deserialize"
    ]:
        with pytest.raises(ValueError) as e:

            @perde.attr(**{attr: "hage"})
            @dataclass
            class B:
                pass

        assert e.value.args[
            0] == f"invalid attribute `{attr}`: invalid string case: `hage`"


@pytest.mark.parametrize("m", FORMATS)
def test_error_invalid_enum_attribute(m):
    for attr, ty in [("rename_all", "str"), ("rename_all_serialize", "str"),
                     ("rename_all_deserialize", "str"), ("rename", "str"),
                     ("as_value", "bool")]:
        with pytest.raises(TypeError) as e:

            @perde.attr(**{attr: 3})
            class A(enum.Enum):
                X = 10

        assert e.value.args[
            0] == f"invalid attribute `{attr}`: expected `{ty}` got `int`: 3"

    for attr in [
            "rename_all", "rename_all_serialize", "rename_all_deserialize"
    ]:
        with pytest.raises(ValueError) as e:

            @perde.attr(**{attr: "hage"})
            class B(enum.Enum):
                X = 10

        assert e.value.args[
            0] == f"invalid attribute `{attr}`: invalid string case: `hage`"


@pytest.mark.parametrize("m", FORMATS)
def test_error_invalid_class_field_attribute(m):
    for attr, ty in [("perde_flatten", "bool"), ("perde_rename", "str"),
                     ("perde_skip", "bool"),
                     ("perde_skip_serializing", "bool"),
                     ("perde_skip_deserializing", "bool"),
                     ("perde_default", "bool")]:
        with pytest.raises(TypeError) as e:

            @perde.attr()
            @dataclass
            class A:
                foo: int = field(metadata={attr: 3})

        assert e.value.args[
            0] == f"invalid attribute `{attr}`: expected `{ty}` got `int`: 3"


@pytest.mark.parametrize("m", FORMATS)
def test_error_invalid_enum_field_attribute(m):
    for attr, ty in [("perde_rename", "str"), ("perde_skip", "bool"),
                     ("perde_skip_serializing", "bool"),
                     ("perde_skip_deserializing", "bool"),
                     ("perde_other", "bool")]:
        with pytest.raises(TypeError) as e:

            @perde.attr()
            class A(perde.Enum):
                X = 100, {attr: 3}

        assert e.value.args[
            0] == f"invalid attribute `{attr}`: expected `{ty}` got `int`: 3"


@pytest.mark.parametrize("m", FORMATS)
def test_error_unsupported_type(m):
    class Abc:
        pass

    with pytest.raises(TypeError) as e:
        m.dumps(Abc())
    assert e.value.args[0] == (
        "invalid argument: unsupported type "
        "`<class 'test_error.test_error_unsupported_type.<locals>.Abc'>`")
