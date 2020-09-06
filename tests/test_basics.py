from dataclasses import dataclass
from typing import List, Dict, Optional, Union, Tuple, TypeVar
from parameterized import parameterized
import enum
import perde
import pytest


def repack(ty, *args, **kwargs):
    e = ty(*args, **kwargs)
    v = perde.json.dumps(e)
    print(f'ok: ser: {v}')
    a = perde.json.loads_as(ty, v)
    assert e == a
    print(f'ok: de: {v}')


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

@pytest.mark.parametrize("t1,v1", expand(PRIMITIVES))
def test_primitives(t1, v1):
    repack(t1, v1)

@pytest.mark.parametrize("t1,v1", expand(PRIMITIVES))
@pytest.mark.parametrize("t2,v2", expand(PRIMITIVES))
def test_simple_classes(t1, t2, v1, v2):
    @dataclass
    class Simple:
        a: t1
        b: t2

    repack(Simple, v1, v2)
