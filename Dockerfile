FROM rust:latest as builder
WORKDIR /usr/src/duplicator
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/mite-duplicator-rust /usr/local/bin/mite-duplicator-rust

CMD ["mite-duplicator-rust"]

#    "--message-socket=tcp://0.0.0.0:14301",
#    "tcp://0.0.0.0:14306",
#    "tcp://0.0.0.0:14307",
#    "tcp://0.0.0.0:14308"