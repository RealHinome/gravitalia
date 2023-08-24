FROM rust:alpine3.18 AS builder

RUN USER=root cargo new --bin graphql
WORKDIR /graphql

ENV     RUSTFLAGS="-C target-feature=-crt-static"
RUN     apk add -q --update-cache --no-cache build-base openssl-dev musl

COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release

FROM alpine:3.18 AS runtime

RUN apk update \
 && apk add --no-cache libssl1.1 musl-dev libgcc tini curl

COPY --from=builder /graphql/target/release/graphql /bin/graphql

EXPOSE 8888/tcp
ENTRYPOINT ["tini", "--"]
CMD     /bin/graphql
