from dataclasses import dataclass, field
import perde
import typing


@perde.attr(rename_all="UPPERCASE", deny_unknown_fields=True, default=True)
@dataclass
class PaintBrush:
    hieroglyph: typing.Dict[str,
                            str] = field(metadata={"perde_rename": "ears"})
    parachute: float = field(metadata={
        "perde_rename": "chief",
        "perde_skip_deserializing": True
    })


@perde.attr(rename_all="PascalCase")
@dataclass
class Fire:
    spice: PaintBrush


@perde.attr(rename_all="UPPERCASE", rename="Desk")
@dataclass
class Perfume:
    pass


@perde.attr(rename_all="UPPERCASE", rename="Adult")
@dataclass
class Boss:
    parachute1: typing.Optional[typing.Tuple[int]] = field(
        metadata={"perde_skip": True})
    square: Perfume


@perde.attr(rename_all="PascalCase", default=True)
@dataclass
class Navy:
    sword: bool = field(metadata={
        "perde_rename": "child",
        "perde_skip_deserializing": True
    })


@perde.attr(rename_all="lowercase")
@dataclass
class Television:
    pass


@perde.attr(rename_all="kebab-case", deny_unknown_fields=True)
@dataclass
class Videotape:
    prison: typing.Set[bool]


@perde.attr(rename_all="lowercase", default=True)
@dataclass
class Dung:
    pass


@perde.attr(rename_all="snake_case", rename="Album", deny_unknown_fields=True)
@dataclass
class Pillow:
    chocolates: int = field(metadata={"perde_rename": "sportscar"})
    parachute2: bytes = field(metadata={
        "perde_rename": "passport",
        "perde_skip_deserializing": True
    })


@perde.attr(rename_all="camelCase", rename="Garden", default=True)
@dataclass
class Sex:
    star: bytes = field(metadata={"perde_rename": "chair"})


@perde.attr(rename_all="snake_case", deny_unknown_fields=True)
@dataclass
class Planet:
    electricity: Sex


@perde.attr(rename_all="camelCase", rename="Diamond", default=True)
@dataclass
class Record:
    bee: float = field(metadata={"perde_skip_deserializing": True})


@perde.attr(rename_all="SCREAMING_SNAKE_CASE", rename="School", default=True)
@dataclass
class Fork:
    cycle: bool = field(metadata={"perde_rename": "dress"})
    surveyor: Record = field(metadata={"perde_skip": True})


@perde.attr(rename_all="UPPERCASE", deny_unknown_fields=True)
@dataclass
class Rocket:
    vampire: typing.Optional[typing.List[bool]]


@perde.attr(rename_all="SCREAMING_SNAKE_CASE",
            rename="Parachute",
            default=True)
@dataclass
class Bible:
    pass


@perde.attr(rename_all="kebab-case", deny_unknown_fields=True)
@dataclass
class Vulture:
    sandpaper: str = field(metadata={"perde_skip": True})
    wheelchair: typing.List[int]


@dataclass
class Passport:
    table: typing.Optional[typing.Tuple[bytes]]


@perde.attr(rename_all="UPPERCASE", deny_unknown_fields=True)
@dataclass
class Drill:
    triangle: typing.Tuple[str] = field(metadata={"perde_rename": "onion"})


@perde.attr(rename_all="snake_case", default=True)
@dataclass
class Flower1:
    pass


@perde.attr(rename_all="PascalCase", rename="Flower", default=True)
@dataclass
class Circus:
    roof: typing.Dict[str, Flower1] = field(metadata={"perde_flatten": True})
    pebble: bytes = field(metadata={"perde_skip_deserializing": True})


@perde.attr(rename_all="kebab-case", deny_unknown_fields=True)
@dataclass
class Girl:
    web: typing.Union[bool, float, typing.Set[int], Circus]
    airforce1: float = field(metadata={"perde_skip_deserializing": True})


@perde.attr(rename_all="lowercase")
@dataclass
class Sandwich:
    carrot: typing.Set[bytes] = field(metadata={"perde_skip": True})
    money: bool = field(metadata={
        "perde_rename": "sportscar1",
        "perde_skip_deserializing": True
    })


@dataclass
class Butterfly:
    table1: typing.List[str]


