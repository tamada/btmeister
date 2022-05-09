FROM rust:1.60 AS builder

WORKDIR /home/btmeister

COPY . .

RUN  adduser --home /home/btmeister --shell /bin/false btmeister \
  && cargo build --release \
  && cp target/release/btmeister /usr/local/bin/btmeister

USER btmeister

ENTRYPOINT [ "/usr/local/bin/btmeister" ]