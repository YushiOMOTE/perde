import perde
import enum
import timeit
from typing_inspect import get_origin, get_args
from dataclasses import dataclass, fields, is_dataclass, field
from typing import Dict, TypeVar, Union, List, Tuple

@dataclass
class C:
    key: int
    value: str

print(perde.json.loads_as(C, '{"key": 3, "value": "hha"}'))
# perde.json.loads_as(C, '{"key": 3, "value": "hha"}')

print('---------- de -----------')

# res_perde_as = timeit.repeat('json.loads_as(C, \'{"key": 300, "value": "hoge"}\')', setup = '''
# from perde import json
# from dataclasses import dataclass

# @dataclass
# class C:
#     key: int
#     value: str
# json.loads_as(C, \'{"key": 300, "value": "hoge"}\')
# ''', number = 100000)

res_perde_as = timeit.repeat('json.loads_as(C, \'{"key": 3, "value": "hoge"}\')', setup = '''
from perde import json
from dataclasses import dataclass

@dataclass
class C:
    key: int
    value: str
json.loads_as(C, \'{"key": 300, "value": "hoge"}\')
''', number = 100000)

data = '{"key": 300, "value": "hoge"}'
check = f'''
# assert json.loads(\'{data}\') == {{"key": 300, "value": "hoge"}}
'''

# data = '{"a": [1,2,3,4,5,6,7,8,9]}'
# check = f'''
# assert json.loads(\'{data}\') == {{"a": [1,2,3,4,5,6,7,8,9]}}
# '''

res_json = timeit.repeat(f'json.loads(\'{data}\')', setup = f"import json{check}", number = 100000)
res_ujson = timeit.repeat(f'json.loads(\'{data}\')', setup = f"import ujson as json{check}", number = 100000)
res_perde = timeit.repeat(f'json.loads(\'{data}\')', setup = f"from perde import json{check}", number = 100000)
res_orjson = timeit.repeat(f'json.loads(\'{data}\')', setup = f"import orjson as json{check}", number = 100000)

print(f'json      = {res_json}')
print(f'perde as  = {res_perde_as}')
print(f'perde     = {res_perde}')
print(f'ujson     = {res_ujson}')
print(f'orjson    = {res_orjson}')

# prep = '''
# from dataclasses import dataclass

# @dataclass
# class C:
#     key: int
#     value: str

# c = C(300, "hoge")
# cc = {"key": 300, "value": "hoge"}
# '''

# print('---------- ser -----------')

# res_json = timeit.repeat('json.dumps(cc)', setup = f"import json{prep}", number = 100000)
# res_ujson = timeit.repeat('ujson.dumps(cc)', setup = f"import ujson{prep}", number = 100000)
# res_perde = timeit.repeat('json.dumps(c)', setup = f"from perde import json{prep}", number = 100000)
# res_orjson = timeit.repeat('orjson.dumps(c)', setup = f"import orjson{prep}", number = 100000)

# print(f'json      = {res_json}')
# print(f'ujson     = {res_ujson}')
# print(f'perde     = {res_perde}')
# print(f'orjson    = {res_orjson}')
