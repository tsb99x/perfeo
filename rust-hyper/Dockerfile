FROM rustlang/rust:nightly as BUILDER

WORKDIR /usr/src/rukan
COPY . .
RUN RUSTFLAGS="-C target-cpu=native" cargo build --release

FROM debian:stretch

WORKDIR /usr/bin
COPY --from=BUILDER /usr/src/rukan/target/release/rukan .

CMD ["./rukan"]
