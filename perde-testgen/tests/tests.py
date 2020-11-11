import defs
import perde_json

def test_json_with_serde():
    for i, t in enumerate(TYPES):
        with open(f'json/{i}') as f:
            s = f.read()
            print(f'repacking {s} as {t}')
            p = perde_json.loads_as(t, s)
            print(f'{p}')
            a = perde_json.dumps(p)
            assert a == s
