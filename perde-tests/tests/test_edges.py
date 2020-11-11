from enum import Enum
from dataclasses import dataclass, field
import typing
import pytest
import perde
from util import FORMATS, repack, repack_as


@pytest.mark.parametrize("m", FORMATS)
def test_union_flatten(m):
    @dataclass
    class N:
        pass

    @dataclass
    class M:
        a: int
        b: typing.Union[float, int, typing.Optional[float]]

    repack_as(m, M, M(3, 3.2))
    repack_as(m, M, M(3, None))


@pytest.mark.parametrize("m", FORMATS)
def test_empty_tuple(m):
    @dataclass
    class Fruit:
        horoscope: typing.Union[float, typing.Set[int], typing.Tuple[()], int]

    repack_as(m, Fruit, Fruit(2))
