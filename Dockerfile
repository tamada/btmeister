FROM rust:1.60 AS builder

WORKDIR /home/btmeister

COPY . .

RUN  adduser --home /home/btmeister --shell /bin/false btmeister \
  && cargo build --release \
  && cp target/release/btmeister /usr/local/bin/btmeister

# FROM alpine:3.10.1
# COPY --from=builder /usr/local/bin/btmeister /usr/local/bin/btmeister
# RUN  adduser -D btmeister
# WORKDIR /home/btmeister

USER btmeister

ENTRYPOINT [ "/usr/local/bin/btmeister" ]