from .perde import resolve  # type: ignore
from .perde import json, toml, yaml, msgpack  # noqa: F401
import sys
import enum


# Workaround in https://github.com/PyO3/pyo3/issues/759
sys.modules["perde.json"] = json
sys.modules["perde.msgpack"] = msgpack
sys.modules["perde.yaml"] = yaml
sys.modules["perde.toml"] = toml


def attr(*args, **kwargs):
    if args:
        return resolve(args[0])

    def func(ty):
        resolve(ty, **kwargs)
        return ty

    return func


class Enum(enum.Enum):
    def __new__(cls, p, *args, **kwargs):
        if isinstance(p, enum.auto):
            raise RuntimeError("enum.auto() is not supported at the moment")
        e = object().__new__(cls)
        e._value_ = p
        if args:
            e._perde_metadata = args[0]
        return e


class IntEnum(enum.IntEnum):
    def __new__(cls, p, *args, **kwargs):
        if isinstance(p, enum.auto):
            raise RuntimeError("enum.auto() is not supported at the moment")
        e = int.__new__(cls, p)
        e._value_ = p
        if args:
            e._perde_metadata = args[0]
        return e
