pipenv-opt ?= $(if $(python-version),--python $(python-version),)

pipenv ?= pipenv run
maturin ?= $(pipenv) maturin
pytest ?= $(pipenv) pytest
doctest ?= $(pipenv) python -m doctest
twine ?= $(pipenv) twine

bench-result-dir ?= assets

build-opt ?= --release
build-version-opt ?= $(if $(python-version),-i python$(python-version),)


.PHONY: setup install-deps install-perde prepare-test
.PHONY: lint pep8 mypy test doctest bench develop build coverage publish test-publish clean


default: setup lint test


setup: install-deps install-perde


install-deps:
	pipenv install --dev --skip-lock $(pipenv-opt)
	cargo install grcov


install-perde: develop


prepare-test:
	make -C perde-tests/gen


lint: pep8 mypy


test: doctest prepare-test
	$(pytest) --benchmark-skip $(test-opt)


bench: prepare-test
	$(pytest) --benchmark-only $(test-opt)


histogram:
	$(foreach fmt,\
		json yaml msgpack toml,\
		$(pytest) --benchmark-only --benchmark-histogram $(bench-result-dir)/$(fmt) --benchmark-json=$(bench-result-dir)/json -m $(fmt);)


develop:
	cd perde; $(maturin) develop $(build-opt)


build:
	cd perde; $(maturin) build $(build-opt) $(build-version-opt)


publish: clean build
	cd perde; $(twine) upload -u $(PYPI_USER) -p $(PYPI_PASSWORD) target/wheels/*


test-publish: clean build
	cd perde; $(twine) upload -u $(TEST_PYPI_USER) -p $(TEST_PYPI_PASSWORD) -r testpypi target/wheels/*


clean:
	cd perde; cargo clean


pep8:
	$(pipenv) flake8


mypy:
	$(pipenv) mypy perde


doctest:
	find docs/src -name "*.md" | xargs $(doctest)
	$(doctest) README.md


coverage:
	grcov -s perde-core -t lcov --llvm --branch -o lcov.info ./perde/target/debug/
