from dataclasses import dataclass, field
import perde
import pytest
from util import FORMATS
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
    @perde.attr(default=True)
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
        b: str = field(metadata={"perde_default": True})
        c: int

    p = m.unpack_data("DefaultConstruct", astype=DefaultConstruct2)
    assert p == DefaultConstruct2("xxx", "", 3)


@pytest.mark.parametrize("m", FORMATS)
def test_field_default_value(m):
    @dataclass
    class DefaultConstruct3:
        a: str
        b: str = field(default="hage")
        c: int = field(default=99)

    p = m.unpack_data("DefaultConstruct", astype=DefaultConstruct3)
    assert p == DefaultConstruct3("xxx", "hage", 3)


@pytest.mark.parametrize("m", FORMATS)
def test_field_default_factory(m):
    @dataclass
    class DefaultConstruct4:
        a: str
        b: str = field(default_factory=lambda: "hage")
        c: int = field(default=99)

    p = m.unpack_data("DefaultConstruct", astype=DefaultConstruct4)
    assert p == DefaultConstruct4("xxx", "hage", 3)


"""rust
#[derive(Serialize, Debug, new)]
struct Skip {
  x: String,
  y: u64,
  z: f64,
  a: String,
  b: String,
}

#[derive(Serialize, Debug, new)]
struct Skipped {
  x: String,
  #[serde(skip)]
  y: u64,
  z: f64,
  a: String,
  b: String,
}

#[derive(Serialize, Debug, new)]
struct SkipDefault {
  x: String,
  y: u64,
  z: f64,
  a: String,
  b: String,
}

add!(Skip {"ssssss".into(), 3, 1.1, "a".into(), "b".into()});
add!(Skipped {"ssssss".into(), 3, 1.1, "a".into(), "b".into()});
add!(SkipDefault {"ssssss".into(), 0, 1.1, "a".into(), "b".into()});
"""


@pytest.mark.parametrize("m", FORMATS)
def test_skip_with_default(m):
    @perde.attr(default=True)
    @dataclass
    class Skip:
        x: str
        y: int = field(metadata={"perde_skip": True})
        z: float
        a: str
        b: str

    p = m.unpack_data("Skip", astype=Skip)
    q = m.unpack_data("Skipped", astype=Skip)
    assert p == q
    assert p == Skip("ssssss", 0, 1.1, "a", "b")
    assert m.dumps(p) == m.data("Skipped")


@pytest.mark.parametrize("m", FORMATS)
def test_field_skip_with_default(m):
    @dataclass
    class Skip:
        x: str
        y: int = field(metadata={"perde_skip": True, "perde_default": True})
        z: float
        a: str
        b: str

    p = m.unpack_data("Skip", astype=Skip)
    q = m.unpack_data("Skipped", astype=Skip)
    assert p == q
    assert p == Skip("ssssss", 0, 1.1, "a", "b")
    assert m.dumps(p) == m.data("Skipped")


@pytest.mark.parametrize("m", FORMATS)
def test_field_skip_with_default_value(m):
    @dataclass
    class Skip:
        x: str
        y: int = field(default=4, metadata={"perde_skip": True})
        z: float = field(default=1.3)
        a: str = field(default_factory=lambda: "aaxx")
        b: str = field(default_factory=lambda: "bbcc")

    p = m.unpack_data("Skip", astype=Skip)
    q = m.unpack_data("Skipped", astype=Skip)
    assert p == q
    assert p == Skip("ssssss", 4, 1.1, "a", "b")
    assert m.dumps(p) == m.data("Skipped")


@pytest.mark.parametrize("m", FORMATS)
def test_field_skip_with_default_factory(m):
    @dataclass
    class Skip:
        x: str
        y: int = field(default_factory=lambda: 400,
                       metadata={"perde_skip": True})
        z: float = field(default=1.3)
        a: str = field(default_factory=lambda: "aaxx")
        b: str = field(default_factory=lambda: "bbcc")

    p = m.unpack_data("Skip", astype=Skip)
    q = m.unpack_data("Skipped", astype=Skip)
    assert p == q
    assert p == Skip("ssssss", 400, 1.1, "a", "b")
    assert m.dumps(p) == m.data("Skipped")


@pytest.mark.parametrize("m", FORMATS)
def test_field_skip_serializing(m):
    @dataclass
    class Skip:
        x: str
        y: int = field(metadata={"perde_skip_serializing": True})
        z: float
        a: str
        b: str

    p = m.unpack_data("Skip", astype=Skip)
    with pytest.raises(RuntimeError):
        m.unpack_data("Skipped", astype=Skip)
    assert p == Skip("ssssss", 3, 1.1, "a", "b")
    assert m.dumps(p) == m.data("Skipped")


@pytest.mark.parametrize("m", FORMATS)
def test_skip_deserializing_with_default(m):
    @perde.attr(default=True)
    @dataclass
    class Skip:
        x: str
        y: int = field(metadata={"perde_skip_deserializing": True})
        z: float
        a: str
        b: str

    p = m.unpack_data("Skip", astype=Skip)
    q = m.unpack_data("Skipped", astype=Skip)
    assert p == q
    assert p == Skip("ssssss", 0, 1.1, "a", "b")
    assert m.dumps(p) == m.data("SkipDefault")
