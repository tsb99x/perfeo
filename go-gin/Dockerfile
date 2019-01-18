FROM golang

ENV GO111MODULE=on

WORKDIR /go/src/perfeo

COPY go.mod go.sum ./
RUN go get -d -v
COPY . .
RUN go install -v

EXPOSE 8080

CMD [ "/go/bin/perfeo" ]

HEALTHCHECK --interval=5s --timeout=30s --retries=3 \
  CMD curl --fail http://localhost:8080/ping || exit 1
