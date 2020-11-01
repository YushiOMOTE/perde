setup: install-deps install-perde

install-deps:
	pipenv install --dev --skip-lock

install-perde:
	pipenv run scripts/for-each.sh develop

test:
	pipenv run pytest -s

build:
	pipenv run scripts/for-each.sh build

publish:
	pipenv run scripts/for-each.sh publish

test-publish:
	pipenv run scripts/for-each.sh test-publish

manifests:
	cd manifest-gen; cargo run templates manifests.yml ..
