# Perfeo

Project to store sample implementations of HTTP web servers.

## API specification

Each server should fully implement following specification:

- Healthcheck PING/PONG route.
    `GET localhost:8080/ping -> 200 PONG`
- Database (MongoDB) insert route.
    `POST {"sample":"json"} localhost:8080/ -> 200 OK`

## Packaging

Source code must be built and run in container.
Docker is selected format of containerization.
Healthchecks implemented in each of container.

## Benchmarking

Benchmarking is done with [wrk](https://github.com/wg/wrk) utility.
Utility launched after DB and API to ensure successful launch.
Standard settings are used and can be found in `wrk/Dockerfile`.
