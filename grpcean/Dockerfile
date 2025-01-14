FROM rust:1-bookworm AS builder

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends --assume-yes protobuf-compiler

WORKDIR /app

ADD . /app

RUN cargo build --release --target-dir bin

FROM debian:bookworm-slim

COPY --from=builder /app/bin/release/server .

EXPOSE 8080

CMD ["./server"]
