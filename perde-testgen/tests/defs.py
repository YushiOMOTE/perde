from dataclasses import dataclass, field
import perde
import typing


@perde.attr(rename="Potato")
@dataclass
class Water:
    pass


@perde.attr(rename_all="UPPERCASE", deny_unknown_fields=True)
@dataclass
class Bed:
    pebble: typing.Union[float, Water, typing.Optional[float]]


@perde.attr(rename_all="UPPERCASE", deny_unknown_fields=True)
@dataclass
class Shower:
    sword: typing.Union[typing.Set[bytes], Bed] = field(
        metadata={"perde_rename": "egg"})


@perde.attr(rename_all="lowercase", default=True)
@dataclass
class Shower1:
    pass


@perde.attr(rename_all="SCREAMING-KEBAB-CASE", rename="Kitchen", default=True)
@dataclass
class Rifle:
    god: typing.Optional[Shower1] = field(metadata={"perde_skip": True})
    swimmingpool: typing.Set[bool]


@perde.attr(rename="Baby2")
@dataclass
class Leg:
    backpack: typing.Dict[str, typing.
                          Union[bytes, typing.List[bytes]]] = field(
                              metadata={"perde_rename": "carrier"})


@perde.attr(rename_all="lowercase", default=True)
@dataclass
class Bathroom:
    pass


@perde.attr(rename_all="snake_case", rename="Church", default=True)
@dataclass
class LeatherJacket:
    rope: bytes = field(metadata={"perde_skip": True})
    map: Bathroom


@perde.attr(rename_all="PascalCase", deny_unknown_fields=True)
@dataclass
class Planet1:
    ice: float


@perde.attr(rename="Chisel", deny_unknown_fields=True)
@dataclass
class Ears:
    rope1: typing.Tuple[()]
    computer: float


@perde.attr(rename_all="kebab-case", deny_unknown_fields=True)
@dataclass
class Gate:
    chisel: typing.Dict[str, typing.List[bool]]


@perde.attr(rename_all="PascalCase", deny_unknown_fields=True)
@dataclass
class Rainbow:
    maze: str


@perde.attr(rename_all="lowercase", rename="Coffee", default=True)
@dataclass
class Square:
    slave: str = field(metadata={"perde_rename": "surveyor"})
    parachute: bytes = field(metadata={"perde_skip": True})


@perde.attr(rename_all="UPPERCASE")
@dataclass
class Explosive:
    pass


@perde.attr(rename_all="snake_case", default=True)
@dataclass
class Liquid:
    pass


@perde.attr(rename_all="SCREAMING-KEBAB-CASE", default=True)
@dataclass
class Album:
    pass


@perde.attr(
    rename_all="SCREAMING-KEBAB-CASE",
    rename="Church2",
    deny_unknown_fields=True)
@dataclass
class Pepper:
    circus: int = field(metadata={"perde_rename": "butterfly"})
    car: str = field(metadata={"perde_skip": True})


@perde.attr(rename_all="PascalCase", rename="Rock")
@dataclass
class Shower2:
    vampire1: Pepper = field(metadata={"perde_skip": True})
    woman: bytes


@perde.attr(rename_all="lowercase", rename="Shoes")
@dataclass
class Spoon1:
    highway: int = field(metadata={"perde_skip": True})
    hose: Shower2


@perde.attr(rename_all="lowercase", rename="Parachute")
@dataclass
class Signature1:
    pass


@perde.attr(rename_all="SCREAMING-KEBAB-CASE", default=True)
@dataclass
class Circle:
    pass


@perde.attr(rename_all="UPPERCASE", deny_unknown_fields=True)
@dataclass
class Window:
    icecream: typing.Dict[str, typing.
                          Union[int, str, typing.Set[bool], typing.
                                Optional[typing.Union[int, typing.Tuple[(
                                )], typing.Union[float, str]]]]]


@perde.attr(rename_all="SCREAMING_SNAKE_CASE", rename="Potato1")
@dataclass
class Bird:
    flower1: float


@perde.attr(rename_all="snake_case", default=True)
@dataclass
class Meteor:
    tunnel: typing.Set[str] = field(metadata={"perde_rename": "diamond"})
    highway1: int = field(metadata={"perde_skip": True})


@perde.attr(rename_all="UPPERCASE")
@dataclass
class Man:
    pass


@perde.attr(rename_all="snake_case")
@dataclass
class Sword:
    pass


@perde.attr(rename_all="lowercase")
@dataclass
class Feather1:
    tunnel1: bool = field(metadata={"perde_skip": True})
    planet2: typing.Union[bool, int, Sword, typing.
                          Optional[typing.List[str]]] = field(
                              metadata={"perde_rename": "signature"})


