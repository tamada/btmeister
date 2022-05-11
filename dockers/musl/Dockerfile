FROM ekidd/rust-musl-builder:latest AS builder

WORKDIR /opt/btmeister

COPY . .

RUN  cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder /opt/btmeister/target/x86_64-unknown-linux-musl/release/btmeister /usr/bin/btmeister

ENTRYPOINT [ "/usr/bin/btmeister" ]
