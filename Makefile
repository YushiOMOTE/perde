packages ?= perde perde-json perde-msgpack perde-yaml

pipenv_opt ?= $(if $(python_version),--python $(python_version),)

develop-targets = $(foreach p,$(packages), develop-$(p))
build-targets = $(foreach p,$(packages), build-$(p))
publish-targets = $(foreach p,$(packages), publish-$(p))
test-publish-targets = $(foreach p,$(packages), test-publish-$(p))

test_pypi ?= https://test.pypi.org/legacy/

pipenv ?= pipenv run
maturin ?= $(pipenv) maturin
pytest ?= $(pipenv) pytest

bench-result-dir ?= assets
bench-results = json yaml msgpack

.PHONY: setup install-deps install-perde prepare-test
.PHONY: test bench develop build publish test-publish manifest test-manifest
.PHONY: $(develop-targets) $(build-targets) $(publish-targets) $(test-publish-targets)

setup: install-deps install-perde

install-deps:
	pipenv install --dev --skip-lock $(pipenv_opt)

install-perde: develop

prepare-test:
	make -C perde-tests/gen

test: prepare-test
	$(pytest) --benchmark-skip $(test_opt)

bench: prepare-test
	$(pytest) --benchmark-only $(test_opt)

bench-images: $(bench-results)

$(bench-results):
	$(pytest) --benchmark-only --benchmark-histogram $(bench-result-dir)/$@ -m $@

develop: $(develop-targets)

build: $(build-targets)

publish: $(publish-targets)

test-publish: test-manifest $(test-publish-targets) manifest

manifests:
	cd manifest-gen; cargo run -- -T templates manifests.yml ..

test-manifest:
	cd manifest-gen; cargo run -- -t -T templates manifests.yml ..

$(develop-targets):
	cd $(patsubst develop-%,%,$@); $(maturin) develop --release

$(build-targets):
	cd $(patsubst build-%,%,$@); $(maturin) build --release

$(publish-targets):
	cd $(patsubst publish-%,%,$@); $(maturin) publish \
		-u $(PYPI_USER) -p $(PYPI_PASSWORD)

$(test-publish-targets):
	cd $(patsubst test-publish-%,%,$@); $(maturin) publish \
		-u $(TEST_PYPI_USER) -p $(TEST_PYPI_PASSWORD) -r $(test_pypi)
