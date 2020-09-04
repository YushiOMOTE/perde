import perde
import enum
from typing_inspect import get_origin, get_args
from dataclasses import dataclass, fields, is_dataclass, field
from typing import Dict, TypeVar, Union, List, Tuple

def attr(**kwargs):
    def func(ty):
        setattr(ty, "__perde_attr__", kwargs)
        return ty
    return func


def test(t, s, e):
    p = perde.load_as(t, s)
    assert p == e, f'\n * expected: {e}\n * got     : {p}'
    print(f'ok: {p}')


def testp(t, s):
    try:
        p = perde.load_as(t, s)
        assert False, "this must panic"
    except:
        pass


@dataclass
class C:
    key: int
    value: str

@attr(deny_unknown_fields = True)
@dataclass
class CC:
    key: int
    value: str

test(C, '{"key": 3, "value": "ok"}', C(3, "ok"))
test(C, '{"key": 3, "value": "ok", "aa": 44}', C(3, "ok"))
test(CC, '{"key": 3, "value": "ok"}', CC(3, "ok"))
testp(CC, '{"key": 3, "value": "ok", "aa": 44}')

@dataclass
class B:
    label: str
    tag: Dict[str, Tuple[str, C, int]]

@dataclass
class A:
    name: int
    value: B

test(A, '{"name": 3, "value": {"label": "hage", "tag": {"10": ["a",{"key": 333, "value": "hey"},5]}}}',
     A(3, B("hage", {"10": ("a", C(333, "hey"), 5)})))

@dataclass
class X:
    some: Union[int, Dict[str, int], C]

test(X, '{"some": {"x": 3}}', X({"x": 3}))

class EN(enum.Enum):
    X = 1
    Y = "hage"
    Z = 3

@dataclass
class E:
    en: EN

test(E, '{"en": "Z"}', E(EN.Z))

@dataclass
class FFF:
    p: str
    q: str

@dataclass
class FF:
    a: int
    b: FFF = field(metadata = {"perde_flatten": True})
    c: int

@dataclass
class F:
    x: int
    y: int
    z: FF = field(metadata = {"perde_flatten": True})

test(F, '{"x":1,"y":2,"a":3,"c":4,"p":"3","q":"4"}', F(1,2,FF(3,FFF("3","4"),4)))

@attr(rename_all = "camelCase")
@dataclass
class R:
    this_is_it: int
    over_night: str

test(R, '{"thisIsIt": 3, "overNight": "haa"}', R(3, "haa"))

@attr(default = True)
@dataclass
class Def:
    a: int
    b: int
    c: int

test(Def, '{"a": 3, "c": 1000}', Def(3, 0, 1000))

@dataclass
class Def2:
    a: int = field(metadata = {"perde_default": True})
    b: int
    c: int

test(Def2, '{"b": 3, "c": 1000}', Def2(0, 3, 1000))

@dataclass
class RenameF:
    a: int
    b: int = field(metadata = {"perde_rename": "kami"})
    c: int

test(RenameF, '{"a": 3, "kami": 100000, "c": 1000}', RenameF(3, 100000, 1000))

@dataclass
class Skip:
    a: int = field(metadata = {"perde_skip": True})
    b: int
    c: int

@dataclass
class SkipDe:
    a: int
    b: int
    c: int = field(metadata = {"perde_skip_deserializing": True})

    test(Skip, '{"b": 3, "c": 1000}', Skip(0, 3, 1000))
test(SkipDe, '{"a": 300, "b": 3}', SkipDe(300, 3, 0))


import timeit

res_perde_as = timeit.repeat('perde.load_as(C, \'{"key": 300, "value": "hoge"}\')', setup = '''
import perde
from dataclasses import dataclass

@dataclass
class C:
    key: int
    value: str
perde.load_as(C, \'{"key": 300, "value": "hoge"}\')
''', number = 100000)

res_json = timeit.repeat('json.loads(\'{"key": 300, "value": "hoge"}\')', setup = "import json", number = 100000)
res_ujson = timeit.repeat('ujson.loads(\'{"key": 300, "value": "hoge"}\')', setup = "import ujson", number = 100000)
res_perde = timeit.repeat('perde.loads(\'{"key": 300, "value": "hoge"}\')', setup = "import perde", number = 100000)
res_orjson = timeit.repeat('orjson.loads(\'{"key": 300, "value": "hoge"}\')', setup = "import orjson", number = 100000)

print(f'json      = {res_json}')
print(f'perde as  = {res_perde_as}')
print(f'perde     = {res_perde}')
print(f'ujson     = {res_ujson}')
print(f'orjson    = {res_orjson}')
