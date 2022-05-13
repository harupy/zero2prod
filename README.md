Commands


```
# commands for cargo
cargo run
cargo test
cargo check

# send requests
curl -v http://127.0.0.1:8000/health_check

# docker
docker build --tag zero2prod --file Dockerfile .
docker run -p 8000:8000 zero2prod
docker run -p 8000:8000 --add-host=host.docker.internal:host-gateway zero2prod
docker run -p 8000:8000 --add-host=host.docker.internal:host-gateway -e APP_ENVIRONMENT=local_docker zero2prod

docker images zero2prod
```
