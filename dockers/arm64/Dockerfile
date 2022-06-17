FROM arm64v8/alpine:3.16 AS builder

WORKDIR /home/btmeister

COPY . .

RUN  apk add --no-cache gcc musl-dev rust cargo \
  && cargo build --release

FROM arm64v8/alpine:3.16

RUN  apk add --no-cache libgcc musl-dev \
  && adduser -D btmeister

COPY --from=builder /home/btmeister/target/release/btmeister /opt/btmeister/btmeister

WORKDIR /home/btmeister
USER btmeister

ENTRYPOINT [ "/opt/btmeister/btmeister" ]
