import enum
from dataclasses import dataclass, field
import pytest
import perde
import typing
from util import FORMATS, FORMATS_EXCEPT, FORMATS_ONLY


def test_error_perde():
    with pytest.raises(TypeError) as e:

        @perde.attr
        class X:
            pass

    assert e.value.args[0] == (
        "unsupported type " "`<class 'test_error.test_error_perde.<locals>.X'>`"
    )

    with pytest.raises(TypeError) as e:

        @perde.attr()
        class Y:
            pass

    assert e.value.args[0] == (
        "unsupported type " "`<class 'test_error.test_error_perde.<locals>.Y'>`"
    )


@pytest.mark.parametrize("m", FORMATS)
def test_error_no_args(m):
    with pytest.raises(TypeError) as e:
        m.dumps()
    assert e.value.args[0] == "dumps() requires 1 positional argument but got 0"

    with pytest.raises(TypeError) as e:
        m.loads()
    assert e.value.args[0] == "loads() requires 1 positional argument but got 0"

    with pytest.raises(TypeError) as e:
        m.loads_as()
    assert e.value.args[0] == "loads_as() requires 2 positional arguments but got 0"


@pytest.mark.parametrize("m", FORMATS)
def test_error_keyword_args(m):
    with pytest.raises(TypeError) as e:
        m.dumps(kw="a")
    assert e.value.args[0] == "dumps() takes no keyword arguments"

    with pytest.raises(TypeError) as e:
        m.loads(kw="a")
    assert e.value.args[0] == "loads() takes no keyword arguments"

    with pytest.raises(TypeError) as e:
        m.loads_as(kw="a")
    assert e.value.args[0] == "loads_as() takes no keyword arguments"


@pytest.mark.parametrize("m", FORMATS)
def test_error_dumps_wrong_number_of_args(m):
    with pytest.raises(TypeError) as e:
        m.dumps("a", "b")
    assert e.value.args[0] == "dumps() requires 1 positional argument but got 2"

    with pytest.raises(TypeError) as e:
        m.loads("a", "b")
    assert e.value.args[0] == "loads() requires 1 positional argument but got 2"

    with pytest.raises(TypeError) as e:
        m.loads_as("a")
    assert e.value.args[0] == "loads_as() requires 2 positional arguments but got 1"
    with pytest.raises(TypeError) as e:
        m.loads_as("a", "b", "c")
    assert e.value.args[0] == "loads_as() requires 2 positional arguments but got 3"


@pytest.mark.parametrize("m", FORMATS_EXCEPT("msgpack"))
def test_error_loads_invalid_argument_type(m):
    with pytest.raises(TypeError) as e:
        m.loads(b"a")
    assert e.value.args[0] == "invalid argument: expected `str` got `bytes`: b'a'"
    with pytest.raises(TypeError) as e:
        m.loads(any)
    assert e.value.args[0] == (
        "invalid argument: expected `str` "
        "got `builtin_function_or_method`:"
        " <built-in function any>"
    )

    with pytest.raises(TypeError) as e:
        m.loads_as(str, b"a")
    assert e.value.args[0] == "invalid argument: expected `str` got `bytes`: b'a'"

    with pytest.raises(TypeError) as e:
        m.loads_as("b", "a")
    assert e.value.args[0] == "invalid argument: `b` is not a type"

    with pytest.raises(TypeError) as e:
        m.loads_as(any, "a")
    assert (
        e.value.args[0] == "invalid argument: `<built-in function any>` is not a type"
    )


@pytest.mark.parametrize("m", FORMATS_ONLY("msgpack"))
def test_error_loads_invalid_argument_type_msgpack(m):
    with pytest.raises(TypeError) as e:
        m.loads("a")
    assert e.value.args[0] == "invalid argument: expected `bytes` got `str`: a"


@pytest.mark.parametrize("m", FORMATS)
def test_error_invalid_class_attribute(m):
    for attr, ty in [
        ("rename_all", "str"),
        ("rename_all_serialize", "str"),
        ("rename_all_deserialize", "str"),
        ("rename", "str"),
        ("deny_unknown_fields", "bool"),
        ("default", "bool"),
    ]:
        with pytest.raises(TypeError) as e:

            @perde.attr(**{attr: 3})
            @dataclass
            class A:
                pass

        assert (
            e.value.args[0]
            == f"invalid attribute `{attr}`: expected `{ty}` got `int`: 3"
        )

    for attr in ["rename_all", "rename_all_serialize", "rename_all_deserialize"]:
        with pytest.raises(ValueError) as e:

            @perde.attr(**{attr: "hage"})
            @dataclass
            class B:
                pass

        assert (
            e.value.args[0]
            == f"invalid attribute `{attr}`: invalid string case: `hage`"
        )


@pytest.mark.parametrize("m", FORMATS)
def test_error_invalid_enum_attribute(m):
    for attr, ty in [
        ("rename_all", "str"),
        ("rename_all_serialize", "str"),
        ("rename_all_deserialize", "str"),
        ("rename", "str"),
        ("as_value", "bool"),
    ]:
        with pytest.raises(TypeError) as e:

            @perde.attr(**{attr: 3})
            class A(enum.Enum):
                X = 10

        assert (
            e.value.args[0]
            == f"invalid attribute `{attr}`: expected `{ty}` got `int`: 3"
        )

    for attr in ["rename_all", "rename_all_serialize", "rename_all_deserialize"]:
        with pytest.raises(ValueError) as e:

            @perde.attr(**{attr: "hage"})
            class B(enum.Enum):
                X = 10

        assert (
            e.value.args[0]
            == f"invalid attribute `{attr}`: invalid string case: `hage`"
        )


