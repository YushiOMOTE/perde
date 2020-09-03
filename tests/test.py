import perde
import enum
from typing_inspect import get_origin, get_args
from dataclasses import dataclass, fields, is_dataclass, field
from typing import Dict, TypeVar, Union, List, Tuple

class E(enum.Enum):
    X = 1
    Y = "hage"
    Z = 3

@dataclass
class C:
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

print(perde.load_as(C, '{"key": 3, "value": "ok"}'))
print(perde.load_as(A, '{"name": 3, "value": {"label": "hage", "tag": {"10": ["a",{"key": 333, "value": "hey"},5]}}}'))
print(perde.load_as(X, '{"some": {"x": 3}}'))
print(perde.load_as(E, '{"en": "Z"}'))