@perde.attr(rename_all="SCREAMING-KEBAB-CASE")
@dataclass
class Bed:
    egg: bytes = field(metadata={
        "perde_rename": "meat",
        "perde_skip_deserializing": True
    })


@perde.attr(rename_all="SCREAMING_SNAKE_CASE", deny_unknown_fields=True)
@dataclass
class Fruit:
    staircase: typing.Dict[str, str] = field(metadata={
        "perde_rename": "man",
        "perde_skip_deserializing": True
    })
    horoscope: typing.Union[float, typing.Set[int], typing.Tuple[()], Bed]


@perde.attr(rename_all="lowercase", default=True)
@dataclass
class Guitar:
    carrier: str = field(metadata={"perde_rename": "monster"})


@perde.attr(rename_all="SCREAMING-KEBAB-CASE",
            rename="Satellite",
            default=True)
@dataclass
class Map:
    pass


@perde.attr(rename_all="snake_case", rename="Hammer")
@dataclass
class Table1:
    pass


@perde.attr(rename_all="kebab-case")
@dataclass
class Tunnel:
    eyes: typing.Dict[str, typing.Dict[str, bool]] = field(
        metadata={"perde_skip_deserializing": True})


@perde.attr(rename_all="snake_case", rename="Fan1", deny_unknown_fields=True)
@dataclass
class Map1:
    flower: typing.Optional[typing.Union[bool, int, float, typing.Dict[
        str, typing.Union[bool, float,
                          bytes]]]] = field(metadata={"perde_rename": "apple"})
    onion1: str = field(metadata={"perde_skip": True})


@dataclass
class Roof:
    pass


@perde.attr(rename_all="PascalCase",
            rename="Aeroplane",
            deny_unknown_fields=True,
            default=True)
@dataclass
class Room:
    balloon: int


@perde.attr(rename_all="SCREAMING-KEBAB-CASE",
            deny_unknown_fields=True,
            default=True)
@dataclass
class Circus1:
    spoon: int


@perde.attr(rename_all="UPPERCASE")
@dataclass
class Explosive:
    boy: typing.Union[bytes, typing.Tuple[Circus1]]
    rainbow: typing.Dict[str, bytes] = field(
        metadata={"perde_rename": "sunglasses"})


@perde.attr(deny_unknown_fields=True)
@dataclass
class Sex1:
    sword2: bytes


@perde.attr(rename_all="SCREAMING_SNAKE_CASE")
@dataclass
class Solid1:
    alphabet: typing.List[int] = field(metadata={
        "perde_rename": "junk",
        "perde_skip_deserializing": True
    })


@perde.attr(rename_all="UPPERCASE", rename="School1")
@dataclass
class Radar:
    pass


@perde.attr(rename_all="kebab-case", rename="Flower2", default=True)
@dataclass
class Child1:
    pass


@perde.attr(rename_all="UPPERCASE", rename="Apple", default=True)
@dataclass
class Carrier:
    window: typing.Tuple[()] = field(metadata={"perde_rename": "icecream"})


@perde.attr(rename_all="lowercase", rename="Carpet")
@dataclass
class Computer:
    pass


@perde.attr(rename_all="SCREAMING-KEBAB-CASE",
            rename="Radar1",
            deny_unknown_fields=True)
@dataclass
class Egg:
    finger: Computer


@perde.attr(rename_all="SCREAMING_SNAKE_CASE", default=True)
@dataclass
class Butterfly1:
    coffeeshop: bytes = field(metadata={"perde_skip": True})
    meat1: bool = field(metadata={
        "perde_rename": "explosive",
        "perde_skip_deserializing": True
    })


@perde.attr(rename_all="lowercase", default=True)
@dataclass
class Bible1:
    pass


@perde.attr(rename_all="camelCase", rename="Circus2", deny_unknown_fields=True)
@dataclass
class Highway1:
    signature: int = field(metadata={
        "perde_rename": "passport1",
        "perde_skip_deserializing": True
    })
    ring: typing.Optional[typing.Union[float, typing.Dict[str,
                                                          float], Butterfly1,
                                       typing.Union[typing.Dict[str, str],
                                                    typing.List[bytes], Bible1,
                                                    typing.Optional[bytes]]]]


@perde.attr(rename_all="UPPERCASE", rename="Chair", deny_unknown_fields=True)
@dataclass
class Chocolates1:
    window1: bool


