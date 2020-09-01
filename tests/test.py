import perde

from typing_inspect import get_origin, get_args
from perde import Schema
from dataclasses import dataclass, fields, is_dataclass, field
from typing import Dict, TypeVar, Union, List, Tuple

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

print(perde.load_as(C, '{"key": 3, "value": "ok"}'))
print(perde.load_as(A, '{"name": 3, "value": {"label": "hage", "tag": {"10": ["a",{"key": 333, "value": "hey"},5]}}}'))
