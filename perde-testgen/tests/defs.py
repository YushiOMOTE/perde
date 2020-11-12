# Generated 2020-11-12T10:23:42.760438126+09:00

from dataclasses import dataclass, field
import perde
import typing


@perde.attr(rename_all="camelCase")
@dataclass
class Preset0:
    apple_pen: bool
    pen_pineapple: int


@dataclass
class Preset1:
    a: bool = field(metadata={"perde_rename": "hage"})
    b: int


@dataclass
class Preset3:
    x: bool
    y: int


@dataclass
class Preset2:
    a: Preset3 = field(metadata={"perde_flatten": True})
    b: int


TYPES = [Preset0, Preset1, Preset2]
