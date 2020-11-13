.PHONY: codegen datagen

pipenv = pipenv run
run = $(pipenv) cargo run -p

default: datagen

codegen:
	$(run) codegen ../tests > datagen/src/main.rs

datagen: codegen
	$(run) datagen ../data
