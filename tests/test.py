import perde

from typing_inspect import get_origin, get_args
from perde import PySchema
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
        return PySchema(f, [], {}, [])

def to_generic(d: TypeVar):
    args = [to_field(arg) for arg in get_args(d)]
    return PySchema(dict, args, {}, [])

def to_primitive(d: TypeVar):
    return PySchema(d, [], {}, [])

def to_class(d: TypeVar):
    fs = dict([(f.name, to_field(f.type)) for f in fields(d)])
    return PySchema(d, [], fs, [])

def to_schema(d: TypeVar):
    return to_field(d)

def perde_register(d):
    print(to_schema(d))
    perde.register(id(d), to_schema(d))
    return d

@perde_register
@dataclass
class B:
    label: str
    tag: Dict[int, List[str]]

@perde_register
@dataclass
class A:
    name: str
    value: B

perde.json_load('{"a": 300}')
