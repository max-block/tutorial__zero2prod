start_docker:
    scripts/init_db.sh

stop_docker:
    docker stop zero2prod_postgresql
    docker rm -v zero2prod_postgresql
