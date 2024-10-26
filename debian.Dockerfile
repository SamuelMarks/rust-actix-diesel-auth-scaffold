FROM rustlang/rust:nightly-slim AS builder

RUN apt update -qq && \
    apt install -y \
      libcom-err2 \
      libffi8 \
      libgmp10 \
      libgnutls30 \
      libgssapi-krb5-2 \
      libhogweed6 \
      libidn2-dev \
      libk5crypto3 \
      libkeyutils1 \
      libkrb5-3 \
      libldap-2.5-0 \
      libnettle8 \
      libp11-kit0 \
      libpq-dev \
      libsasl2-2 \
      libtasn1-6 \
      libunistring2

ARG ARCH=aarch64
ARG ARCH_VARIANT=arm64
ADD https://github.com/atkrad/wait4x/releases/download/v2.14.2/wait4x-linux-${ARCH_VARIANT}.tar.gz /tmp/dls/
RUN tar xf /tmp/dls/wait4x-linux-${ARCH_VARIANT}.tar.gz -C /usr/local/bin/ wait4x

WORKDIR /app

COPY . .

# ENV RUSTFLAGS='-C relocation-model=static -C strip=symbols'
# ENV CC=musl-gcc
# ENV RUSTFLAGS='-C target-feature=+crt-static'
RUN cargo build --all-features
