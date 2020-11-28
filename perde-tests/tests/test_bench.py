import pytest
from typing import Dict, List
from util import BENCH_FORMATS, BENCH_FORMATS_EXCEPT, idfn

"""rust
add_value("BenchNumber", 1311);
add_value("BenchString", "aiueo");
"""


@pytest.mark.benchmark(group="pack-number")
@pytest.mark.parametrize("m", BENCH_FORMATS_EXCEPT("toml"), ids=idfn)
def test_bench_pack_number(m, benchmark):
    m.pack_bench(benchmark, 1311)


@pytest.mark.benchmark(group="unpack-number")
@pytest.mark.parametrize("m", BENCH_FORMATS_EXCEPT("toml"), ids=idfn)
def test_bench_unpack_number(m, benchmark):
    m.unpack_bench(benchmark, m.data("BenchNumber"), int)


@pytest.mark.benchmark(group="pack-string")
@pytest.mark.parametrize("m", BENCH_FORMATS_EXCEPT("toml"), ids=idfn)
def test_bench_pack_string(m, benchmark):
    m.pack_bench(benchmark, "aiueo")


@pytest.mark.benchmark(group="unpack-string")
@pytest.mark.parametrize("m", BENCH_FORMATS_EXCEPT("toml"), ids=idfn)
def test_bench_unpack_string(m, benchmark):
    m.unpack_bench(benchmark, m.data("BenchString"), str)


"""rust
add_value("BenchDict", {
    let mut map = IndexMap::new();
    map.insert("10".to_string(), 10000);
    map.insert("101".into(), 100030);
    map.insert("102".into(), 102000);
    map
});
"""


@pytest.mark.benchmark(group="pack-dict")
@pytest.mark.parametrize("m", BENCH_FORMATS, ids=idfn)
def test_bench_pack_dict(m, benchmark):
    m.pack_bench(benchmark, {"10": 10000, "101": 10030, "102": 102000})


@pytest.mark.benchmark(group="unpack-dict")
@pytest.mark.parametrize("m", BENCH_FORMATS, ids=idfn)
def test_bench_unpack_dict(m, benchmark):
    m.unpack_bench(benchmark, m.data("BenchDict"), Dict[str, int])


"""rust
add_value("BenchList", vec![1i64, 2, -3, 4, 5, -8]);
"""


@pytest.mark.benchmark(group="pack-list")
@pytest.mark.parametrize("m", BENCH_FORMATS_EXCEPT("toml"), ids=idfn)
def test_bench_pack_list(m, benchmark):
    m.pack_bench(benchmark, [1, 2, -3, 4, 5, -8])


@pytest.mark.benchmark(group="unpack-list")
@pytest.mark.parametrize("m", BENCH_FORMATS_EXCEPT("toml"), ids=idfn)
def test_bench_unpack_list(m, benchmark):
    m.unpack_bench(benchmark, m.data("BenchList"), List[int])
