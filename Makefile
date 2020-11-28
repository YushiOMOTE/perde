pipenv-opt ?= $(if $(python-version),--python $(python-version),)

pipenv ?= pipenv run
maturin ?= $(pipenv) maturin
pytest ?= $(pipenv) pytest
doctest ?= $(pipenv) python -m doctest
twine ?= $(pipenv) twine

bench-result-dir ?= assets

build-opt ?= --release
build-version-opt ?= $(if $(python-version),-i python$(python-version),)


.PHONY: setup install-deps install-perde
.PHONY: lint pep8 mypy clippy test test-codegen test-datagen fmt fmt-rs fmt-py doctest bench develop build coverage publish test-publish clean


default: setup lint test


setup: install-deps install-perde


install-deps:
	pipenv install --dev --skip-lock $(pipenv-opt)


install-perde: develop


lint: pep8 mypy clippy fmt


test: doctest test-datagen
	$(pytest) --benchmark-skip $(test-opt)


bench: test-datagen
	$(pytest) --benchmark-only $(test-opt)


histogram:
	$(foreach fmt,\
		json yaml msgpack toml,\
		$(pytest) --benchmark-only --benchmark-histogram $(bench-result-dir)/$(fmt) --benchmark-json=$(bench-result-dir)/json -m $(fmt);)


develop:
	$(maturin) develop -m perde/Cargo.toml $(build-opt)


build:
	$(maturin) build -m perde/Cargo.toml $(build-opt) $(build-version-opt)


publish: clean build
	$(twine) upload -u $(PYPI_USER) -p $(PYPI_PASSWORD) target/wheels/*


test-publish: clean build
	$(twine) upload -u $(TEST_PYPI_USER) -p $(TEST_PYPI_PASSWORD) -r testpypi target/wheels/*


clean:
	cargo clean


pep8:
	$(pipenv) flake8


mypy:
	$(pipenv) mypy perde


clippy:
	cargo clippy -- -D warnings


fmt: fmt-rs fmt-py


fmt-rs:
	cargo fmt --all -- --check


fmt-py:
	$(pipenv) black --check .


doctest:
	$(pipenv) python docs/test.py


coverage:
	grcov -s . -t html --llvm --branch -o coverage ./target/debug


test-codegen:
	cargo run -p codegen -- perde-tests/tests perde-tests/gen/datagen/src/main.rs


test-datagen: test-codegen
	cargo run -p datagen -- perde-tests/data
