from dataclasses import dataclass
from typing import List, Dict, TypeVar
import pytest
from util import FORMATS_EXCEPT, FORMATS_ONLY, repack_as


@dataclass
class Entry:
    """
    The type `ty` and its possible values `valus`
    """
    ty: TypeVar
    values: List[object]

    def expand(self):
        return [(self.ty, v) for v in self.values]


def expand(es):
    return [v for e in es for v in e.expand()]


PRIMITIVES = [
    Entry(bool, [True, False]),
    Entry(int, [-100, 0, 100]),
    Entry(float, [-3.1415, 0.0, 1.4142]),
    Entry(str, ["wazzaaa", ""]),
    Entry(bytes, [b'abc\x03', b'']),
    Entry(bytearray,
          [bytearray(b'abc\x03'), bytearray(b'')])
]

FEW_PRIMITIVES = [
    Entry(int, [-100, 0, 100]),
    Entry(str, ["wazzaaa", ""]),
]

LISTS = [
    Entry(List[bool], [[True, False], []]),
    Entry(List[int], [[-18, 0, 5], []]),
    Entry(List[float], [[-3.1415, 0.0, 1.4142], []]),
    Entry(List[str], [["wazzaaa", ""], []]),
    Entry(List[bytes], [[b'abc\x03', b''], []]),
    Entry(List[bytearray],
          [[bytearray(b'abc\x03'), bytearray(b'')], []])
]

FEW_LISTS = [
    Entry(List[int], [[-18, 0, 5], []]),
    Entry(List[str], [["wazzaaa", ""], []]),
]

DICTS_SK = [
    Entry(Dict[str, bool], [{
        "k": True
    }, {}]),
    Entry(Dict[str, int], [{
        "a": 3
    }, {}]),
    Entry(Dict[str, float], [{
        "v": -1.4,
        "p": 0.0,
        "n": 2.2
    }, {}]),
    Entry(Dict[str, str], [{
        "v": "avc",
        "p": ""
    }, {
        "n": "x"
    }, {}]),
    Entry(Dict[str, bytes], [{
        "v": b"aaaa",
        "z": b""
    }, {
        "p": b"v"
    }, {}]),
]

FEW_DICTS_SK = [
    Entry(Dict[str, int], [{
        "a": 3
    }, {}]),
    Entry(Dict[str, str], [{
        "v": "avc",
        "p": ""
    }, {
        "n": "x"
    }, {}]),
]


@pytest.mark.parametrize("m", FORMATS_EXCEPT("toml"))
@pytest.mark.parametrize("t1,v1", expand(PRIMITIVES + LISTS + DICTS_SK))
def test_primitives(m, t1, v1):
    repack_as(m, t1, v1)


@pytest.mark.parametrize("m", FORMATS_EXCEPT("toml"))
@pytest.mark.parametrize("t1,v1", expand(PRIMITIVES + LISTS + DICTS_SK))
@pytest.mark.parametrize("t2,v2", expand(PRIMITIVES + LISTS + DICTS_SK))
def test_simple_classes(m, t1, t2, v1, v2):
    @dataclass
    class Test:
        a: t1
        b: t2

    repack_as(m, Test, Test(v1, v2))


@pytest.mark.parametrize("m", FORMATS_EXCEPT("toml"))
@pytest.mark.parametrize("t1,v1", expand(FEW_PRIMITIVES))
@pytest.mark.parametrize("t2,v2",
                         expand(FEW_PRIMITIVES + FEW_LISTS + FEW_DICTS_SK))
@pytest.mark.parametrize("t3,v3",
                         expand(FEW_PRIMITIVES + FEW_LISTS + FEW_DICTS_SK))
def test_nested_classes(m, t1, t2, t3, v1, v2, v3):
    @dataclass
    class Child:
        a: t1
        b: t2

    @dataclass
    class Test1:
        x: t3
        y: Child

    repack_as(m, Test1, Test1(v3, Child(v1, v2)))

    @dataclass
    class Test2:
        x: Child
        y: t3

    repack_as(m, Test2, Test2(Child(v1, v2), v3))

    @dataclass
    class Test3:
        p: t1
        q: Child
        r: t3

    repack_as(m, Test3, Test3(v1, Child(v1, v2), v3))


@pytest.mark.parametrize("m", FORMATS_ONLY("toml"))
@pytest.mark.parametrize("t1,v1", expand(PRIMITIVES))
@pytest.mark.parametrize("t2,v2", expand(PRIMITIVES + LISTS + DICTS_SK))
def test_simple_classes_tables_after(m, t1, t2, v1, v2):
    @dataclass
    class Test:
        a: t1
        b: t2

    repack_as(m, Test, Test(v1, v2))


@pytest.mark.parametrize("m", FORMATS_ONLY("toml"))
@pytest.mark.parametrize("t1,v1", expand(FEW_PRIMITIVES))
@pytest.mark.parametrize("t2,v2",
                         expand(FEW_PRIMITIVES + FEW_LISTS + FEW_DICTS_SK))
@pytest.mark.parametrize("t3,v3", expand(FEW_PRIMITIVES))
def test_nested_classes_tables_after(m, t1, t2, t3, v1, v2, v3):
    @dataclass
    class Child:
        a: t1
        b: t2

    @dataclass
    class Test1:
        x: t3
        y: Child

    repack_as(m, Test1, Test1(v3, Child(v1, v2)))
