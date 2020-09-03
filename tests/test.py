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

class E(enum.Enum):
    X = 1
    Y = "hage"
    Z = 3

@dataclass
class C:
    key: int
    value: str

@attr(deny_unknown_fields = True)
@dataclass
class CC:
    key: int
    value: str

@dataclass
class B:
    label: str
    tag: Dict[str, Tuple[str, C, int]]

@dataclass
class A:
    name: int
    value: B

@dataclass
class X:
    some: Union[int, Dict[str, int], C]

@dataclass
class E:
    en: E

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

@attr(rename_all = "camelCase")
@dataclass
class R:
    this_is_it: int
    over_night: str

@attr(default = True)
@dataclass
class Def:
    a: int
    b: int
    c: int

print(perde.load_as(C, '{"key": 3, "value": "ok"}'))
print(perde.load_as(C, '{"key": 3, "value": "ok", "aa": 44}'))

print(perde.load_as(CC, '{"key": 3, "value": "ok"}'))
try:
    print(perde.load_as(CC, '{"key": 3, "value": "ok", "aa": 44}'))
    exit()
except:
    print(f'OK')

print(perde.load_as(A, '{"name": 3, "value": {"label": "hage", "tag": {"10": ["a",{"key": 333, "value": "hey"},5]}}}'))
print(perde.load_as(X, '{"some": {"x": 3}}'))
print(perde.load_as(E, '{"en": "Z"}'))
print(perde.load_as(F, '{"x":1,"y":2,"a":3,"c":4,"p":"3","q":"4"}'))
print(perde.load_as(R, '{"thisIsIt": 3, "overNight": "haa"}'))
print(perde.load_as(Def, '{"a": 3, "c": 1000}'))
