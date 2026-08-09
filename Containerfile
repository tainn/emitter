FROM docker.io/rust:1.97.1-alpine3.24 AS build
ARG TARGETARCH
WORKDIR /build
COPY src /build/src
COPY Cargo.toml /build/Cargo.toml
RUN apk add --no-cache musl-dev
RUN \
  case "${TARGETARCH}" in \
    "amd64") RUSTARCH="x86_64" ;; \
    "arm64") RUSTARCH="aarch64" ;; \
  esac; \
  rustup target add ${RUSTARCH}-unknown-linux-musl; \
  cargo build --release --target ${RUSTARCH}-unknown-linux-musl; \
  cp /build/target/${RUSTARCH}-unknown-linux-musl/release/emitter /build/emitter

FROM quay.io/fedora/fedora-minimal:44
RUN \
  dnf install -y --nodocs --setopt install_weak_deps=0 git && \
  dnf clean all
WORKDIR /app
COPY --from=build /build/emitter /app/emitter
