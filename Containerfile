FROM docker.io/rust:1.97.1 AS build
WORKDIR /build
COPY Cargo.toml /build/Cargo.toml
COPY src /build/src
RUN cargo build --release

FROM quay.io/fedora/fedora-minimal:44
RUN dnf install -y --nodocs --setopt install_weak_deps=0 git && dnf clean all
WORKDIR /app
COPY --from=build /build/target/release/emitter /app/emitter
ENTRYPOINT [ "/app/emitter" ]
