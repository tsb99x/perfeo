FROM docker.io/library/maven

WORKDIR /usr/src/perfeo
COPY . .
RUN mvn package

EXPOSE 8080

CMD java                      \
    -server                    \
    -XX:+UseNUMA                \
    -XX:+UseParallelGC           \
    -Dvertx.disableMetrics=true   \
    -Dvertx.disableH2c=true        \
    -Dvertx.disableWebsockets=true  \
    -Dvertx.flashPolicyHandler=false \
    -Dvertx.threadChecks=false        \
    -Dvertx.disableContextTimings=true \
    -Dvertx.disableTCCL=true            \
    -jar                                 \
    target/perfeo-1.0.0-fat.jar           \
    --instances                            \
    `grep --count ^processor /proc/cpuinfo`

HEALTHCHECK --interval=5s --timeout=30s --retries=3 \
  CMD curl --fail http://localhost:8080/ping || exit 1
