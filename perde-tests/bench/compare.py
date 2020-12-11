from typing import Any, Callable, Optional, List
from dataclasses import dataclass
from serde.json import to_json, from_json
from serde.msgpack import to_msgpack, from_msgpack
import statistics
import timeit
import pygal
import perde
import attr
import cattr
import msgpack
import json


def key(lib: str, fmt: str, data: str, dir: str):
    return f"{lib}.{fmt}.{data}.{dir}"


def cat(fmt: str, data: str, dir: str):
    return f"{fmt}.{data}.{dir}"


@dataclass
class Report:
    library: str
    format: str
    data: str
    dir: str
    values: List[float]
    norm: float = None

    @property
    def name(self) -> str:
        return key(self.library, self.format, self.data, self.dir)

    @property
    def min(self) -> float:
        return min(self.values)

    @property
    def max(self) -> float:
        return max(self.values)

    @property
    def mean(self) -> float:
        return statistics.mean(self.values)

    @property
    def cat(self) -> str:
        return cat(self.format, self.data, self.dir)


@dataclass
class Entry:
    library: str
    format: str
    enc: Optional[Callable[[Any], Any]]
    dec: Optional[Callable[[Any, Any], Any]]
    setup: Callable[[Any, Any], Any] = lambda x, b: x

    def run(self, name, models):
        models = self.setup(models)
        exec(models, globals())
        obj = eval(name)
        cls = type(obj)
        enc = self.enc
        dec = self.dec

        if enc and dec:
            data = enc(obj)
            assert obj == dec(cls, data)

        tags = [self.library, self.format, name]

        er = enc and run(*tags, "encode", lambda: enc(obj))
        dr = dec and run(*tags, "decode", lambda: dec(cls, data))

        return er, dr


def run(library, format, data, dir, f, *args, **kwargs):
    r = timeit.repeat(f, *args, number=10000, **kwargs)
    r = Report(library, format, data, dir, r)
    print(r)
    return r


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

data = ["DATA_A", "DATA_B"]


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

# Run tests
reports = [e.run(d, models) for d in data for e in entries]
enc_reports = [e for e, _ in reports]
dec_reports = [d for _, d in reports]


def normalize(reports: List):
    reports = [r for r in reports if r is not None]
    base = {r.cat: r.mean for r in reports if r.library == "perde"}
    for r in reports:
        r.norm = r.mean / base[r.cat]
    return reports


enc_reports = normalize(enc_reports)
dec_reports = normalize(dec_reports)


def make_barchart(filename, title, reports, fmts):
    print(f"-------- {filename} --------")
    for r in reports:
        print(r)

    c = pygal.Bar()
    c.title = title

    header = [r.library for r in reports]
    header = list(set(header))

    def sorter(v):
        if v == "perde":
            return ""
        return v

    header.sort(key=sorter)

    def find(reports, h, f):
        return next(r for r in reports if r.library == h and r.format == f)

    c.x_labels = header
    for f in fmts:
        c.add(f, [find(reports, h, f).norm for h in header])
    c.render_to_file(f"{filename}.svg")


data_a_enc = [r for r in enc_reports if r.data == "DATA_A"]
data_a_dec = [r for r in dec_reports if r.data == "DATA_A"]

data_b_enc = [r for r in enc_reports if r.data == "DATA_B"]
data_b_dec = [r for r in dec_reports if r.data == "DATA_B"]

make_barchart("serialize_a", "serialization", data_a_enc, ["json", "msgpack"])
make_barchart("deserialize_a", "deserialization", data_a_dec, ["json", "msgpack"])

make_barchart("serialize_b", "serialization", data_b_enc, ["json", "msgpack"])
make_barchart("deserialize_b", "deserialization", data_b_dec, ["json", "msgpack"])
