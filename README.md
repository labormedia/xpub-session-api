# xpub-session-api



## Run

Initialize storage:

```console
docker run -p 27017:27017 --name mongodb -d mongodb/mongodb-community-server:latest
docker run -p 6379:6379 --name actix-redis -d redis
```

```console
cargo run --release
```