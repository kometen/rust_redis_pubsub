Taken from https://github.com/paoloposso/rust_redis_pubsub/blob/main/Cargo.toml.

```
$ docker build -t web_service:latest .
$ docker run -p3000:3000 --add-host=host.docker.internal:host-gateway web_service:latest
```
