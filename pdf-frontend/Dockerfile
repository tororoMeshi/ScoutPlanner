FROM rust:latest AS builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/scoutplanner-frontend /usr/local/bin/scoutplanner-frontend
EXPOSE 8080
CMD ["/usr/local/bin/scoutplanner-frontend"]
