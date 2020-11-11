.PHONY: all codegen datagen

run = pipenv run cargo run --bin
run-codegen = $(run) codegen --
run-datagen = $(run) datagen --

default: codegen datagen

schemagen:
	$(run-codegen) schema -d 5 -n 100 schema.yml

presetgen:
	$(run-codegen) schema -p rename rename.yml

codegen: schema.yml rename.yml
	$(run-codegen) code datagen/src/defs.rs tests/defs.py $^

datagen:
	$(run-datagen) -f json