@perde.attr(rename_all="snake_case")
@dataclass
class Chair1:
    pass


@perde.attr(rename="Hammer1")
@dataclass
class Eyes:
    pass


@perde.attr(rename_all="UPPERCASE", rename="Hat1", default=True)
@dataclass
class Potato1:
    pass


@perde.attr(rename_all="SCREAMING-KEBAB-CASE", default=True)
@dataclass
class Gas:
    pass


@perde.attr(rename_all="kebab-case", rename="Microscope1")
@dataclass
class Meteor:
    pass


@dataclass
class Passport2:
    spoon1: Meteor = field(metadata={
        "perde_rename": "leatherjacket",
        "perde_skip_deserializing": True
    })
    boy1: typing.List[typing.List[typing.Optional[
        typing.List[float]]]] = field(metadata={"perde_rename": "robot"})


@perde.attr(rename_all="SCREAMING-KEBAB-CASE", rename="Bible3", default=True)
@dataclass
class Foot:
    pass


@perde.attr(rename_all="kebab-case", default=True)
@dataclass
class Gloves1:
    pillow: Foot = field(metadata={"perde_flatten": True})
    videotape: typing.List[typing.Dict[str, typing.List[bytes]]]


@perde.attr(rename_all="camelCase", deny_unknown_fields=True)
@dataclass
class SpotLight:
    pillow1: typing.Optional[typing.Optional[typing.List[
        typing.List[int]]]] = field(metadata={
            "perde_rename": "bed",
            "perde_skip_deserializing": True
        })
    monster1: bytes


@perde.attr(rename_all="PascalCase")
@dataclass
class Sandpaper:
    room: typing.Optional[typing.Tuple[typing.Tuple[int, typing.Dict[
        str, bool]]]] = field(metadata={
            "perde_rename": "comet",
            "perde_skip_deserializing": True
        })


@perde.attr(rename_all="SCREAMING_SNAKE_CASE", rename="Carrier1", default=True)
@dataclass
class Tongue:
    pass


@perde.attr(rename_all="kebab-case")
@dataclass
class Foot1:
    finger1: typing.List[bool] = field(metadata={
        "perde_rename": "dress1",
        "perde_skip_deserializing": True
    })


@perde.attr(rename_all="camelCase", default=True)
@dataclass
class Bank:
    sunglasses3: typing.Optional[bool] = field(
        metadata={"perde_skip_deserializing": True})


@dataclass
class Potato2:
    pass


@perde.attr(rename_all="PascalCase", deny_unknown_fields=True, default=True)
@dataclass
class Sandwich1:
    software1: bytes = field(metadata={"perde_skip": True})
    school1: bool


@perde.attr(rename_all="SCREAMING_SNAKE_CASE", deny_unknown_fields=True)
@dataclass
class Stomach:
    explosive2: typing.List[str] = field(metadata={"perde_skip": True})
    carrot1: float


@perde.attr(rename_all="PascalCase", rename="IceCream")
@dataclass
class Chair2:
    brain: typing.Optional[Tongue] = field(metadata={"perde_skip": True})
    gas: typing.Union[bytes, typing.Set[int], Foot1, typing.Union[
        str, Bank, typing.Union[
            typing.Dict[str, str], typing.List[typing.Optional[bool]],
            typing.Set[typing.Union[int, Potato2, Sandwich1,
                                    typing.Optional[str]]],
            typing.Union[typing.Dict[str, int],
                         typing.Tuple[typing.Dict[str, typing.Dict[str,
                                                                   float]]],
                         Stomach]]]] = field(
                             metadata={"perde_rename": "kaleidoscope"})


@perde.attr(rename_all="SCREAMING-KEBAB-CASE", rename="Hammer2")
@dataclass
class Shower:
    pass


@perde.attr(rename_all="SCREAMING_SNAKE_CASE",
            rename="Pants",
            deny_unknown_fields=True,
            default=True)
@dataclass
class Aircraft1:
    crystal: Shower


@perde.attr(rename_all="UPPERCASE", deny_unknown_fields=True, default=True)
@dataclass
class Meat2:
    signature1: Aircraft1 = field(metadata={"perde_rename": "skeleton"})


