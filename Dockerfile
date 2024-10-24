FROM rustlang/rust:nightly-alpine

WORKDIR /src
COPY . .

RUN apk add --no-cache musl-dev libpq-dev postgresql16-dev && \
    cargo build --release

ENTRYPOINT ["cargo", "test"]
