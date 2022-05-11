FROM rust:1.60.0 as builder

WORKDIR /app

COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder /app/target/release/btmeister /

ENTRYPOINT [ "/btmeister" ]