@perde.attr(rename_all="SCREAMING-KEBAB-CASE", deny_unknown_fields=True)
@dataclass
class Rifle:
    foot: int = field(metadata={"perde_skip_deserializing": True})
    pyramid: typing.Optional[typing.Optional[bytes]] = field(
        metadata={"perde_rename": "pocket"})


@perde.attr(rename_all="SCREAMING-KEBAB-CASE")
@dataclass
class Shop1:
    teeth: typing.Optional[bool] = field(metadata={"perde_rename": "fruit"})


@dataclass
class A:
    a: bool
    b: int


TYPES = [
    typing.Union[float, bytes, Fire], typing.Tuple[bytes], typing.List[bool],
    Boss, typing.Optional[typing.List[typing.Tuple[()]]],
    typing.Dict[str,
                typing.Set[bool]], int, typing.List[typing.Optional[bool]],
    Navy, typing.Optional[typing.Union[bool, str, typing.Set[bytes],
                                       Television]],
    typing.Union[bytes, typing.Dict[str, typing.Union[
        float, str, Videotape,
        typing.Union[str, typing.Dict[str, int],
                     typing.Union[int, float, typing.List[bool], Dung]]]],
                 typing.Set[typing.Union[bool, str,
                                         Pillow]], typing.Tuple[Planet, int]],
    typing.Union[bool, typing.List[str],
                 typing.Union[Fork, typing.Union[bool, float, typing.Set[str],
                                                 typing.Tuple[float]]]], bytes,
    bytes, typing.List[int], str, str, Rocket, typing.Optional[Bible], bool,
    float, typing.Tuple[()], bool, typing.Optional[bool],
    typing.Union[int, typing.List[float], Vulture], int, bool,
    typing.List[typing.Union[float, typing.Set[bool],
                             typing.Tuple[typing.Tuple[typing.Optional[int],
                                                       bytes], Passport],
                             typing.Union[int, str, typing.Dict[str, int],
                                          typing.Set[int]]]],
    typing.Optional[bool], typing.Union[bool, int, str, bytes], Drill, Girl,
    float, typing.List[typing.Tuple[Sandwich, typing.Set[bytes]]],
    typing.Union[int, float,
                 Butterfly], int, str, Fruit, typing.Union[bool, str, Guitar],
    typing.Tuple[typing.List[typing.Optional[int]], typing.Dict[str, bool]],
    bool, float, str, Map, bool, Table1, str, typing.Set[bytes],
    typing.Union[bool, int, typing.Tuple[str, bytes],
                 Tunnel], Map1, typing.Tuple[Roof, typing.Tuple[bytes]],
    typing.Union[bool, bytes, typing.Dict[str, bytes],
                 typing.Set[int]], int, typing.Dict[str,
                                                    Room], bool, bool, bool,
    float, int, typing.Optional[float], float, typing.Optional[Explosive],
    float, typing.Set[bytes], int, typing.Tuple[typing.Tuple[float],
                                                typing.Set[bool]], int,
    typing.List[typing.Union[typing.Set[str], typing.Tuple[typing.Dict[str,
                                                                       float],
                                                           Sex1], Solid1]],
    typing.Union[bytes, typing.Tuple[typing.Union[bool, str, bytes,
                                                  typing.Dict[str, bool]]],
                 Radar], str, typing.Dict[str, bool],
    typing.Union[float, Child1, Carrier, typing.Optional[Egg]], Highway1, str,
    typing.Union[str, typing.Set[str],
                 typing.Tuple[bool]], typing.Tuple[typing.Dict[str, bool],
                                                   bytes], Chocolates1, str,
    typing.Union[bool, str, typing.List[int],
                 typing.Optional[str]], Chair1, typing.List[bytes], Eyes,
    typing.Optional[bytes], bytes, Potato1, typing.Union[float, str, Gas],
    typing.Optional[typing.Tuple[bool]], Passport2,
    typing.Tuple[Gloves1,
                 typing.Set[int]], SpotLight, int, typing.Union[int,
                                                                Sandpaper],
    bool, typing.Tuple[Chair2, typing.Tuple[typing.Dict[str, Meat2]]], bytes,
    typing.Dict[str, typing.Dict[str,
                                 Rifle]], bytes, typing.Union[typing.List[int],
                                                              typing.Tuple[()],
                                                              Shop1],
    typing.Dict[str, bytes], typing.Dict[str, typing.Dict[str, float]], A
]
