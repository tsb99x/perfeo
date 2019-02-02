# FROM jordi/ab
#
# WORKDIR /usr/src/http_serv_perf
# COPY . .
#
# ENTRYPOINT ["ab", "-k", "-l", "-c", "32", "-n", "4096", "-T", "application/json", "-p", "post.json", "http://api:8080/"]

FROM williamyeh/wrk

WORKDIR /usr/src/http_serv_perf
COPY . .

CMD [ "-t32", "-c256", "-d10s", "-s", "post_test.lua", "--latency", "http://api:8080/" ]
#CMD ["-t8", "-c16384", "-d1m", "-s", "post_test.lua", "--latency", "--timeout", "10s", "http://api:8080/"]
#CMD ["-t8", "-c16384", "-d1m", "-s", "post_test.lua", "--latency", "--timeout", "5s", "http://api:8080/"]
