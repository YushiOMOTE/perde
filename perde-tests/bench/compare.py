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
import ujson
import orjson
import toml
import yaml


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
    with_cls: bool
    library: str
    format: str
    enc: Optional[Callable[[Any], Any]]
    dec: Optional[Callable]
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
            if self.with_cls:
                obj2 = dec(cls, data)
            else:
                obj2 = dec(data)
            assert obj == obj2

        tags = [self.library, self.format, name]

        if enc:

            def ef():
                enc(obj)

        if dec:
            if self.with_cls:

                def df():
                    dec(cls, data)

            else:

                def df():
                    dec(data)

        er = enc and run(*tags, "encode", ef)
        dr = dec and run(*tags, "decode", df)

        return er, dr


def sentry(*args):
    return Entry(True, *args)


def dentry(*args):
    return Entry(False, *args)


def run(library, format, data, dir, f, *args, **kwargs):
    r = timeit.repeat(f, *args, number=10000, **kwargs)
    r = Report(library, format, data, dir, r)
    print(r)
    return r


def normalize(reports: List[Report]):
    reports = [r for r in reports if r is not None]
    base = {r.cat: r.mean for r in reports if r.library == "perde"}
    for r in reports:
        r.norm = r.mean / base[r.cat]
    return reports


def make_barchart(filename: str, title: str, reports: List[Report], fmt: str):
    reports = [r for r in reports if r.format == fmt]

    print(f"-------- {filename} --------")
    for r in reports:
        print(r)

    c = pygal.Bar(show_legend=False)
    c.title = title

    header = [r.library for r in reports]
    header = list(set(header))

    def sorter(v):
        if v == "perde":
            return ""
        return v

    header.sort(key=sorter)

    def find_norm(reports, h, f):
        for r in reports:
            if r.library == h and r.format == f:
                return r.norm
        return None

    c.x_labels = header
    c.y_title = "normalized elapsed time"
    c.add(fmt, [find_norm(reports, h, fmt) for h in header])
    c.render_to_file(f"{filename}.svg")


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
DATA_C = {{"a": 10, "b": "Foo", "c": 3.3, "d": True}}
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


# Benchmark entries for struct (de)serialization
struct_entries = [
    sentry("perde", "json", perde.json.dumps, perde.json.loads_as, no_setup),
    sentry("perde", "msgpack", perde.msgpack.dumps, perde.msgpack.loads_as, no_setup),
    sentry("pyserde", "json", to_json, from_json, pyserde_setup),
    sentry("pyserde", "msgpack", to_msgpack, from_msgpack, pyserde_setup),
    sentry("attrs", "json", attr_json_dumps, None, attr_setup),
    sentry("attrs", "msgpack", attr_msgpack_dumps, None, attr_setup),
    sentry("cattrs", "json", cattr_json_dumps, cattr_json_loads, attr_setup),
    sentry("cattrs", "msgpack", cattr_msgpack_dumps, cattr_msgpack_loads, attr_setup),
    sentry(
        "mashumaro",
        "json",
        mashumaro_json_dumps,
        mashumaro_json_loads,
        mashumaro_json_setup,
    ),
    sentry(
        "mashumaro",
        "msgpack",
        mashumaro_msgpack_dumps,
        mashumaro_msgpack_loads,
        mashumaro_msgpack_setup,
    ),
]

# Benchmark entries for dict (de)serialization
dict_entries = [
    dentry("perde", "json", perde.json.dumps, perde.json.loads, no_setup),
    dentry("perde", "msgpack", perde.msgpack.dumps, perde.msgpack.loads, no_setup),
    dentry("perde", "toml", perde.toml.dumps, perde.toml.loads, no_setup),
    dentry("perde", "yaml", perde.yaml.dumps, perde.yaml.loads, no_setup),
    dentry("json", "json", json.dumps, json.loads, no_setup),
    dentry("orjson", "json", orjson.dumps, orjson.loads, no_setup),
    dentry("ujson", "json", ujson.dumps, ujson.loads, no_setup),
    dentry("toml", "toml", toml.dumps, toml.loads, no_setup),
    dentry("yaml", "yaml", yaml.dump, yaml.safe_load, no_setup),
    dentry("msgpack", "msgpack", msgpack.dumps, msgpack.loads, no_setup),
]


# Run tests
def run_benchmark(entries: List[Entry], data: str, fmt: str):
    reports = [e.run(data, models) for e in entries]
    enc_reports = [e for e, _ in reports]
    dec_reports = [d for _, d in reports]

    enc_reports = normalize(enc_reports)
    dec_reports = normalize(dec_reports)

    make_barchart(
        f"serialize_{fmt}_{data.lower()}", f"{fmt} serialization", enc_reports, fmt
    )
    make_barchart(
        f"deserialize_{fmt}_{data.lower()}", f"{fmt} deserialization", dec_reports, fmt
    )


run_benchmark(struct_entries, "DATA_A", "json")
run_benchmark(struct_entries, "DATA_A", "msgpack")
run_benchmark(dict_entries, "DATA_C", "json")
run_benchmark(dict_entries, "DATA_C", "toml")
run_benchmark(dict_entries, "DATA_C", "yaml")
run_benchmark(dict_entries, "DATA_C", "msgpack")
