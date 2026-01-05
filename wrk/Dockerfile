FROM docker.io/williamyeh/wrk:4.0.2

WORKDIR /usr/src/perfeo
COPY . .

CMD [ "-t32", "-c256", "-d1m", "-s", "post_test.lua", "--latency", "http://api:8080/" ]
