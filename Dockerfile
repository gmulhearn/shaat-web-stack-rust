FROM rust:1.71 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/web-service
COPY src src
COPY templates templates
COPY migrations migrations
COPY .sqlx .sqlx
COPY build.rs Cargo.toml Cargo.lock ./

RUN apt-get update; \
    apt-get install -y --no-install-recommends \
        libclang-dev

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian11

COPY --from=build /usr/local/cargo/bin/actix-html-templates /usr/local/bin/web-service
COPY templates templates
COPY migrations migrations
COPY static static

CMD ["web-service"]
