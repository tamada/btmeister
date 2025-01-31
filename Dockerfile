FROM rust:1-alpine3.20 AS builder

RUN apk --no-cache add musl-dev

WORKDIR /app

COPY Cargo.toml .
RUN    mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release

COPY src    /app/src
COPY assets /app/assets
RUN    touch src/main.rs \
    && cargo build --release \
    && strip target/release/btmeister -o btmeister

FROM gcr.io/distroless/static-debian12:nonroot
USER nonroot

ARG VERSION=0.7.4

LABEL org.opencontainers.image.authors="Haruaki Tamada <tamada@users.noreply.github.com>" \
    org.opencontainers.image.url="https://github.com/tamada/btmeister" \
    org.opencontainers.image.documentation="Detecting the build tools in use." \
    org.opencontainers.image.source="https://github.com/tamada/btmeister/blob/main/Dockerfile" \
    org.opencontainers.image.version="${VERSION}"

WORKDIR /app

ENV HOME=/app
ENV BTMEISTER_HOME=/opt/btmeister

COPY --from=builder /app/btmeister /opt/btmeister/btmeister

ENTRYPOINT [ "/opt/btmeister/btmeister" ]
