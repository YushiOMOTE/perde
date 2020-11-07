from enum import Enum
from dataclasses import dataclass
import perde_json
import typing


def repack(v):
    print(f'repacking {v}...')
    s = perde_json.dumps(v)
    print(f'packed: {s}')
    r = perde_json.loads(s)
    print(f'unpacked: {r}')
    assert r == v


def repack_as(t, v):
    print(f'repacking {v} as {t}...')
    s = perde_json.dumps(v)
    print(f'packed: {s}')
    r = perde_json.loads_as(t, s)
    print(f'unpacked: {r}')
    assert r == v


def test_number():
    repack(10)


def test_string():
    repack("hage")


def test_float():
    repack(1.3)


def test_bool():
    repack(True)
    repack(False)


def test_bytes():
    repack_as(bytes, b'1234')


def test_bytearray():
    repack_as(bytearray, bytearray(b'1234'))


def test_dict():
    repack({"a": 10})


def test_list():
    repack([1, 2, 3])


def test_set():
    repack_as(set, {1, 2, 3})


def test_frozen_set():
    repack_as(frozenset, {1, 2, 3})


def test_tuple():
    repack_as(typing.Tuple[int, str, bytes], (3, "abc", b"def"))


def test_class():
    @dataclass
    class A:
        a: int
        b: str
        c: bytes

    repack_as(A, A(100, "hage", b"hoge"))


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
    repack_as(typing.Union[int, str, bytes], b"abyte")
