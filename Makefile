docker-build:
	docker build --tag zero2prod --file Dockerfile .


docker-run:
	docker run -p 3000:3000 zero2prod


update:
	cargo fmt && cargo clippy