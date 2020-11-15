format-packages ?= perde-json perde-msgpack perde-yaml perde-toml
packages ?= perde $(format-packages)

pipenv_opt ?= $(if $(python_version),--python $(python_version),)

develop-targets = $(addprefix develop-,$(packages))
build-targets = $(addprefix build-,$(packages))
coverage-targets = $(addprefix coverage-,$(packages))
publish-targets = $(addprefix publish-,$(packages))
test-publish-targets = $(addprefix test-publish-,$(packages))

test_pypi ?= https://test.pypi.org/legacy/

pipenv ?= pipenv run
maturin ?= $(pipenv) maturin
pytest ?= $(pipenv) pytest

bench-result-dir ?= assets
bench-histograms = $(format-packages:perde-%=histogram-%)

build-opt ?= --release


.PHONY: setup install-deps install-perde prepare-test
.PHONY: lint pep8 mypy test bench develop build coverage publish test-publish manifest test-manifest
.PHONY: $(develop-targets) $(build-targets) $(coverage-targets) $(publish-targets) $(test-publish-targets)
.PHONY: bench-histogram $(bench-histograms)


default: setup lint test


setup: install-deps install-perde


install-deps:
	pipenv install --dev --skip-lock $(pipenv_opt)
	cargo install grcov


install-perde: develop


prepare-test:
	make -C perde-tests/gen


lint: pep8 mypy


test: prepare-test
	$(pytest) --benchmark-skip $(test_opt)


bench: prepare-test
	$(pytest) --benchmark-only $(test_opt)


bench-histogram: $(bench-histograms)


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
	cd $(@:build-%=%); $(maturin) build $(build-opt)


$(publish-targets):
	cd $(@:publish-%=%); $(maturin) publish \
		-u $(PYPI_USER) -p $(PYPI_PASSWORD)


$(test-publish-targets):
	cd $(@:test-publish-%=%); $(maturin) publish \
		-u $(TEST_PYPI_USER) -p $(TEST_PYPI_PASSWORD) -r $(test_pypi)


coverage:
	grcov -s perde-core -t lcov --llvm --branch -o lcov.info \
		./perde/target/debug/ \
		./perde-json/target/debug/ \
		./perde-yaml/target/debug/ \
		./perde-msgpack/target/debug/ \
		./perde-toml/target/debug/
