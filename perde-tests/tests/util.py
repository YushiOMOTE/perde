from dataclasses import dataclass, field
from typing import List, Dict, Optional, Union, Tuple, TypeVar, Any
from typing_inspect import get_origin
import enum
import perde_json
import pytest
import json
import perde_json, perde_yaml, perde_msgpack
import os

@dataclass
class Format:
    name: str
    package: Any
    argtype: Any

    def dumps(self, v):
        return self.package.dumps(v)


    def loads(self, v):
        return self.package.loads(v)


    def loads_as(self, t, v):
        return self.package.loads_as(t, v)


    def data(self, name: str):
        p = self.data_path(name)

        if self.argtype is str:
            with open(p) as f:
                return f.read()
        elif self.argtype is bytes:
            with open(p, 'rb') as f:
                return f.read()


    def data_path(self, name: str):
        d = os.path.dirname(__file__)
        base = os.path.join(d, '../data/')
        return f'{base}/{self.name}/{name}'


    def unpack_data(self, name: str, astype = None):
        d = self.data(name)
        print(f'unpacking {d}')
        if astype is None:
            s = self.loads(d)
        else:
            s = self.loads_as(astype, d)
        print(f'unpacked {s}')
        return s


    def repack_data(self, name: str, astype = None, expect = None):
        d = self.data(name)
        print(f'repacking {d} in `{self.name}`...')
        if astype is not None:
            v = self.loads_as(astype, d)
        else:
            v = self.loads(d)
        print(f'unpacked {v}')
        if expect is not None:
            assert v == expect
        v = self.dumps(v)
        print(f'packed {v}')
        assert v == d


    def unpack_type(self, ty):
        return self.unpack_data(ty.__name__, astype = ty)


    def repack_type(self, ty):
        self.repack_data(ty.__name__, astype = ty)


FORMATS = [
    Format("json", perde_json, str),
    Format("yaml", perde_yaml, str),
    Format("msgpack", perde_msgpack, bytes)
]


def FORMATS_EXCEPT(*args):
    return [f for f in FORMATS if f.name not in args]


def repack(m, v):
    print(f'repacking {v}...')
    s = m.package.dumps(v)
    print(f'packed: {s}')
    r = m.package.loads(s)
    print(f'unpacked: {r}')
    assert r == v


def repack_as(m, t, v):
    print(f'repacking {v} as {t}...')
    s = m.package.dumps(v)
    print(f'packed: {s}')
    r = m.package.loads_as(t, s)
    print(f'unpacked: {r}')
    assert r == v


def repack_json(ty, *args, **kwargs):
    oty = get_origin(ty) or ty
    e = oty(*args, **kwargs)
    assert e is not None
    v = perde_json.dumps(e)
    print(f'ok: ser: {v}')
    a = perde_json.loads_as(ty, v)
    assert a is not None
    assert e == a
    print(f'ok: de: {a}')
    return v


def comp(a, e):
    e = json.dumps(e, separators=(',', ':'))
    assert a == e
    print(f'ok: de: {e}')
