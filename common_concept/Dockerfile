FROM rust:1.60.0-slim-buster as builder
WORKDIR /app
COPY . .
ARG DEBIAN_FRONTEND=noninteractive
RUN rustup component add rustfmt

RUN cargo build --release

FROM ubuntu:20.04

WORKDIR /app

# Copy bin & config files
COPY --from=builder /app/target/release .

CMD ["./common_concept"]