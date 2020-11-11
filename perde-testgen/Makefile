.PHONY: all codegen datagen

default: codegen datagen

schemagen:
	cargo run --bin codegen -- schema -d 5 -n 100 schema.yml

codegen:
	cargo run --bin codegen -- code schema.yml datagen/src/defs.rs tests/defs.py

datagen:
	cargo run --bin datagen -- -f json
