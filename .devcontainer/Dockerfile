######################################################
# 引数
# 値はdevcontainer.env で管理している。
######################################################
ARG RUST_VERSION=1.70.0

# 公式イメージを使用
FROM rust:${RUST_VERSION}-slim

ARG USERNAME=atcoder
ARG USER_UID=1000
ARG USER_GID=1000
ARG WORKDIR_NAME=app
ARG WORKDIR=/home/${USERNAME}/${WORKDIR_NAME}

RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y git \
    && groupadd --gid ${USER_GID} ${USERNAME} \
    && useradd --uid ${USER_UID} --gid ${USER_GID} -m ${USERNAME} \
    && mkdir ${WORKDIR} \
    && chown -R ${USERNAME}:${USERNAME} /home/${USERNAME}
