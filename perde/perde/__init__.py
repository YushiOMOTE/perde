from .perde import *
from .perde import resolve
import enum

def attr(**kwargs):
    def func(ty):
        resolve(ty, **kwargs)
        return ty
    return func


class Enum(enum.Enum):
    def __new__(cls, p, *args, **kwargs):
        if isinstance(p, enum.auto):
            raise RuntimeError(f'enum.auto() is not supported at the moment')
        e = object().__new__(cls)
        e._value_ = p
        if args:
            e._perde_metadata = args[0]
        return e


class IntEnum(enum.IntEnum):
    def __new__(cls, p, *args, **kwargs):
        if isinstance(p, enum.auto):
            raise RuntimeError(f'enum.auto() is not supported at the moment')
        e = int.__new__(cls, p)
        e._value_ = p
        if args:
            e._perde_metadata = args[0]
        return e
