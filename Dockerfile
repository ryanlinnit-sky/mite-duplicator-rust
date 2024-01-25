FROM --platform=amd64 rust:latest as builder
WORKDIR /usr/src/duplicator
COPY . .
RUN cargo install --path .

FROM --platform=amd64 debian:bookworm-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/mite-duplicator-rust /usr/local/bin/mite-duplicator-rust

ARG USERID="9000"
ARG USERGROUPID="9000"
ARG USER_NAME="miteuser"

RUN addgroup --gid ${USERGROUPID} ${USER_NAME}
RUN adduser --uid ${USERID} --gid ${USERGROUPID} ${USER_NAME}

USER ${USERID}

CMD ["mite-duplicator-rust"]

#    "--message-socket=tcp://0.0.0.0:14301",
#    "tcp://0.0.0.0:14306",
#    "tcp://0.0.0.0:14307",
#    "tcp://0.0.0.0:14308"