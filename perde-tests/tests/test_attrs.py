from dataclasses import dataclass, field
from typing import List, Dict, Optional, Union, Tuple, TypeVar
import enum
import perde, perde_json
import pytest

from util import *

"""rust
#[derive(Serialize, Debug, new)]
struct Plain {
  a: String,
  b: String,
  c: u64,
}

add!(Plain {"xxx".into(), "yyy".into(), 3});
"""
@pytest.mark.parametrize("m", FORMATS)
def test_plain(m):
    @dataclass
    class Plain:
        a: str
        b: str
        c: int

    m.repack_type(Plain)


"""rust
#[derive(Serialize, Debug, new)]
struct Rename {
  a: String,
  #[serde(rename = "x")]
  b: String,
  c: u64,
}

add!(Rename {"xxx".into(), "yyy".into(), 3});
"""
@pytest.mark.parametrize("m", FORMATS)
def test_rename(m):
    @dataclass
    class Rename:
        a: str
        b: str = field(metadata = {"perde_rename": "x"})
        c: int

    m.repack_type(Rename)


"""rust
#[derive(Serialize, Debug, new)]
#[serde(rename_all = "camelCase")]
struct RenameAll {
  pen_pineapple: String,
  apple_pen: String,
}

add!(RenameAll {"xxx".into(), "yyy".into()});
"""
@pytest.mark.parametrize("m", FORMATS)
def test_rename_all(m):
    @perde.attr(rename_all = "camelCase")
    @dataclass
    class RenameAll:
        pen_pineapple: str
        apple_pen: str

    m.repack_type(RenameAll)


"""rust
#[derive(Serialize, Debug, new)]
#[serde(rename_all = "camelCase")]
struct RenameAllRename {
  pen_pineapple: String,
  #[serde(rename = "pen_pen")]
  apple_pen: String,
}

add!(RenameAllRename {"xxx".into(), "yyy".into()});
"""
@pytest.mark.parametrize("m", FORMATS)
def test_rename_in_rename_all(m):
    @perde.attr(rename_all = "camelCase")
    @dataclass
    class RenameAllRename:
        pen_pineapple: str
        apple_pen: str = field(metadata = {"perde_rename": "pen_pen"})

    m.repack_type(RenameAllRename)


"""rust
#[derive(Serialize, Debug, new)]
struct NestedRenameChild {
  a: String,
  #[serde(rename = "d")]
  b: String,
}

#[derive(Serialize, Debug, new)]
struct NestedRename {
  x: String,
  #[serde(rename = "w")]
  y: NestedRenameChild,
  z: i64,
}

add!(NestedRename {"xxx".into(), NestedRenameChild::new("ppp".into(), "qqq".into()), 1111});
"""
@pytest.mark.parametrize("m", FORMATS)
def test_rename_in_rename_all(m):
    @dataclass
    class NestedRenameChild:
        a: str
        b: str = field(metadata = {"perde_rename": "d"})

    @dataclass
    class NestedRename:
        x: str
        y: NestedRenameChild = field(metadata = {"perde_rename": "w"})
        z: int

    m.repack_type(NestedRename)


"""rust
#[derive(Serialize, Debug, new)]
#[serde(rename_all = "UPPERCASE")]
struct NestedRenameAllChild {
  a: String,
  b: String,
}

#[derive(Serialize, Debug, new)]
struct NestedRenameAll {
  x: String,
  y: NestedRenameAllChild,
  z: i64,
}

add!(NestedRenameAll {"xxx".into(), NestedRenameAllChild::new("ppp".into(), "qqq".into()), 1111});
"""
@pytest.mark.parametrize("m", FORMATS)
def test_rename_in_rename_all(m):
    @perde.attr(rename_all = "UPPERCASE")
    @dataclass
    class NestedRenameAllChild:
        a: str
        b: str

    @dataclass
    class NestedRenameAll:
        x: str
        y: NestedRenameAllChild
        z: int

    m.repack_type(NestedRenameAll)


"""rust
#[derive(Serialize, Debug, new)]
struct FlattenChild {
  a: String,
  b: String,
}

#[derive(Serialize, Debug, new)]
struct Flatten {
  x: String,
  #[serde(flatten)]
  y: FlattenChild,
  z: i64,
}

add!(Flatten {"xxx".into(), FlattenChild::new("ppp".into(), "qqq".into()), 1111}
     except "msgpack");
"""
@pytest.mark.parametrize("m", FORMATS_EXCEPT("msgpack"))
def test_rename_in_rename_all(m):
    @dataclass
    class FlattenChild:
        a: str
        b: str

    @dataclass
    class Flatten:
        x: str
        y: FlattenChild = field(metadata = {"perde_flatten": True})
        z: int

    m.repack_type(Flatten)
