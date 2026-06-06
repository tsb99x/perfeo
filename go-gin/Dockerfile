FROM docker.io/library/golang:1.24

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download
COPY . .
RUN go build -o perfeo .

EXPOSE 8080

CMD [ "./perfeo" ]

HEALTHCHECK --interval=5s --timeout=30s --retries=3 \
  CMD curl --fail http://localhost:8080/ping || exit 1
