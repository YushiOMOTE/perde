from .perde import *

def attr(**kwargs):
    def func(ty):
        resolve(ty, **kwargs)
        return ty
    return func
