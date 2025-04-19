ARG RUST_VERSION

# 公式イメージを使用
FROM rust:${RUST_VERSION}-slim

ARG RUST_ANALYZER_VERSION
ARG RUST_USERNAME
ARG RUST_USER_UID
ARG RUST_USER_GID
ARG RUST_WORKDIR_NAME
ARG RUST_WORKDIR=/home/${RUST_USERNAME}/${RUST_WORKDIR_NAME}

RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y git \
    && rustup component add clippy rustfmt \
    && groupadd --gid ${RUST_USER_GID} ${RUST_USERNAME} \
    && useradd --uid ${RUST_USER_UID} --gid ${RUST_USER_GID} -m ${RUST_USERNAME} \
    && mkdir ${RUST_WORKDIR} \
    && chown -R ${RUST_USERNAME}:${RUST_USERNAME} /home/${RUST_USERNAME}
