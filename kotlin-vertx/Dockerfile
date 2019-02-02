FROM maven

WORKDIR /usr/src/kotlin-vertx
COPY . .
RUN mvn clean package
EXPOSE 8080
CMD java                                    \
    -server                                 \
    -XX:+UseNUMA                            \
    -XX:+UseParallelGC                      \
    -Dvertx.disableMetrics=true             \
    -Dvertx.disableH2c=true                 \
    -Dvertx.disableWebsockets=true          \
    -Dvertx.flashPolicyHandler=false        \
    -Dvertx.threadChecks=false              \
    -Dvertx.disableContextTimings=true      \
    -Dvertx.disableTCCL=true                \
    -jar                                    \
    target/perf-1.0.0-SNAPSHOT-fat.jar      \
    --instances                             \
    `grep --count ^processor /proc/cpuinfo` \
