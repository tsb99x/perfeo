FROM rustlang/rust:nightly as BUILDER

WORKDIR /usr/src/rust-gotham
COPY . .
RUN RUSTFLAGS="-C target-cpu=native" cargo build --release

CMD ["./target/release/rust-gotham"]

#FROM debian:stretch
#
#WORKDIR /usr/bin
#COPY --from=BUILDER /usr/src/rust-gotham/target/release/rust-gotham .
#
#CMD ["./rust-gotham"]
