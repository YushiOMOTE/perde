.PHONY: all codegen datagen

default: codegen datagen

codegen:
	cargo run --bin codegen > datagen/src/main.rs

datagen:
	cargo run --bin datagen
