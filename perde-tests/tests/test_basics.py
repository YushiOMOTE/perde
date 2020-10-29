from dataclasses import dataclass, field
from typing import List, Dict, Optional, Union, Tuple, TypeVar
from typing_inspect import get_origin
import enum
import perde_json
import pytest

from util import repack

@dataclass
class Entry:
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
    Entry(bytearray, [bytearray(b'abc\x03'), bytearray(b'')])
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
    Entry(List[bytearray], [[bytearray(b'abc\x03'), bytearray(b'')], []])
]

FEW_LISTS = [
    Entry(List[int], [[-18, 0, 5], []]),
    Entry(List[str], [["wazzaaa", ""], []]),
]

DICTS_SK = [
    Entry(Dict[str, bool], [{"k": True}, {}]),
    Entry(Dict[str, int], [{"a": 3}, {}]),
    Entry(Dict[str, float], [{"v": -1.4, "p": 0.0, "n": 2.2}, {}]),
    Entry(Dict[str, str], [{"v": "avc", "p": ""}, {"n": "x"}, {}]),
    Entry(Dict[str, bytes], [{"v": b"aaaa", "z": b""}, {"p": b"v"}, {}]),
]

FEW_DICTS_SK = [
    Entry(Dict[str, int], [{"a": 3}, {}]),
    Entry(Dict[str, str], [{"v": "avc", "p": ""}, {"n": "x"}, {}]),
]

@pytest.mark.parametrize("t1,v1", expand(PRIMITIVES + LISTS + DICTS_SK))
def test_primitives(t1, v1):
    repack(t1, v1)

@pytest.mark.parametrize("t1,v1", expand(PRIMITIVES + LISTS + DICTS_SK))
@pytest.mark.parametrize("t2,v2", expand(PRIMITIVES + LISTS + DICTS_SK))
def test_simple_classes(t1, t2, v1, v2):
    @dataclass
    class Test:
        a: t1
        b: t2

    repack(Test, v1, v2)

@pytest.mark.parametrize("t1,v1", expand(FEW_PRIMITIVES))
@pytest.mark.parametrize("t2,v2", expand(FEW_PRIMITIVES + FEW_LISTS + FEW_DICTS_SK))
@pytest.mark.parametrize("t3,v3", expand(FEW_PRIMITIVES + FEW_LISTS + FEW_DICTS_SK))
def test_nested_classes(t1, t2, t3, v1, v2, v3):
    @dataclass
    class Test2:
        a: t1
        b: t2

    @dataclass
    class Test:
        x: t3
        y: Test2

    repack(Test, v3, Test2(v1, v2))
