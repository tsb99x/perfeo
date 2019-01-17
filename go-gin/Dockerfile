FROM golang

WORKDIR /go/src/perfeo
COPY . .

ENV GO111MODULE=on
RUN go get -d -v ./...
RUN go install -v ./...

EXPOSE 8080

CMD [ "perfeo" ]

HEALTHCHECK --interval=5s --timeout=30s --retries=3 \
  CMD curl --fail http://localhost:8080/ping || exit 1
