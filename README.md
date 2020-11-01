# perde: python-wrapped serde

[![Project Status: WIP – Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![PyPi](https://img.shields.io/pypi/v/perde.svg)](https://pypi.python.org/pypi/perde)
[![Supported python versions](https://img.shields.io/pypi/pyversions/perde.svg)](https://pypi.org/project/perde/)
[![Actions Status](https://github.com/YushiOMOTE/perde/workflows/Rust/badge.svg)](https://github.com/YushiOMOTE/perde/actions)

![](https://github.com/YushiOMOTE/perde/blob/master/assets/logo.png)

Python wrapper around [the powerful Rust serialization framework](https://github.com/serde-rs/serde).

* Serialization & deserialization of python data structures.
* Supports dataclasses and most generics.
* Supports various format. By design, `perde` can support as many format as `serde` can.
* Provide case conversion of field names, skipping serialization/deserialization, structure flattening.
* Strict type checking.
* Very fast.

### Install

```sh
pip install perde-json
pip install perde-yaml
pip install perde-msgpack
```

### Usage

```python
import perde_json
import perde_yaml
import perde_msgpack

@dataclass
class A:
    key: int
    value: str

# Serialize objects into json, yaml, msgpack
perde_json.dumps(A(300, "json"))
perde_yaml.dumps(A(300, "yaml"))
perde_msgpack.dumps(A(300, "msgpack"))

# Deserialize as dataclasses
perde_json.loads_as(A, '{"key": 300, "value": "hoge"}')
perde_yaml.loads_as(A, '''key: 300
value: hoge
''')
perde_msgpack.loads_as(A, b'\x82\xA3\x6B\x65\x79\xCD\x01\x2C\xA5\x76\x61\x6C\x75\x65\xCD\x01\x90')

# Deserialize as objects
perde_json.loads_as(A, '{"key": 300, "value": "hoge"}')
perde_yaml.loads_as(A, '''key: 300
value: hoge
''')
perde_msgpack.loads_as(A, b'\x82\xA3\x6B\x65\x79\xCD\x01\x2C\xA5\x76\x61\x6C\x75\x65\xCD\x01\x90')
```

### Supported formats

* [x] JSON
* [x] YAML
* [x] MessagePack
* [ ] CBOR
* [ ] Toml
* [ ] Pickle
* [ ] RON
* [ ] BSON
* [ ] Avro
* [ ] JSON5
* [ ] Postcard
* [ ] URL
* [ ] Environment variables
* [ ] AWS Parameter Store
* [ ] S-expressions
* [ ] D-Bus
* [ ] FlexBuffer
* [ ] XML

### Benchmark

```
json(de) -------------------
json      = [0.1892565581947565, 0.1789955347776413, 0.19771194644272327, 0.17869805544614792, 0.1817416027188301]
perde as  = [0.07256896048784256, 0.06387559697031975, 0.06289006397128105, 0.06492204032838345, 0.06444761715829372]
perde     = [0.03787072375416756, 0.03849206678569317, 0.03701256774365902, 0.03784210607409477, 0.03712223842740059]
ujson     = [0.03571947664022446, 0.0350071769207716, 0.035524480044841766, 0.03537473827600479, 0.03500896133482456]
orjson    = [0.024663090705871582, 0.026005828753113747, 0.025051748380064964, 0.0264505036175251, 0.024867044761776924]

yaml(de) -------------------
yaml      = [1.8657512124627829, 1.8705988600850105, 1.8599027246236801, 1.8804237693548203, 1.8527513016015291]
perde as  = [0.29090225137770176, 0.27482700906693935, 0.2708629425615072, 0.2854452319443226, 0.28280119970440865]
perde     = [0.22424191236495972, 0.2495588045567274, 0.22433684580028057, 0.22169128619134426, 0.22160297632217407]

msgpack(de) ----------------
msgpack   = [0.03487630747258663, 0.035033950582146645, 0.03426872752606869, 0.03444667346775532, 0.03443203307688236]
perde as  = [0.07079600915312767, 0.05985707975924015, 0.06260973773896694, 0.060033876448869705, 0.0608107578009367]
perde     = [0.03339817374944687, 0.033870622515678406, 0.033603109419345856, 0.034254319965839386, 0.034998660907149315]
```

```
json(ser) ------------------
json      = [0.2153916023671627, 0.20939842239022255, 0.2292985152453184, 0.20938796736299992, 0.20893244817852974]
ujson     = [0.04131609573960304, 0.04082906246185303, 0.04345548339188099, 0.040903979912400246, 0.04144351929426193]
perde     = [0.053302960470318794, 0.053485700860619545, 0.054095394909381866, 0.05770992115139961, 0.05336238816380501]
orjson    = [0.04534510709345341, 0.045184383168816566, 0.046133121475577354, 0.0456595029681921, 0.04615986533463001]

yaml(ser) ------------------
yaml      = [1.8657512124627829, 1.8705988600850105, 1.8599027246236801, 1.8804237693548203, 1.8527513016015291]
perde     = [0.01173756830394268, 0.011586908251047134, 0.011359155178070068, 0.011403439566493034, 0.013109922409057617]

msgpack(ser) ---------------
msgpack   = [0.03487630747258663, 0.035033950582146645, 0.03426872752606869, 0.03444667346775532, 0.03443203307688236]
perde     = [0.054882919415831566, 0.05104514956474304, 0.05093616619706154, 0.050708770751953125, 0.05338519997894764]
```

#### Benchmark note

* Deserialization
    * `perde`: Deserialize to `dict`. (non-dataclass)
    * `perde as`: Deserialize to `dataclass`.
    * Others: Deserialize to `dict`. (non-dataclass)
* Serialization
    * `perde`, `orjson`: Serialize `dataclasses`.
    * Others: Serialize `dict`.
