import perde
from perde import json_load
from dataclasses import dataclass, fields, is_dataclass, field
from typing import Dict, TypeVar, Union, List

@dataclass
class Schema:
    cls: TypeVar
    fields: Dict[str, Union[TypeVar, 'Schema']]
    attr: List[str] = field(default_factory = list)

def to_field(f):
    if is_dataclass(f.type):
        return to_schema(f.type)
    else:
        return f.type

def to_schema(d):
    fs = [(f.name, to_field(f)) for f in fields(d)]
    return Schema(d, fs)

def perde(d):
    print(to_schema(d))
    return d

@perde
@dataclass
class B:
    label: str
    tag: int

@perde
@dataclass
class A:
    name: str
    value: B

json_load('{"a": 300}')
