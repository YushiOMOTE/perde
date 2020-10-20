from .perde import *
from .perde import resolve

def attr(**kwargs):
    def func(ty):
        resolve(ty, **kwargs)
        return ty
    return func
