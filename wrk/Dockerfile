FROM williamyeh/wrk

WORKDIR /usr/src/http_serv_perf
COPY . .

CMD ["-t32", "-c32768", "-d60s", "-s", "post_test.lua", "http://api:8080/"]
