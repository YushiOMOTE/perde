from dataclasses import dataclass, field
from typing import List, Dict, Optional, Union, Tuple, TypeVar
import enum
import perde
import pytest

from util import repack, comp


def test_rename():
    @dataclass
    class Test:
        a: str
        b: str = field(metadata = {"perde_rename": "x"})
        c: int

    comp(repack(Test, "yes", "no", 3), {"a":"yes","x":"no","c":3})


def test_rename_all():
    @perde.attr(rename_all="camelCase")
    @dataclass
    class Test:
        this_is_it: str
        that_is_what: str
        this_was_that: int

    comp(repack(Test, "yes", "no", 3), {"thisIsIt":"yes","thatIsWhat":"no","thisWasThat":3})


def test_rename_all_and_rename():
    @perde.attr(rename_all="camelCase")
    @dataclass
    class Test:
        this_is_it: str
        that_is_what: str = field(metadata = {"perde_rename": "that_is_which"})
        this_was_that: int

    comp(repack(Test, "yes", "no", 3), {"thisIsIt":"yes","that_is_which":"no","thisWasThat":3})


def test_nested_rename():
    @dataclass
    class Test2:
        a: str
        b: str = field(metadata = {"perde_rename": "d"})
        c: int

    @dataclass
    class Test:
        x: str
        y: Test2 = field(metadata = {"perde_rename": "w"})
        z: int

    comp(repack(Test, "yes", Test2("faa", "foo", -10), 3), {"x":"yes","w":{"a":"faa","d":"foo","c":-10},"z":3})


def test_nested_rename_all():
    @perde.attr(rename_all="camelCase")
    @dataclass
    class Test2:
        a_k: str
        b_k: str
        c_k: int

    @dataclass
    class Test:
        a: str
        b: Test2
        c: int

    comp(repack(Test, "yes", Test2("he", "she", 0), 3), {"a":"yes","b":{"aK":"he","bK":"she","cK":0},"c":3})
