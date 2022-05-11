FROM alpine:3.15.4 AS builder

WORKDIR /home/btmeister

COPY . .

RUN  apk add --no-cache gcc musl-dev rust cargo \
  && cargo build --release

FROM alpine:3.15.4

RUN  apk add --no-cache libgcc musl-dev \
  && adduser -D btmeister

COPY --from=builder /home/btmeister/target/release/btmeister /opt/btmeister/btmeister

WORKDIR /home/btmeister
USER btmeister

ENTRYPOINT [ "/opt/btmeister/btmeister" ]