@pytest.mark.parametrize("m", FORMATS)
def test_error_invalid_class_field_attribute(m):
    for attr, ty in [
        ("perde_flatten", "bool"),
        ("perde_rename", "str"),
        ("perde_skip", "bool"),
        ("perde_skip_serializing", "bool"),
        ("perde_skip_deserializing", "bool"),
        ("perde_default", "bool"),
    ]:
        with pytest.raises(TypeError) as e:

            @perde.attr()
            @dataclass
            class A:
                foo: int = field(metadata={attr: 3})

        assert (
            e.value.args[0]
            == f"invalid attribute `{attr}`: expected `{ty}` got `int`: 3"
        )


@pytest.mark.parametrize("m", FORMATS)
def test_error_invalid_enum_field_attribute(m):
    for attr, ty in [
        ("perde_rename", "str"),
        ("perde_skip", "bool"),
        ("perde_skip_serializing", "bool"),
        ("perde_skip_deserializing", "bool"),
        ("perde_other", "bool"),
    ]:
        with pytest.raises(TypeError) as e:

            @perde.attr()
            class A(perde.Enum):
                X = 100, {attr: 3}

        assert (
            e.value.args[0]
            == f"invalid attribute `{attr}`: expected `{ty}` got `int`: 3"
        )


@pytest.mark.parametrize("m", FORMATS)
def test_error_unsupported_type(m):
    class Abc:
        pass

    with pytest.raises(TypeError) as e:
        m.dumps(Abc())
    assert e.value.args[0] == (
        "invalid argument: unsupported type "
        "`<class 'test_error.test_error_unsupported_type.<locals>.Abc'>`"
    )


"""rust
#[derive(Serialize, Debug, new)]
struct TypeMismatch {
    a: String,
    b: Vec<u32>,
}

add!(TypeMismatch { "hage".into(), vec![1,2,3] });
"""


@pytest.mark.parametrize("m", FORMATS)
def test_error_decode_type_mismatch(m):
    @dataclass
    class TypeMismatch:
        a: str
        b: str

    with pytest.raises(m.errtype) as e:
        m.loads_as(TypeMismatch, m.data("TypeMismatch"))

    print(f"{m.name}: {e}")


"""rust
#[derive(Serialize, Debug, new)]
struct MissingMember {
  a: String,
}

add!(MissingMember { "hage".into() });
"""


@pytest.mark.parametrize("m", FORMATS)
def test_error_decode_missing_member(m):
    @dataclass
    class MissingMember:
        a: str
        b: str

    with pytest.raises(m.errtype) as e:
        m.loads_as(MissingMember, m.data("MissingMember"))

    print(f"{m.name}: {e}")


"""rust
#[derive(Serialize, Debug, new)]
struct TooManyMember {
  a: String,
  b: String,
  c: i64,
}

add!(TooManyMember { "hage".into(), "faa".into(), 33 });
"""


@pytest.mark.parametrize("m", FORMATS)
def test_error_decode_too_many_member(m):
    @perde.attr(deny_unknown_fields=True)
    @dataclass
    class TooManyMember:
        a: str
        b: str

    with pytest.raises(m.errtype) as e:
        m.loads_as(TooManyMember, m.data("TooManyMember"))

    print(f"{m.name}: {e}")


"""rust
#[derive(Serialize, Debug, new)]
struct SkipEnumError {
  x: i64,
  e: String,
}

add!(SkipEnumError { 3, "A".into() });
"""


@pytest.mark.parametrize("m", FORMATS)
def test_error_encode_skipped_enum(m):
    class E(perde.Enum):
        A = 1
        B = 2, {"perde_skip": True}
        C = 3

    @dataclass
    class SkipEnumError:
        x: int
        e: E

    assert m.data("SkipEnumError") == m.dumps(SkipEnumError(3, E.A))

    with pytest.raises(m.errtype) as e:
        m.dumps(SkipEnumError(3, E.B))

    assert e.value.args[0] == "variant `B` is marked as `skip` and cannot be serialized"

    class E2(perde.Enum):
        A = 1
        B = 2, {"perde_skip_serializing": True}
        C = 3

    @dataclass
    class SkipEnumError2:
        x: int
        e: E2

    assert m.data("SkipEnumError") == m.dumps(SkipEnumError2(3, E2.A))

    with pytest.raises(m.errtype) as e:
        m.dumps(SkipEnumError(3, E2.B))

    assert e.value.args[0] == "variant `B` is marked as `skip` and cannot be serialized"


"""rust
#[derive(Serialize, Debug, new)]
struct DictFlattenMsgpack {
  x: String,
  y: i64,
  pp: String,
  ppp: String,
  pppp: String,
}

add!(DictFlattenMsgpack {
     "hey".into(), -103223,
     "q1".into(), "q2".into(), "q3".into()
    });
"""


@pytest.mark.parametrize("m", FORMATS)
def test_error_dict_flatten_msgpack(m):
    @dataclass
    class DictFlattenMsgpack:
        x: str
        y: int
        z: typing.Dict[str, str] = field(metadata={"perde_flatten": True})

    d = DictFlattenMsgpack("hey", -103223, {"pp": "q1", "ppp": "q2", "pppp": "q3"})

    if m.fmtname == "msgpack":
        with pytest.raises(m.errtype) as e:
            m.dumps(d)
        print(e)
    else:
        assert m.dumps(d) == m.data("DictFlattenMsgpack")
