pipenv = pipenv run
run = $(pipenv) cargo run -p


.PHONY: codegen datagen


default: datagen


codegen:
	$(run) codegen -- ../tests datagen/src/main.rs


datagen: codegen
	$(run) datagen -- ../data
