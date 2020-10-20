import perde
import enum
import timeit
from typing_inspect import get_origin, get_args
from dataclasses import dataclass, fields, is_dataclass, field
from typing import Dict, TypeVar, Union, List, Tuple

print('json(de) --------------')

res_perde_as = timeit.repeat('json.loads_as(C, \'{"key": 3, "value": "hoge"}\')', setup = '''
import perde_json as json
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

res_json = timeit.repeat(f'json.loads(\'{data}\')', setup = f"import json{check}", number = 100000)
res_ujson = timeit.repeat(f'json.loads(\'{data}\')', setup = f"import ujson as json{check}", number = 100000)
res_perde = timeit.repeat(f'json.loads(\'{data}\')', setup = f"import perde_json as json{check}", number = 100000)
res_orjson = timeit.repeat(f'json.loads(\'{data}\')', setup = f"import orjson as json{check}", number = 100000)

print(f'json      = {res_json}')
print(f'perde as  = {res_perde_as}')
print(f'perde     = {res_perde}')
print(f'ujson     = {res_ujson}')
print(f'orjson    = {res_orjson}')

print('yaml(de) ----------------')

res_perde_as = timeit.repeat('yaml.loads_as(C, \'{"key": 300, "value": "hoge"}\')', setup = '''
import perde_yaml as yaml
from dataclasses import dataclass

@dataclass
class C:
    key: int
    value: str
yaml.loads_as(C, \'{"key": 300, "value": "hoge"}\')
''', number = 100000)

# Why so slow...
res_yaml = timeit.repeat(f'yaml.load(\'{data}\')', setup = f"import yaml{check}", number = 10000)
res_perde = timeit.repeat(f'yaml.loads(\'{data}\')', setup = f"import perde_yaml as yaml{check}", number = 100000)

print(f'yaml      = {res_yaml}')
print(f'perde as  = {res_perde_as}')
print(f'perde     = {res_perde}')

print('msgpack(de) ----------------')
res_perde_as = timeit.repeat('msgpack.loads_as(C, b\'\\x82\\xA3\\x6B\\x65\\x79\\xCD\\x01\\x2C\\xA5\\x76\\x61\\x6C\\x75\\x65\\xCD\\x01\\x90\')', setup = '''
import perde_msgpack as msgpack
from dataclasses import dataclass

@dataclass
class C:
    key: int
    value: int
''', number = 100000)

res_msgpack = timeit.repeat('msgpack.loads(b\'\\x82\\xA3\\x6B\\x65\\x79\\xCD\\x01\\x2C\\xA5\\x76\\x61\\x6C\\x75\\x65\\xCD\\x01\\x90\')', setup = f"import msgpack", number = 100000)
res_perde = timeit.repeat('msgpack.loads(b\'\\x82\\xA3\\x6B\\x65\\x79\\xCD\\x01\\x2C\\xA5\\x76\\x61\\x6C\\x75\\x65\\xCD\\x01\\x90\')', setup = f"import perde_msgpack as msgpack", number = 100000)

print(f'msgpack   = {res_msgpack}')
print(f'perde as  = {res_perde_as}')
print(f'perde     = {res_perde}')

prep = '''
from dataclasses import dataclass

@dataclass
class C:
    key: int
    value: str

c = C(300, "hoge")
cc = {"key": 300, "value": "hoge"}
'''

print('---------- ser -----------')

res_json = timeit.repeat('json.dumps(cc)', setup = f"import json{prep}", number = 100000)
res_ujson = timeit.repeat('ujson.dumps(cc)', setup = f"import ujson{prep}", number = 100000)
res_perde = timeit.repeat('json.dumps(c)', setup = f"import perde_json as json{prep}", number = 100000)
res_orjson = timeit.repeat('orjson.dumps(c)', setup = f"import orjson{prep}", number = 100000)
res_orjson = timeit.repeat('yaml.dump(cc)', setup = f"import yaml{prep}", number = 10000)
res_orjson = timeit.repeat('msgpack.dumps(cc)', setup = f"import msgpack{prep}", number = 100000)

res_yaml_perde = timeit.repeat('yaml.dumps(c)', setup = f"import perde_yaml as yaml{prep}", number = 10000)
res_msgpack_perde = timeit.repeat('msgpack.dumps(c)', setup = f"import perde_msgpack as msgpack{prep}", number = 100000)


print(f'json      = {res_json}')
print(f'ujson     = {res_ujson}')
print(f'perde     = {res_perde}')
print(f'orjson    = {res_orjson}')

print(f'yaml      = {res_yaml}')
print(f'perde     = {res_yaml_perde}')

print(f'msgpack   = {res_msgpack}')
print(f'perde     = {res_msgpack_perde}')
