FROM rust:1.47 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM ubuntu:18.04
RUN apt-get update && apt-get install -y ca-certificates tzdata && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/vote-handler /app/vote_handler
ENTRYPOINT ["./vote_handler"]