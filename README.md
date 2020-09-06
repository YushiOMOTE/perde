# perde

Experimentally connecting Rust and Python for serialization/deserialization.

### Install

```sh
pip install perde
```

### Usage

```sh
from perde import json

@dataclass
class A:
    key: int
    value: str

# Serialize json as data class (member types are verified before writing)
json.dumps(A(300, "hoge"))

# Deserialize json as data class (with type checking)
json.loads_as(A, \'{"key": 300, "value": "hoge"}\')

# Deserialize json as objects (without type checking)
json.loads(\'{"key": 300, "value": "hoge"}\')
```

### Develop

1. Install [`maturin`](https://pypi.org/project/maturin/).

    ```sh
    pip install maturin
    ```

2. Create a virtualenv. (e.g. using [`pyenv`](https://github.com/pyenv/pyenv))

    ```sh
    pyenv virtualenv myenv
    pyenv activate myenv
    ```

3. Build and install `perde` locally with `maturin`.

    ```sh
    maturin develop --release
    ```

4. Do something.

    ```sh
    # Run tests
    pytest
    
    # Run benchmark
    python benches/compare.py
    ```

### Benchmark

```
---------- de -----------
json      = [0.26801170399999996, 0.2677857140000002, 0.26969751200000003, 0.26798411899999985, 0.26833023800000033]
perde as  = [0.211489977, 0.179649139, 0.18023794699999995, 0.1804793090000001, 0.186093984]
perde     = [0.13880599800000004, 0.14001113600000004, 0.13964565800000006, 0.14055392300000014, 0.14022343800000003]
ujson     = [0.05726369500000006, 0.05905630499999992, 0.057515596, 0.05737184300000031, 0.05719437999999988]
orjson    = [0.048624420000000335, 0.04795801799999966, 0.048007664999999644, 0.048000748000000204, 0.04796366700000032]
---------- ser -----------
json      = [0.3255122990000001, 0.31933131099999956, 0.31206383300000073, 0.3152893850000007, 0.4371285949999999]
ujson     = [0.06691166299999995, 0.06797659700000036, 0.0665169419999998, 0.06802924099999963, 0.06653239099999997]
perde     = [0.1784434509999997, 0.1777624530000006, 0.187134715, 0.1896725160000008, 0.19498700100000033]
orjson    = [0.046137584999999426, 0.04493453699999961, 0.04459659599999988, 0.04568931200000037, 0.04486202400000039]
```

* `json`: Built-in `json`.
* `ujson`: [ujson](https://github.com/ultrajson/ultrajson).
* `orjson`: [orjson](https://github.com/ijl/orjson) (serialize data class, deserialize as objects without type checking)
* `perde as`: By `perde`. (serialize data class, deserialize as data class with type checking)
* `perde`: By `perde`. (serialize/deserialize as objects; no type checking)
