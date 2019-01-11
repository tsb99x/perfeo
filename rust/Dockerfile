FROM rustlang/rust:nightly as BUILDER

WORKDIR /usr/src/rukan
COPY . .
RUN cargo build --release
RUN ls -la target/release

FROM debian:stretch

WORKDIR /usr/bin
COPY --from=BUILDER /usr/src/rukan/target/release/rukan .
RUN ls -la

CMD ["./rukan"]
