FROM docker.io/rust:1.97.1 AS build
WORKDIR /build
COPY Cargo.toml /build/Cargo.toml
COPY src /build/src
RUN cargo build --release

FROM quay.io/fedora/fedora-minimal:44
WORKDIR /app
RUN dnf install -y --nodocs --setopt install_weak_deps=0 git gnupg2 openssh-clients && dnf clean all
RUN mkdir -p /root/.ssh
COPY id_ed25519 /root/.ssh/id_ed25519
RUN ssh-keyscan github.com >> /root/.ssh/known_hosts
RUN chmod 700 /root/.ssh/ && chmod 600 /root/.ssh/*
COPY secret.keys.asc public.keys.asc ownertrust.txt /app/
RUN gpg --batch --import secret.keys.asc public.keys.asc && gpg --batch --import-ownertrust < ownertrust.txt && gpgconf --kill all && rm -f /root/.gnupg/*.lock
COPY --from=build /build/target/release/emitter /app/emitter
ENTRYPOINT [ "/app/emitter" ]
