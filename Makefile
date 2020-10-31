setup: install-deps install-perde

install-deps:
	pipenv install --dev --skip-lock

install-perde:
	pipenv run scripts/maturin.sh develop

test:
	pipenv run pytest -s

build:
	pipenv run scripts/maturin.sh build

publish:
	pipenv run scripts/maturin.sh publish
