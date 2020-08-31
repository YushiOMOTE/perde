import perde

from typing_inspect import get_origin, get_args
from perde import Schema
from dataclasses import dataclass, fields, is_dataclass, field
from typing import Dict, TypeVar, Union, List

def to_field(f: TypeVar):
    print(get_origin(f))
    if is_dataclass(f):
        print("is dataclass!!")
        return to_class(f)
    elif get_origin(f) is not None:
        print("is generic!!")
        return to_generic(f)
    else:
        print("is primitive!!")
        return Schema(f, [], {}, [])

def to_generic(d: TypeVar):
    args = [to_field(arg) for arg in get_args(d)]
    return Schema(dict, args, {}, [])

def to_primitive(d: TypeVar):
    return Schema(d, [], {}, [])

def to_class(d: TypeVar):
    fs = dict([(f.name, to_field(f.type)) for f in fields(d)])
    return Schema(d, [], fs, [])

def to_schema(d: TypeVar):
    return to_field(d)

def perde_register(d):
    s = to_schema(d)
    print(s)
    setattr(d, '__schema__', s)
    return d

@perde_register
@dataclass
class B:
    label: str
    tag: Dict[int, List[str]]

@perde_register
@dataclass
class A:
    name: int
    value: B

print(perde.json_load(A, '{"name": 3, "value": {"label": "hage", "tag": {}}}'))
