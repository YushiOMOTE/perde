import enum
import perde
import pytest
from util import FORMATS_EXCEPT

"""rust
add_value("EnumX", "X");
add_value("EnumY", "Y");
add_value("EnumZ", "Z");

add_value("EnumXValue", "hi");
add_value("EnumYValue", "foo");
add_value("EnumZValue", 3);
"""


@pytest.mark.parametrize("m", FORMATS_EXCEPT("toml"))
def test_enum(m):
    class E(enum.Enum):
        X = "hi"
        Y = "foo"
        Z = 3

    m.repack_as(E, E.X)
    m.repack_as(E, E.Y)
    m.repack_as(E, E.Z)
    assert m.data("EnumX") == m.dumps(E.X)
    assert m.data("EnumY") == m.dumps(E.Y)
    assert m.data("EnumZ") == m.dumps(E.Z)


@pytest.mark.parametrize("m", FORMATS_EXCEPT("toml"))
def test_enum_value(m):
    @perde.attr(as_value=True)
    class E(enum.Enum):
        X = "hi"
        Y = "foo"
        Z = 3

    m.repack_as(E, E.X)
    m.repack_as(E, E.Y)
    m.repack_as(E, E.Z)
    assert m.data("EnumXValue") == m.dumps(E.X)
    assert m.data("EnumYValue") == m.dumps(E.Y)
    assert m.data("EnumZValue") == m.dumps(E.Z)


"""rust
add_value("EnumYRename", "Yay");
"""


@pytest.mark.parametrize("m", FORMATS_EXCEPT("toml"))
def test_enum_rename(m):
    class E(perde.Enum):
        X = "hi"
        Y = "foo", {"perde_rename": "Yay"}
        Z = 3

    m.repack_as(E, E.X)
    m.repack_as(E, E.Y)
    m.repack_as(E, E.Z)
    assert m.data("EnumX") == m.dumps(E.X)
    assert m.data("EnumYRename") == m.dumps(E.Y)
    assert m.data("EnumZ") == m.dumps(E.Z)


"""rust
add_value("EnumRenameAllX", "pan-piano");
add_value("EnumRenameAllY", "pan-piano-good");
add_value("EnumRenameAllZ", "pan-piano-excellent");
"""


@pytest.mark.parametrize("m", FORMATS_EXCEPT("toml"))
def test_enum_rename_all(m):
    @perde.attr(rename_all="kebab-case")
    class E(perde.Enum):
        PanPiano = "itta"
        PanPianoGood = "yatta"
        PanPianoExcellent = "sorena"

    m.repack_as(E, E.PanPiano)
    m.repack_as(E, E.PanPianoGood)
    m.repack_as(E, E.PanPianoExcellent)
    assert m.data("EnumRenameAllX") == m.dumps(E.PanPiano)
    assert m.data("EnumRenameAllY") == m.dumps(E.PanPianoGood)
    assert m.data("EnumRenameAllZ") == m.dumps(E.PanPianoExcellent)


"""rust
add_value("EnumRenameAllYRename", "PaiPai");
"""


@pytest.mark.parametrize("m", FORMATS_EXCEPT("toml"))
def test_enum_rename_in_rename_all(m):
    @perde.attr(rename_all="kebab-case")
    class E(perde.Enum):
        PanPiano = "itta"
        PanPianoGood = "yatta", {"perde_rename": "PaiPai"}
        PanPianoExcellent = "sorena"

    m.repack_as(E, E.PanPiano)
    m.repack_as(E, E.PanPianoGood)
    m.repack_as(E, E.PanPianoExcellent)
    assert m.data("EnumRenameAllX") == m.dumps(E.PanPiano)
    assert m.data("EnumRenameAllYRename") == m.dumps(E.PanPianoGood)
    assert m.data("EnumRenameAllZ") == m.dumps(E.PanPianoExcellent)


