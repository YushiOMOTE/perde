from enum import Enum
from dataclasses import dataclass
import typing
import pytest
from util import FORMATS, repack, repack_as


@pytest.mark.parametrize("m", FORMATS)
def test_union_empty(m):
    @dataclass
    class N:
        pass

    @dataclass
    class M:
        a: int
        b: typing.Union[float, int, typing.Optional[float]]

    repack_as(m, M, M(3, 3.2))
    repack_as(m, M, M(3, None))
