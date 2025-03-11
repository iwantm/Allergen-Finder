# Build stage
FROM rust:1.85-slim as builder
RUN apt-get update && apt-get install -y curl libpq-dev build-essential musl-tools
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm
RUN apt-get update && apt-get install -y curl libpq-dev build-essential musl-tools
COPY --from=builder /usr/src/app/target/release/backend-rs /usr/local/bin/backend-rs
EXPOSE 8080
CMD ["backend-rs"]