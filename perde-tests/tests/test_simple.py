from dataclasses import dataclass
import enum
import typing
import pytest
import perde
from util import FORMATS, repack, repack_as


@pytest.mark.parametrize("m", FORMATS)
def test_number(m):
    repack(m, -10)
    repack(m, 0)
    repack(m, 10)
    repack_as(m, int, -10)
    repack_as(m, int, 0)
    repack_as(m, int, 10)


@pytest.mark.parametrize("m", FORMATS)
def test_string(m):
    repack(m, "")
    repack(m, "hage")
    repack(m, "コンテナ")
    repack_as(m, str, "")
    repack_as(m, str, "hage")
    repack_as(m, str, "コンテナ")


@pytest.mark.parametrize("m", FORMATS)
def test_float(m):
    repack(m, -1.2)
    repack(m, 0.0)
    repack(m, 1.3)
    repack_as(m, float, -1.2)
    repack_as(m, float, 0.0)
    repack_as(m, float, 1.3)


@pytest.mark.parametrize("m", FORMATS)
def test_bool(m):
    repack(m, True)
    repack(m, False)
    repack_as(m, bool, True)
    repack_as(m, bool, False)


@pytest.mark.parametrize("m", FORMATS)
def test_bytes(m):
    repack_as(m, bytes, b'1234')
    repack_as(m, bytes, b'')


@pytest.mark.parametrize("m", FORMATS)
def test_bytearray(m):
    repack_as(m, bytearray, bytearray(b'1234'))
    repack_as(m, bytearray, bytearray(b''))


@pytest.mark.parametrize("m", FORMATS)
def test_dict(m):
    repack(m, {"a": 10})
    repack(m, {})
    repack(m, {"a":{"b": 10}})
    repack_as(m, dict, {"a": 10})
    repack_as(m, dict, {})
    repack_as(m, dict, {"a":{"b": 10}})
    repack_as(m, typing.Dict, {"a": 10})
    repack_as(m, typing.Dict[str, int], {"a": 10})
    repack_as(m, typing.Dict[str, int], {})
    repack_as(m, typing.Dict[str, typing.Dict[str, int]], {"a":{"b": 10}})
    repack_as(m, typing.Dict[str, typing.Any], {"xxx": 3.3})


@pytest.mark.parametrize("m", FORMATS)
def test_list(m):
    repack(m, [1, 2, 3])
    repack(m, [1])
    repack(m, [])
    repack_as(m, list, [1, 2, 3])
    repack_as(m, list, [1])
    repack_as(m, list, [])
    repack_as(m, typing.List, [1, 2, 3])
    repack_as(m, typing.List[int], [1, 2, 3])
    repack_as(m, typing.List[int], [1])
    repack_as(m, typing.List[int], [])
    repack_as(m, typing.List[typing.Any], ["a", "b", "c"])
    repack_as(m, typing.List[typing.Any], ["a", "b"])
    repack_as(m, typing.List[typing.Any], ["a"])
    repack_as(m, typing.List[typing.Any], [])


@pytest.mark.parametrize("m", FORMATS)
def test_set(m):
    repack_as(m, set, {1, 2, 3})
    repack_as(m, set, {"a", "b", "c"})
    repack_as(m, set, set())
    repack_as(m, typing.Set, {1, 2, 3})
    repack_as(m, typing.Set[int], {1, 2, 3})
    repack_as(m, typing.Set[str], {"a", "b", "c"})
    repack_as(m, typing.Set[str], set())
    repack_as(m, typing.Set[typing.Any], {"a", "b", "c"})
    repack_as(m, typing.Set[typing.Any], set())


@pytest.mark.parametrize("m", FORMATS)
def test_frozen_set(m):
    repack_as(m, frozenset, {1, 2, 3})
    repack_as(m, frozenset, {"a", "b", "c"})
    repack_as(m, frozenset, frozenset())
    repack_as(m, typing.FrozenSet, {1, 2, 3})
    repack_as(m, typing.FrozenSet[int], {1, 2, 3})
    repack_as(m, typing.FrozenSet[str], {"a", "b", "c"})
    repack_as(m, typing.FrozenSet[str], frozenset())
    repack_as(m, typing.FrozenSet[typing.Any], {"a", "b"})
    repack_as(m, typing.FrozenSet[typing.Any], {"a"})
    repack_as(m, typing.FrozenSet[typing.Any], frozenset())


