from dataclasses import dataclass, field
from typing import Dict
import perde
import pytest
from util import FORMATS, FORMATS_EXCEPT
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
#[serde(rename_all = "camelCase")]
struct RenameAll {
  pen_pineapple: String,
  apple_pen: String,
}

add!(RenameAll {"xxx".into(), "yyy".into()});
"""


@pytest.mark.parametrize("m", FORMATS)
def test_rename_all(m):
    @perde.attr(rename_all="camelCase")
    @dataclass
    class RenameAll:
        pen_pineapple: str
        apple_pen: str

    m.repack_type(RenameAll)


"""rust
#[derive(Serialize, Debug, new)]
#[serde(rename = "RenameAllSerialize", rename_all = "PascalCase")]
struct RenameAllSerializeOutput {
  pen_pineapple: String,
  apple_pen: String,
}

#[derive(Serialize, Debug, new)]
#[serde(rename = "RenameAllSerialize")]
struct RenameAllSerializeInput {
  pen_pineapple: String,
  apple_pen: String,
}

add!(RenameAllSerializeInput {"--".into(), "==".into()});
add!(RenameAllSerializeOutput {"--".into(), "==".into()});
"""


@pytest.mark.parametrize("m", FORMATS)
def test_rename_all_serialize(m):
    @perde.attr(rename_all_serialize="PascalCase")
    @dataclass
    class RenameAllSerialize:
        pen_pineapple: str
        apple_pen: str

    d = m.unpack_data("RenameAllSerializeInput", astype=RenameAllSerialize)
    v = m.dumps(d)
    e = m.data("RenameAllSerializeOutput")
    assert v == e


"""rust
#[derive(Serialize, Debug, new)]
#[serde(rename = "RenameAllDeserialize")]
struct RenameAllDeserializeOutput {
  pen_pineapple: String,
  apple_pen: String,
}

#[derive(Serialize, Debug, new)]
#[serde(rename = "RenameAllDeserialize", rename_all = "SCREAMING_SNAKE_CASE")]
struct RenameAllDeserializeInput {
  pen_pineapple: String,
  apple_pen: String,
}

add!(RenameAllDeserializeInput {"--".into(), "==".into()});
add!(RenameAllDeserializeOutput {"--".into(), "==".into()});
"""


@pytest.mark.parametrize("m", FORMATS)
def test_rename_all_deserialize(m):
    @perde.attr(rename_all_deserialize="SCREAMING_SNAKE_CASE")
    @dataclass
    class RenameAllDeserialize:
        pen_pineapple: str
        apple_pen: str

    d = m.unpack_data("RenameAllDeserializeInput", astype=RenameAllDeserialize)
    v = m.dumps(d)
    e = m.data("RenameAllDeserializeOutput")
    assert v == e


"""rust
#[derive(Serialize, Debug, new)]
struct DenyUnknownFields {
  x: String,
  y: i64,
  z: i64,
  q: String,
}

add!(DenyUnknownFields {"aaaaa".into(), 1, -2, "unknown".into()});
"""


@pytest.mark.parametrize("m", FORMATS)
def test_deny_unknown_fields(m):
    @dataclass
    class NoDenyUnknownFields:
        x: str
        y: int
        z: int

    @perde.attr(deny_unknown_fields=True)
    @dataclass
    class DenyUnknownFields:
        x: str
        y: int
        z: int

    e = m.unpack_data("DenyUnknownFields", astype=NoDenyUnknownFields)
    assert e == NoDenyUnknownFields("aaaaa", 1, -2)
    with pytest.raises(RuntimeError) as e:
        m.unpack_data("DenyUnknownFields", astype=DenyUnknownFields)
    print(f'{e}')


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
        b: str = field(metadata={"perde_rename": "x"})
        c: int

    m.repack_type(Rename)


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
    @perde.attr(rename_all="camelCase")
    @dataclass
    class RenameAllRename:
        pen_pineapple: str
        apple_pen: str = field(metadata={"perde_rename": "pen_pen"})

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

add!(NestedRename
     {"xxx".into(),
      NestedRenameChild::new("ppp".into(), "qqq".into()),
      1111}
     except "toml");
"""


@pytest.mark.parametrize("m", FORMATS_EXCEPT("toml"))
def test_nested_rename(m):
    @dataclass
    class NestedRenameChild:
        a: str
        b: str = field(metadata={"perde_rename": "d"})

    @dataclass
    class NestedRename:
        x: str
        y: NestedRenameChild = field(metadata={"perde_rename": "w"})
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

add!(NestedRenameAll
     {"xxx".into(),
      NestedRenameAllChild::new("ppp".into(), "qqq".into()),
      1111}
     except "toml");
"""


@pytest.mark.parametrize("m", FORMATS_EXCEPT("toml"))
def test_nested_rename_all(m):
    @perde.attr(rename_all="UPPERCASE")
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

add!(Flatten
     {"xxx".into(),
      FlattenChild::new("ppp".into(), "qqq".into()),
      1111}
     except "msgpack");
"""


@pytest.mark.parametrize("m", FORMATS_EXCEPT("msgpack"))
def test_flatten(m):
    @dataclass
    class FlattenChild:
        a: str
        b: str

    @dataclass
    class Flatten:
        x: str
        y: FlattenChild = field(metadata={"perde_flatten": True})
        z: int

    m.repack_type(Flatten)


"""rust
#[derive(Serialize, Debug, new)]
struct DictFlatten {
  x: String,
  y: i64,
  #[serde(flatten)]
  z: IndexMap<String, String>,
}

add!(DictFlatten {"hey".into(), -103223,
    {
     let mut m = IndexMap::new();
     m.insert("pp".into(), "q1".into());
     m.insert("ppp".into(), "q2".into());
     m.insert("pppp".into(), "q3".into());
     m
    }}
     except "msgpack");
"""


@pytest.mark.parametrize("m", FORMATS_EXCEPT("msgpack"))
def test_dict_flatten(m):
    @dataclass
    class DictFlatten:
        x: str
        y: int
        z: Dict[str, str] = field(metadata={"perde_flatten": True})

    m.repack_type(DictFlatten)
