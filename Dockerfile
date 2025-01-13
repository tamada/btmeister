FROM rust:1-alpine3.20 AS builder

RUN apk --no-cache add musl-dev

WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY src    /app/src
COPY assets /app/assets
RUN touch src/main.rs
RUN cargo build --release
RUN strip target/release/btmeister -o btmeister

FROM gcr.io/distroless/static-debian12:nonroot
USER nonroot

WORKDIR /app

ENV HOME=/app
ENV BTMEISTER_HOME=/opt/btmeister

COPY --from=builder /app/btmeister /opt/btmeister

ENTRYPOINT [ "/opt/btmeister" ]
