setup: install-deps install-perde

install-deps:
	pipenv install --dev --skip-lock

install-perde: develop

test:
	pipenv run pytest

develop:
	pipenv run scripts/for-each.sh develop

develop-json:
	pipenv run scripts/for-each.sh develop perde-json

build:
	pipenv run scripts/for-each.sh build

publish:
	pipenv run scripts/for-each.sh publish

test-publish:
	cd manifest-gen; cargo run -- -t -T templates manifests.yml ..
	pipenv run scripts/for-each.sh test-publish

manifests:
	cd manifest-gen; cargo run -- -T templates manifests.yml ..
