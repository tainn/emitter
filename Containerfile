FROM docker.io/rust:1.97.1 AS build
WORKDIR /build
COPY Cargo.toml /build/Cargo.toml
COPY src /build/src
RUN cargo build --release

FROM quay.io/fedora/fedora-minimal:44
RUN dnf install -y --nodocs --setopt install_weak_deps=0 git gnupg2 && dnf clean all
WORKDIR /app
COPY --from=build /build/target/release/emitter /app/emitter
COPY secret.keys.asc public.keys.asc ownertrust.txt /app/
RUN gpg --batch --import secret.keys.asc public.keys.asc && gpg --batch --import-ownertrust < ownertrust.txt && gpgconf --kill all && rm -f ~/.gnupg/*.lock
ENTRYPOINT [ "/app/emitter" ]
