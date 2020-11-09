from enum import Enum
from dataclasses import dataclass
import perde_json, perde_yaml, perde_msgpack
import typing


FORMATS = [perde_json, perde_yaml, perde_msgpack]


def repack(v):
    for m in FORMATS:
        print(f'repacking {v}...')
        s = m.dumps(v)
        print(f'packed: {s}')
        r = m.loads(s)
        print(f'unpacked: {r}')
        assert r == v


def repack_as(t, v):
    for m in FORMATS:
        print(f'repacking {v} as {t}...')
        s = m.dumps(v)
        print(f'packed: {s}')
        r = m.loads_as(t, s)
        print(f'unpacked: {r}')
        assert r == v


def test_number():
    repack(-10)
    repack(0)
    repack(10)


def test_string():
    repack("")
    repack("hage")
    repack("コンテナ")


def test_float():
    repack(-1.2)
    repack(0.0)
    repack(1.3)


def test_bool():
    repack(True)
    repack(False)


def test_bytes():
    repack_as(bytes, b'1234')
    repack_as(bytes, b'')


def test_bytearray():
    repack_as(bytearray, bytearray(b'1234'))
    repack_as(bytearray, bytearray(b''))


def test_dict():
    repack({"a": 10})
    repack({})
    repack({"a":{"b": 10}})


def test_list():
    repack([1, 2, 3])
    repack([])


def test_set():
    repack_as(set, {1, 2, 3})
    repack_as(set, {"a", "b", "c"})
    repack_as(set, set())


def test_frozen_set():
    repack_as(frozenset, {1, 2, 3})
    repack_as(frozenset, frozenset())


def test_tuple():
    repack_as(typing.Tuple[int, str, bytes], (3, "abc", b"def"))
    repack_as(typing.Tuple[str, dict], ("hage", {"a": -10}))


def test_class():
    @dataclass
    class A:
        a: int
        b: str
        c: bytes

    @dataclass
    class B:
        x: A
        b: str

    repack_as(A, A(100, "hage", b"hoge"))
    repack_as(B, B(A(-1, "", b"33"), "foo"))


def test_enum():
    class E(Enum):
        X = 1
        Y = "hage"
        Z = 3.3

    repack_as(E, E.X)
    repack_as(E, E.Y)
    repack_as(E, E.Z)


def test_optional():
    repack_as(typing.Optional[int], 3)
    repack_as(typing.Optional[str], "lel")


def test_union():
    repack_as(typing.Union[int, str, bytes], 3)
    repack_as(typing.Union[int, str, bytes], b"abyte")
    repack_as(typing.Union[int, str, bytes], "hage")
