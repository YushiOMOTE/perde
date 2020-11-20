import pytest
from util import FORMATS, FORMATS_EXCEPT, FORMATS_ONLY


@pytest.mark.parametrize("m", FORMATS)
def test_error_no_args(m):
    with pytest.raises(TypeError) as e:
        m.dumps()
    assert e.value.args[
        0] == "dumps() requires 1 positional argument but got 0"

    with pytest.raises(TypeError) as e:
        m.loads()
    assert e.value.args[
        0] == "loads() requires 1 positional argument but got 0"

    with pytest.raises(TypeError) as e:
        m.loads_as()
    assert e.value.args[
        0] == "loads_as() requires 2 positional arguments but got 0"


@pytest.mark.parametrize("m", FORMATS)
def test_error_keyword_args(m):
    with pytest.raises(TypeError) as e:
        m.dumps(kw='a')
    assert e.value.args[0] == "dumps() takes no keyword arguments"

    with pytest.raises(TypeError) as e:
        m.loads(kw='a')
    assert e.value.args[0] == "loads() takes no keyword arguments"

    with pytest.raises(TypeError) as e:
        m.loads_as(kw='a')
    assert e.value.args[0] == "loads_as() takes no keyword arguments"


@pytest.mark.parametrize("m", FORMATS)
def test_error_dumps_wrong_number_of_args(m):
    with pytest.raises(TypeError) as e:
        m.dumps('a', 'b')
    assert e.value.args[
        0] == "dumps() requires 1 positional argument but got 2"

    with pytest.raises(TypeError) as e:
        m.loads('a', 'b')
    assert e.value.args[
        0] == "loads() requires 1 positional argument but got 2"

    with pytest.raises(TypeError) as e:
        m.loads_as('a')
    assert e.value.args[
        0] == "loads_as() requires 2 positional arguments but got 1"
    with pytest.raises(TypeError) as e:
        m.loads_as('a', 'b', 'c')
    assert e.value.args[
        0] == "loads_as() requires 2 positional arguments but got 3"


@pytest.mark.parametrize("m", FORMATS_EXCEPT("msgpack"))
def test_error_loads_invalid_argument_type(m):
    with pytest.raises(TypeError) as e:
        m.loads(b"a")
    assert e.value.args[
        0] == "invalid argument: expected `str` got `bytes`: b'a'"
    with pytest.raises(TypeError) as e:
        m.loads(any)
    assert e.value.args[0] == ("invalid argument: expected `str` "
                               "got `builtin_function_or_method`:"
                               " <built-in function any>")

    with pytest.raises(TypeError) as e:
        m.loads_as(str, b"a")
    assert e.value.args[
        0] == "invalid argument: expected `str` got `bytes`: b'a'"

    with pytest.raises(TypeError) as e:
        m.loads_as("b", "a")
    assert e.value.args[0] == "invalid argument: `b` is not a type"

    with pytest.raises(TypeError) as e:
        m.loads_as(any, "a")
    assert e.value.args[
        0] == "invalid argument: `<built-in function any>` is not a type"


@pytest.mark.parametrize("m", FORMATS_ONLY("msgpack"))
def test_error_loads_invalid_argument_type_msgpack(m):
    with pytest.raises(TypeError) as e:
        m.loads("a")
    assert e.value.args[0] == "invalid argument: expected `bytes` got `str`: a"


@pytest.mark.parametrize("m", FORMATS)
def test_error_unsupported_type(m):
    class Abc:
        pass

    with pytest.raises(TypeError) as e:
        m.dumps(Abc())
    assert e.value.args[0] == (
        "unsupported type "
        "`<class 'test_error.test_error_unsupported_type.<locals>.Abc'>`")
