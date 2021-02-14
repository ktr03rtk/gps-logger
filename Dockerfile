FROM rust:1.50 as builder
WORKDIR /usr/src/gps-logger
COPY . .
RUN cargo install --path .

FROM debian:10-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/gps-logger /usr/local/bin/gps-logger
CMD ["gps-logger"]
