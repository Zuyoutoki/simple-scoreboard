# syntax=docker/dockerfile:1
FROM rust:1.72-alpine as builder

RUN USER=root cargo new --bin scoreboard
WORKDIR ./scoreboard
RUN apk update \
    && apk add --no-cache musl-dev \
    && rm -rf /var/cache/apk/*
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/scoreboard*
RUN cargo build --release

# --------------------------------------------------

FROM alpine:latest
ARG APP=/app

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*
ENV TZ=Etc/UTC \
    APP_USER=appuser

EXPOSE 8000

RUN adduser --disabled-password $APP_USER \
    && mkdir -p $APP

COPY --from=builder /scoreboard/target/release/scoreboard $APP/scoreboard
COPY --from=builder /scoreboard/static $APP/static/
COPY --from=builder /scoreboard/migrations $APP/migrations/
COPY --from=builder /scoreboard/templates $APP/templates/
RUN chown -R $APP_USER:$APP_USER $APP

USER $APP_USER
WORKDIR $APP

CMD ["./scoreboard"]
