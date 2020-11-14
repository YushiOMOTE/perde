from dataclasses import dataclass, field
from typing import List, Dict, Optional, Union, Tuple, TypeVar, Any
from typing_inspect import get_origin
import enum
import pytest
import os

import perde_json
import perde_yaml
import perde_msgpack

import json
import yaml
import msgpack


@dataclass
class Format:
    name: str
    fmtname: str
    package: Any
    argtype: Any


    def dumps(self, v):
        return self.package.dumps(v)


    def loads(self, v):
        return self.package.loads(v)


    def loads_as(self, t, v):
        return self.package.loads_as(t, v)


    def repack(self, v):
        print(f'repacking {v}...')
        s = self.package.dumps(v)
        print(f'packed: {s}')
        r = self.package.loads(s)
        print(f'unpacked: {r}')
        assert r == v


    def repack_as(self, t, v):
        print(f'repacking {v} as {t}...')
        s = self.package.dumps(v)
        print(f'packed: {s}')
        r = self.package.loads_as(t, s)
        print(f'unpacked: {r}')
        assert r == v


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
        return f'{base}/{self.fmtname}/{name}'


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


    def pack_bench(self, benchmark, v):
        self.package.pack_bench(benchmark, v)


    def unpack_bench(self, benchmark, v, t):
        self.package.unpack_bench(benchmark, v, t)



FORMATS = [
    Format("json", "json", perde_json, str),
    Format("yaml", "yaml", perde_yaml, str),
    Format("msgpack", "msgpack", perde_msgpack, bytes)
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


class Json:
    def pack_bench(b, v):
        b(json.dumps, v)

    def unpack_bench(b, v, t):
        b(json.loads, v)


class Yaml:
    def pack_bench(b, v):
        b(yaml.dump, v)

    def unpack_bench(b, v, t):
        b(yaml.safe_load, v)


class MsgPack:
    def pack_bench(b, v):
        b(msgpack.dumps, v)

    def unpack_bench(b, v, t):
        b(msgpack.loads, v)


class PerdeJson:
    def pack_bench(b, v):
        b(perde_json.dumps, v)

    def unpack_bench(b, v, t):
        b(perde_json.loads, v)


class PerdeYaml:
    def pack_bench(b, v):
        b(perde_yaml.dumps, v)

    def unpack_bench(b, v, t):
        b(perde_yaml.loads, v)


class PerdeMsgPack:
    def pack_bench(b, v):
        b(perde_msgpack.dumps, v)

    def unpack_bench(b, v, t):
        b(perde_msgpack.loads, v)


class PerdeJsonAs:
    def pack_bench(b, v):
        b(perde_json.dumps, v)

    def unpack_bench(b, v, t):
        b(perde_json.loads_as, t, v)


class PerdeYamlAs:
    def pack_bench(b, v):
        b(perde_yaml.dumps, v)

    def unpack_bench(b, v, t):
        b(perde_yaml.loads_as, t, v)


class PerdeMsgPackAs:
    def pack_bench(b, v):
        b(perde_msgpack.dumps, v)

    def unpack_bench(b, v, t):
        b(perde_msgpack.loads_as, t, v)


def idfn(m):
    return m.name


BENCH_FORMATS = [
    Format("json", "json", Json, str),
    Format("yaml", "yaml", Yaml, str),
    Format("msgpack", "msgpack", MsgPack, bytes),
    Format("perde_json", "json", PerdeJson, str),
    Format("perde_yaml", "yaml", PerdeYaml, str),
    Format("perde_msgpack", "msgpack", PerdeMsgPack, bytes),
    Format("perde_json_as", "json", PerdeJsonAs, str),
    Format("perde_yaml_as", "yaml", PerdeYamlAs, str),
    Format("perde_msgpack_as", "msgpack", PerdeMsgPackAs, bytes)
]

MARKED_BENCH_FORMATS = [pytest.param(c, marks=[getattr(pytest.mark, c.fmtname)]) for c in BENCH_FORMATS]
