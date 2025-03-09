FROM rust:latest AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev ca-certificates && apt-get clean
COPY --from=builder /usr/src/app/target/release/pdf-backend /usr/local/bin/pdf-backend
EXPOSE 8081
CMD ["/usr/local/bin/pdf-backend"]
