```
$ docker build -t user_service:latest .
$ docker run -p3001:3001 --add-host=host.docker.internal:host-gateway user_service:latest
```
