FROM rust:1.43

WORKDIR /app

COPY bin .

RUN rustup override set nightly \
  && cargo install --path .

CMD ["main"]
