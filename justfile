dev:
    cargo watch -x run | bunyan

start_docker:
    scripts/init_db.sh

stop_docker:
    docker stop zero2prod_postgresql
    docker rm -v zero2prod_postgresql

build-docker:
    docker build --tag zero2prod --file Dockerfile .

sqlx-prepare:
    cargo sqlx prepare -- --lib

db-migrate:
    SKIP_DOCKER=true ./scripts/init_db.sh