@perde.attr(rename_all="PascalCase", rename="Printer")
@dataclass
class Pillow:
    electricity: int = field(metadata={"perde_skip_deserializing": True})
    aeroplane: bytes = field(metadata={"perde_skip_deserializing": True})


@perde.attr(rename_all="PascalCase", rename="Flower1", default=True)
@dataclass
class Airforce:
    car1: typing.Set[bool] = field(metadata={"perde_rename": "map1"})


@perde.attr(
    rename_all="lowercase",
    rename="Electricity1",
    deny_unknown_fields=True,
    default=True)
@dataclass
class Library:
    rifle: bool


@perde.attr(rename_all="lowercase", rename="Aeroplane")
@dataclass
class Milk:
    vulture1: typing.Union[int, typing.Tuple[float, typing.Tuple[bool]],
                           Airforce, Library] = field(
                               metadata={"perde_rename": "pillow"})
    coffee: bytes


@perde.attr(rename_all="SCREAMING-KEBAB-CASE")
@dataclass
class Horse1:
    spectrum: Milk = field(metadata={"perde_flatten": True})


@perde.attr(rename_all="lowercase", rename="Computer")
@dataclass
class Button:
    floodlight: bool


@perde.attr(rename_all="UPPERCASE", rename="Vacuum1")
@dataclass
class Pendulum1:
    pass


@dataclass
class Typewriter2:
    pass


TYPES = [
    float, bool, int, str, Shower, float, str, str, typing.Tuple[Rifle, int],
    typing.Optional[str], bytes, Leg,
    typing.Union[bool, float, typing.Dict[str, float], typing.Set[str]],
    typing.Union[bool, int, typing.Dict[str, int]], bytes, int, bytes, int,
    typing.Union[bool, typing.
                 Tuple[typing.Dict[str, typing.List[LeatherJacket]]], Planet1],
    typing.Dict[str, typing.Set[str]], bool, typing.List[typing.List[bool]],
    bytes, typing.List[typing.Union[
        float, bytes, typing.
        Union[typing.Dict[str, typing.Dict[str, float]], typing.
              List[typing.List[typing.Set[bool]]], Ears, typing.
              Union[bytes, typing.List[typing.Tuple[bytes]], Gate, typing.
                    Optional[Rainbow]]]]], bool, bool, typing.Set[bytes],
    typing.Set[bool], Square, typing.Optional[typing.Dict[str, bool]],
    typing.Optional[Explosive], typing.Set[bool],
    typing.Union[int, float, typing.Dict[str, bool], typing.Optional[bool]],
    typing.Union[bool, typing.List[typing.Set[str]], typing.
                 Tuple[int, typing.Optional[str]], Liquid],
    typing.Dict[str, typing.List[bool]], int,
    typing.Union[bool, Album, Spoon1], bytes, Signature1, bytes,
    typing.List[typing.Union[int, typing.List[
        typing.Dict[str, typing.Union[bool, int, float]]], typing.Optional[
            typing.Union[int, bytes, typing.Optional[str]]]]], Circle,
    typing.Optional[str], typing.Set[int], str, Window,
    typing.Union[bool, typing.Dict[str, int], typing.Set[str], Bird], bytes,
    bytes, typing.Set[bytes], float, typing.Optional[bool], int,
    typing.Optional[str], str, typing.Set[int], bytes,
    typing.Tuple[typing.Optional[typing.Tuple[()]], float], int,
    typing.Union[float, bytes, Meteor],
    typing.Union[int, bytes, typing.Tuple[()]], Man, typing.Optional[str],
    Feather1, int, typing.Tuple[()], typing.Tuple[()],
    typing.Union[bool, int, bytes, typing.Optional[typing.List[bool]]], float,
    typing.List[typing.Union[bytes, typing.List[bytes], typing.
                             Tuple[int, bytes]]], str,
    typing.Tuple[bytes, typing.Dict[str, bool]], bool, str, typing.List[str],
    bytes, bytes, typing.Optional[typing.Optional[bytes]], Pillow, bool,
    typing.Union[bool, float, typing.Set[str], typing.Optional[str]],
    typing.Dict[str, typing.
                Union[bool, typing.
                      Dict[str, typing.
                           Optional[typing.Optional[str]]], Horse1, Button]],
    typing.List[str], typing.Set[bool], str, typing.Dict[str, float],
    typing.Set[int], Pendulum1, int, bool,
    typing.Dict[str, typing.Union[int, bytes, typing.
                                  Union[bool, float, bytes, typing.Set[int]]]],
    typing.Union[int, bytes, typing.Set[bool]], float, typing.Set[bool],
    typing.List[str], float, typing.List[typing.List[str]], str,
    typing.Tuple[typing.List[typing.List[str]], typing.
                 List[typing.List[typing.Tuple[typing.Tuple[(
                 )], Typewriter2]]]], float
]
