FROM --platform=$BUILDPLATFORM rust:1.60.0 as builder

ARG TARGETARCH
WORKDIR /app
COPY rust /app

RUN cargo build --release --target $BUILDPLATFORM

FROM gcr.io/distroless/cc

COPY --from=builder /app/target/release/btmeister /

ENTRYPOINT [ "/btmeister" ]
