FROM rustlang/rust:nightly as BUILDER

WORKDIR /usr/src/rust-rocket
COPY . .
RUN RUSTFLAGS="-C target-cpu=native" cargo build --release

FROM debian:stretch

WORKDIR /usr/bin
COPY --from=BUILDER /usr/src/rust-rocket/target/release/rust-rocket .

CMD ["./rust-rocket"]
