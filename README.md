# Perfeo

Directory of sample HTTP web server implementations.
Each implementation should be compliant with the general API specification.

## API specification

- Healthcheck PING/PONG route.

    `GET localhost:8080/ping -> 200 PONG`

- Database (MongoDB) insert route.

    `POST {"sample":"json"} localhost:8080/ -> 200 OK`

    `POST {"broken...json"} localhost:8080/ -> 400 BAD_REQUEST`

    `POST {"sample":"json"} localhost:8080/ -> 500 INTERNAL_SERVER_ERROR`

## Packaging

Source code is to be built and run in a container (Docker).
To start default benchmarking suite, use `docker-compose up wrk`.
To start only API part, use `docker-compose up api`.
Health checks implemented in each container as a convenience for chained launch.

## Benchmarking

Benchmarking is done by [wrk](https://github.com/wg/wrk) utility, wrapped in [williamyeh/wrk](https://hub.docker.com/r/williamyeh/wrk/) image.
Utility launched after DB and API by default.
Thread/connection settings can be found in `wrk/Dockerfile`.
