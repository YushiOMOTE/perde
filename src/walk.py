from typing_inspect import get_origin, get_args, is_union_type, is_optional_type
from perde import Schema
from dataclasses import dataclass, fields, is_dataclass, field
from typing import Dict, TypeVar, Union, List, Optional, Union
from enum import Enum

def to_schema(t: TypeVar, attr = {}):
    print(f'type is {t}')
    print(f'origin is {get_origin(t)}')
    if is_dataclass(t):
        return to_class(t, attr)
    if is_generic(t):
        return to_generic(t, attr)
    if issubclass(t, Enum):
        return to_enum(t, attr)
    if issubclass(t, (bool, int, float, str, bytes, bytearray)):
        return to_simple(t, attr)
    else:
        raise TypeError(f'Unsupported type {t}')

def to_simple(t: TypeVar, attr = {}):
    return Schema(t, t.__name__, [], {}, attr)

def is_generic(t: TypeVar):
    if get_origin(t) is not None:
        return True
    if is_union_type(t) or is_optional_type(t):
        return True
    return False

def to_generic(t: TypeVar, attr = {}):
    args = [to_schema(arg) for arg in get_args(t)]
    if is_optional_type(t):
        ty = type(None)
        name = "option"
    elif is_union_type(t):
        ty = type(None)
        name = "union"
    else:
        ty = get_origin(t)
        name = ty.__name__
    print(f'type == {ty}')
    return Schema(ty, name, args, {}, attr)

def to_enum(t: TypeVar, attr = {}):
    fs = dict([(f.name, to_schema(type(f.value))) for f in t])
    return Schema(t, "enum", [], fs, attr)

def to_class(t: TypeVar, attr = {}):
    fs = dict([(f.name, to_schema(f.type, dict(f.metadata))) for f in fields(t)])
    return Schema(t, "class", [], fs, attr)