"""rust
add_value("EnumRenameAllXRaw", "PanPiano");
add_value("EnumRenameAllYRaw", "PanPianoGood");
add_value("EnumRenameAllZRaw", "PanPianoExcellent");
"""


@pytest.mark.parametrize("m", FORMATS_EXCEPT("toml"))
def test_enum_rename_all_serialize(m):
    @perde.attr(rename_all_serialize="kebab-case")
    class E(perde.Enum):
        PanPiano = "itta"
        PanPianoGood = "yatta"
        PanPianoExcellent = "sorena"

    assert m.loads_as(E, m.data("EnumRenameAllXRaw")) == E.PanPiano
    assert m.loads_as(E, m.data("EnumRenameAllYRaw")) == E.PanPianoGood
    assert m.loads_as(E, m.data("EnumRenameAllZRaw")) == E.PanPianoExcellent
    assert m.data("EnumRenameAllX") == m.dumps(E.PanPiano)
    assert m.data("EnumRenameAllY") == m.dumps(E.PanPianoGood)
    assert m.data("EnumRenameAllZ") == m.dumps(E.PanPianoExcellent)


@pytest.mark.parametrize("m", FORMATS_EXCEPT("toml"))
def test_enum_rename_all_deserialize(m):
    @perde.attr(rename_all_deserialize="kebab-case")
    class E(perde.Enum):
        PanPiano = "itta"
        PanPianoGood = "yatta"
        PanPianoExcellent = "sorena"

    assert m.loads_as(E, m.data("EnumRenameAllX")) == E.PanPiano
    assert m.loads_as(E, m.data("EnumRenameAllY")) == E.PanPianoGood
    assert m.loads_as(E, m.data("EnumRenameAllZ")) == E.PanPianoExcellent
    assert m.data("EnumRenameAllXRaw") == m.dumps(E.PanPiano)
    assert m.data("EnumRenameAllYRaw") == m.dumps(E.PanPianoGood)
    assert m.data("EnumRenameAllZRaw") == m.dumps(E.PanPianoExcellent)


@pytest.mark.parametrize("m", FORMATS_EXCEPT("toml"))
def test_enum_skip(m):
    class E(perde.Enum):
        X = "hi"
        Y = "foo", {"perde_skip": True}
        Z = 3

    m.repack_as(E, E.X)
    m.repack_as(E, E.Z)
    with pytest.raises(Exception):
        m.dumps(E.Y)
    with pytest.raises(Exception) as e:
        m.loads_as(E, m.data("EnumY"))
    print(f"{e}")


@pytest.mark.parametrize("m", FORMATS_EXCEPT("toml"))
def test_enum_skip_serializing(m):
    class E(perde.Enum):
        X = "hi"
        Y = "foo", {"perde_skip_serializing": True}
        Z = 3

    m.repack_as(E, E.X)
    m.repack_as(E, E.Z)
    with pytest.raises(Exception):
        m.dumps(E.Y)
    assert E.Y == m.loads_as(E, m.data("EnumY"))


@pytest.mark.parametrize("m", FORMATS_EXCEPT("toml"))
def test_enum_skip_deserializing(m):
    class E(perde.Enum):
        X = "hi"
        Y = "foo", {"perde_skip_deserializing": True}
        Z = 3

    m.repack_as(E, E.X)
    m.repack_as(E, E.Z)
    assert m.data("EnumY") == m.dumps(E.Y)
    with pytest.raises(Exception) as e:
        m.loads_as(E, m.data("EnumY"))
    print(f"{e}")


"""rust
add_value("Other", "fafafafa");
"""


@pytest.mark.parametrize("m", FORMATS_EXCEPT("toml"))
def test_enum_skip_other(m):
    class E(perde.Enum):
        X = "hi"
        Y = "foo"
        Z = 3
        P = 100, {"perde_other": True}

    m.repack_as(E, E.X)
    m.repack_as(E, E.Y)
    m.repack_as(E, E.Z)
    assert E.P == m.loads_as(E, m.data("Other"))
