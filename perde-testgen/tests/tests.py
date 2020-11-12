import defs
import perde_json, perde_yaml, perde_msgpack

def test_json_with_serde():
    for i, t in enumerate(defs.TYPES):
        with open(f'json/{i}') as f:
            s = f.read()
            print(f'repacking {s} as {t}')
            p = perde_json.loads_as(t, s)
            print(f'{p}')
            a = perde_json.dumps(p)
            assert a == s

def test_yaml_with_serde():
    for i, t in enumerate(defs.TYPES):
        with open(f'yaml/{i}') as f:
            s = f.read()
            print(f'repacking {s} as {t}')
            p = perde_yaml.loads_as(t, s)
            print(f'{p}')
            a = perde_yaml.dumps(p)
            assert a == s

def test_msgpack_with_serde():
    for i, t in enumerate(defs.TYPES):
        with open(f'msgpack/{i}', 'rb') as f:
            s = f.read()
            print(f'repacking {s} as {t}')
            p = perde_msgpack.loads_as(t, s)
            print(f'{p}')
            a = perde_msgpack.dumps(p)
            assert a == s
