# syntax=docker/dockerfile:1.3
FROM rust:1.72-bullseye as rust-build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /root

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install cargo-strip

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/root/target \
    cargo build --release && \
    cargo strip && \
    mv /root/target/release/flight-service /root

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install libpq5 -y && \
    apt-get install ca-certificates -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=rust-build /root/flight-service /
COPY --from=rust-build /root/.env /.env

WORKDIR /
ENV RUST_LOG=info
CMD ["./flight-service"]

EXPOSE 8060