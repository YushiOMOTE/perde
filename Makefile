format-packages ?= perde-json perde-msgpack perde-yaml perde-toml
packages ?= perde $(format-packages)

pipenv-opt ?= $(if $(python-version),--python $(python-version),)

develop-targets = $(addprefix develop-,$(packages))
build-targets = $(addprefix build-,$(packages))
coverage-targets = $(addprefix coverage-,$(packages))
publish-targets = $(addprefix publish-,$(packages))
test-publish-targets = $(addprefix test-publish-,$(packages))

test-pypi ?= https://test.pypi.org/legacy/

pipenv ?= pipenv run
maturin ?= $(pipenv) maturin
pytest ?= $(pipenv) pytest

bench-result-dir ?= assets
bench-histograms = $(format-packages:perde-%=histogram-%)

build-opt ?= --release
build-version-opt ?= $(if $(python-version),-i python$(python-version),)


.PHONY: setup install-deps install-perde prepare-test
.PHONY: lint pep8 mypy test bench develop build coverage publish test-publish manifest test-manifest
.PHONY: $(develop-targets) $(build-targets) $(coverage-targets) $(publish-targets) $(test-publish-targets)
.PHONY: bench-histogram $(bench-histograms)


default: setup lint test


setup: install-deps install-perde


install-deps:
	pipenv install --dev --skip-lock $(pipenv-opt)
	cargo install grcov


install-perde: develop


prepare-test:
	make -C perde-tests/gen


lint: pep8 mypy


test: prepare-test
	$(pytest) --benchmark-skip $(test-opt)


bench: prepare-test
	$(pytest) --benchmark-only $(test-opt)


histogram: $(bench-histograms)


$(bench-histograms):
	$(pytest) --benchmark-only --benchmark-histogram $(bench-result-dir)/${@:histogram-%=%} -m ${@:histogram-%=%}


develop: $(develop-targets)


build: $(build-targets)


publish: $(publish-targets)


test-publish: test-manifest $(test-publish-targets) manifest


pep8:
	$(pipenv) flake8


mypy:
	$(pipenv) mypy perde


manifests:
	cd manifest-gen; cargo run -- -T templates manifests.yml ..


test-manifest:
	cd manifest-gen; cargo run -- -t -T templates manifests.yml ..


$(develop-targets):
	cd $(@:develop-%=%); $(maturin) develop $(build-opt)


$(build-targets):
	cd $(@:build-%=%); $(maturin) build $(build-opt) $(build-version-opt)


$(publish-targets):
	cd $(@:publish-%=%); $(maturin) publish \
		-u $(PYPI_USER) -p $(PYPI_PASSWORD) $(build-opt) $(build-version-opt)


$(test-publish-targets):
	cd $(@:test-publish-%=%); $(maturin) publish \
		-u $(TEST_PYPI_USER) -p $(TEST_PYPI_PASSWORD) -r $(test-pypi) $(build-opt) $(build-version-opt)


coverage:
	grcov -s perde-core -t lcov --llvm --branch -o lcov.info \
		./perde/target/debug/ \
		./perde-json/target/debug/ \
		./perde-yaml/target/debug/ \
		./perde-msgpack/target/debug/ \
		./perde-toml/target/debug/
