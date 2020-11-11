.PHONY: all codegen datagen

default: codegen datagen

codegen:
	cargo run --bin codegen -- datagen/src/defs.rs tests/defs.py

datagen:
	cargo run --bin datagen -- .
