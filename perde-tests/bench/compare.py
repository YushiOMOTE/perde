from typing import Any, Callable, Optional
from dataclasses import dataclass
from serde.json import to_json, from_json
from serde.msgpack import to_msgpack, from_msgpack
import statistics
import perde
import timeit


import attr
import cattr
import msgpack
import json


@dataclass
class Report:
    name: str
    min: float
    max: float
    ave: float


@dataclass
class Entry:
    library: str
    format: str
    enc: Optional[Callable[[Any], Any]]
    dec: Optional[Callable[[Any, Any], Any]]
    setup: Callable[[Any, Any], Any] = lambda x, b: x

    def run(self, name, models):
        models = self.setup(models)
        # print(models)
        exec(models, globals())
        obj = eval(name)
        cls = type(obj)
        enc = self.enc
        dec = self.dec

        if enc and dec:
            data = enc(obj)
            assert obj == dec(cls, data)

        er = enc and run(
            f"{self.library}.{self.format}.{name}.encode", lambda: enc(obj)
        )
        dr = dec and run(
            f"{self.library}.{self.format}.{name}.decode", lambda: dec(cls, data)
        )

        return er, dr


def run(name, f, *args, **kwargs):
    r = timeit.repeat(f, *args, number=100000, **kwargs)
    return Report(name, min(r), max(r), statistics.mean(r))


models = """
import mashumaro
from serde import serialize, deserialize
from typing import List, Dict
from dataclasses import dataclass

{attr}
@dataclass
class A{base}:
    a: int
    b: str
    c: float
    d: bool

{attr}
@dataclass
class B{base}:
    a: List[A]
    b: Dict[str, int]


DATA_A = A(a=10, b="Foo", c=3.3, d=True)
DATA_B = B(a=[A(a=a, b=str(a), c=float(a), d=True) for a in range(10)],
           b={{"a": 100, "b": 200, "c": 300}})
"""

targets = ["DATA_A", "DATA_B"]


def no_setup(models):
    return models.format(attr="", base="")


def pyserde_setup(models):
    return models.format(attr="@serialize\n@deserialize", base="")


def attr_setup(cls):
    return models.format(attr="@attr.s(auto_attribs=True)", base="")


def attr_json_dumps(obj):
    return json.dumps(attr.asdict(obj))


def attr_msgpack_dumps(obj):
    return msgpack.dumps(attr.asdict(obj))


def cattr_json_dumps(obj):
    return json.dumps(cattr.unstructure(obj))


def cattr_json_loads(cls, data):
    return cattr.structure(json.loads(data), cls)


def cattr_msgpack_dumps(obj):
    return msgpack.dumps(cattr.unstructure(obj))


def cattr_msgpack_loads(cls, data):
    return cattr.structure(msgpack.loads(data), cls)


def mashumaro_json_setup(models):
    return models.format(attr="", base="(mashumaro.DataClassJSONMixin)")


def mashumaro_json_dumps(obj):
    return obj.to_json()


def mashumaro_json_loads(cls, data):
    return cls.from_json(data)


def mashumaro_msgpack_setup(models):
    return models.format(attr="", base="(mashumaro.DataClassMessagePackMixin)")


def mashumaro_msgpack_dumps(obj):
    return obj.to_msgpack()


def mashumaro_msgpack_loads(cls, data):
    return cls.from_msgpack(data)


entries = [
    Entry("pyserde", "json", to_json, from_json, pyserde_setup),
    Entry("pyserde", "msgpack", to_msgpack, from_msgpack, pyserde_setup),
    Entry("perde", "json", perde.json.dumps, perde.json.loads_as, no_setup),
    Entry("perde", "msgpack", perde.msgpack.dumps, perde.msgpack.loads_as, no_setup),
    Entry("attrs", "json", attr_json_dumps, None, attr_setup),
    Entry("attrs", "msgpack", attr_msgpack_dumps, None, attr_setup),
    Entry("cattrs", "json", cattr_json_dumps, cattr_json_loads, attr_setup),
    Entry("cattrs", "msgpack", cattr_msgpack_dumps, cattr_msgpack_loads, attr_setup),
    Entry(
        "mashumaro",
        "json",
        mashumaro_json_dumps,
        mashumaro_json_loads,
        mashumaro_json_setup,
    ),
    Entry(
        "mashumaro",
        "msgpack",
        mashumaro_msgpack_dumps,
        mashumaro_msgpack_loads,
        mashumaro_msgpack_setup,
    ),
]

for t in targets:
    for e in entries:
        print(e.run(t, models))
