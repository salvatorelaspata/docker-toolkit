# Sample Node.js application

This repository is a sample Node.js application for Docker's documentation.

## Build and run

```bash
docker compose up --build -d
```

## Kill and remove

```bash
docker compose rm
```

## Run tests when developing locally

```bash
docker compose run server npm run test
# new image
docker build -t node-docker-image-test --progress=plain --no-cache --target test .
```