@pytest.mark.parametrize("m", FORMATS)
def test_tuple(m):
    repack_as(m, tuple, ("hage", -100, 3.14))
    repack_as(m, tuple, (33, {"a": 10}))
    repack_as(m, tuple, ("hage",))
    repack_as(m, tuple, ())
    repack_as(m, typing.Tuple, (3, "abc", "def"))
    repack_as(m, typing.Tuple[int, str, bytes], (3, "abc", b"def"))
    repack_as(m, typing.Tuple[str, dict], ("hage", {"a": -10}))
    repack_as(m, typing.Tuple[str], ("foo",))
    repack_as(m, typing.Tuple[int], ())
    repack_as(m, typing.Tuple[int, str, typing.Any], (3, "abc", "def"))
    repack_as(m, typing.Tuple[int, typing.Any, bytes], (3, "abc", b"def"))
    repack_as(m, typing.Tuple[typing.Any, str, bytes], (3, "abc", b"def"))


@pytest.mark.parametrize("m", FORMATS)
def test_class(m):
    @dataclass
    class A:
        a: int
        b: str
        c: bytes

    @dataclass
    class B:
        x: A
        b: str

    repack_as(m, A, A(100, "hage", b"hoge"))
    repack_as(m, B, B(A(-1, "", b"33"), "foo"))

    @dataclass
    class C:
        x: typing.Any
        b: str

    repack_as(m, C, C(3, "hello"))

    @dataclass
    class V:
        pass

    repack_as(m, V, V())


@pytest.mark.parametrize("m", FORMATS)
def test_enum(m):
    class E(enum.Enum):
        X = 1
        Y = "hage"
        Z = 3.3

    repack_as(m, E, E.X)
    repack_as(m, E, E.Y)
    repack_as(m, E, E.Z)

    @perde.attr(as_value = True)
    class EV(enum.Enum):
        X = 1
        Y = "hage"
        Z = 3.3

    repack_as(m, EV, EV.X)
    repack_as(m, EV, EV.Y)
    repack_as(m, EV, EV.Z)

    class IE(enum.IntEnum):
        X = 1
        Y = 4
        Z = 5

    repack_as(m, IE, IE.X)
    repack_as(m, IE, IE.Y)
    repack_as(m, IE, IE.Z)

    @perde.attr(as_value = True)
    class IEV(enum.IntEnum):
        X = 1
        Y = 4
        Z = 5

    repack_as(m, IEV, IEV.X)
    repack_as(m, IEV, IEV.Y)
    repack_as(m, IEV, IEV.Z)


@pytest.mark.parametrize("m", FORMATS)
def test_flag(m):
    class E(enum.Flag):
        X = enum.auto()
        Y = enum.auto()
        Z = X | Y

    repack_as(m, E, E.X)
    repack_as(m, E, E.Y)
    repack_as(m, E, E.Z)

    @perde.attr(as_value = True)
    class EV(enum.Flag):
        X = enum.auto()
        Y = enum.auto()
        Z = X | Y

    repack_as(m, EV, EV.X)
    repack_as(m, EV, EV.Y)
    repack_as(m, EV, EV.Z)

    class IE(enum.IntFlag):
        X = enum.auto()
        Y = enum.auto()
        Z = X | Y

    repack_as(m, IE, IE.X)
    repack_as(m, IE, IE.Y)
    repack_as(m, IE, IE.Z)

    @perde.attr(as_value = True)
    class IEV(enum.IntFlag):
        X = enum.auto()
        Y = enum.auto()
        Z = X | Y

    repack_as(m, IEV, IEV.X)
    repack_as(m, IEV, IEV.Y)
    repack_as(m, IEV, IEV.Z)


@pytest.mark.parametrize("m", FORMATS)
def test_optional(m):
    repack_as(m, typing.Optional, 3.3)
    repack_as(m, typing.Optional[int], 3)
    repack_as(m, typing.Optional[str], "lel")


@pytest.mark.parametrize("m", FORMATS)
def test_union(m):
    repack_as(m, typing.Union, 3)
    repack_as(m, typing.Union[int, str, bytes], 3)
    repack_as(m, typing.Union[int, str, bytes], b"abyte")
    repack_as(m, typing.Union[int, str, bytes], "hage")


@pytest.mark.parametrize("m", FORMATS)
def test_any(m):
    repack_as(m, typing.Any, 3)
    repack_as(m, typing.Any, "abc")
    repack_as(m, typing.Any, [1, 2, 3])
