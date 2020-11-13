from dataclasses import dataclass, field
from typing import List, Dict, Optional, Union, Tuple, TypeVar
import enum
import perde, perde_json
import pytest

from util import *

"""rust
#[derive(Serialize, Debug, new)]
struct DefaultConstruct {
  a: String,
  c: u64,
}

add!(DefaultConstruct {"xxx".into(), 3});
"""
@pytest.mark.parametrize("m", FORMATS)
def test_default(m):
    @perde.attr(default = True)
    @dataclass
    class DefaultConstruct:
        a: str
        b: str
        c: int

    p = m.unpack_data("DefaultConstruct", astype=DefaultConstruct)
    assert p == DefaultConstruct("xxx", "", 3)


@pytest.mark.parametrize("m", FORMATS)
def test_field_default(m):
    @dataclass
    class DefaultConstruct2:
        a: str
        b: str = field(metadata = {"perde_default": True})
        c: int

    p = m.unpack_data("DefaultConstruct", astype=DefaultConstruct2)
    assert p == DefaultConstruct2("xxx", "", 3)


@pytest.mark.parametrize("m", FORMATS)
def test_field_default_value(m):
    @dataclass
    class DefaultConstruct3:
        a: str
        b: str = field(default = "hage")
        c: int = field(default = 99)

    p = m.unpack_data("DefaultConstruct", astype=DefaultConstruct3)
    assert p == DefaultConstruct3("xxx", "hage", 3)


@pytest.mark.parametrize("m", FORMATS)
def test_field_default_factory(m):
    @dataclass
    class DefaultConstruct4:
        a: str
        b: str = field(default_factory = lambda: "hage")
        c: int = field(default = 99)

    p = m.unpack_data("DefaultConstruct", astype=DefaultConstruct4)
    assert p == DefaultConstruct4("xxx", "hage", 3)
