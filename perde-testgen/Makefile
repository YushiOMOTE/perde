.PHONY: gen schemagen presetgen codegen datagen test

pipenv = pipenv run
run = $(pipenv) cargo run --bin
run-codegen = $(run) codegen --
run-datagen = $(run) datagen --
run-test = $(pipenv) pytest

default: gen

gen: presetgen codegen datagen

schemagen:
	$(run-codegen) schema -d 5 -n 100 schema.yml

presetgen:
	$(run-codegen) schema -p rename rename.yml

codegen: rename.yml #schema.yml (don't use until stabilized)
	$(run-codegen) code datagen/src/defs.rs tests/defs.py $^

datagen:
	$(run-datagen) -f json -f yaml

test: gen
	$(run-test) -x tests/tests.py

