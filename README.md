# rust_redis_pubsub
A simple pubsub-example using Rust and Redis.

To run issue

```
docker-compose up
```

Send a GET request to `localhost:3000/hello`. This will send a message to a
Redis-channel via the web_service-program.

When a message reaches the channel the user_service picks it up and send another message
to a differenct channel. This message is read by the web_service and returned as a response
to the GET-request.

It is not very stable so it stops responding after a few GET-request. Stop and restart when this
happens.
