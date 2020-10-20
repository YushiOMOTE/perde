from dataclasses import dataclass, field
from typing import List, Dict, Optional, Union, Tuple, TypeVar
from typing_inspect import get_origin
import enum
import perde_json
import pytest
import json


def repack(ty, *args, **kwargs):
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
