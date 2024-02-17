# syntax=docker/dockerfile:1
FROM rust:1.72-alpine as builder
RUN USER=root cargo new --bin scoreboard
WORKDIR ./scoreboard
RUN apk update && apk add --no-cache musl-dev && rm -rf /var/cache/apk/*
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/deps/scoreboard*
ADD . ./
RUN cargo build --release

FROM alpine:latest
RUN apk update && apk add --no-cache ca-certificates tzdata && rm -rf /var/cache/apk/*
ENV TZ=Etc/UTC
EXPOSE 8000
RUN adduser --disabled-password appuser && mkdir -p /app
COPY --from=builder /scoreboard/db /app/db
COPY --from=builder /scoreboard/templates /app/templates
COPY --from=builder /scoreboard/Rocket.toml /app
COPY --from=builder /scoreboard/target/release/scoreboard /app/scoreboard
RUN chown -R appuser:appuser /app
USER appuser
WORKDIR /app
CMD ["./scoreboard"]
