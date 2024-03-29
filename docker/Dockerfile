# build subdao-node
FROM paritytech/ci-linux:production as builder

LABEL maintainer="subdao-dev"
ARG RUST_VERSION=nightly-2020-10-06
ARG PROFILE=debug
ARG GIT_REPO="https://github.com/SubDAO-Network/subdao-node.git"
ARG GIT_BRANCH="3.0_dev"

RUN git clone --recursive ${GIT_REPO}
WORKDIR /builds/subdao-node
RUN rustup toolchain install ${RUST_VERSION}
RUN rustup target add wasm32-unknown-unknown --toolchain ${RUST_VERSION}
RUN rustup default ${RUST_VERSION}

RUN cargo build
RUN cp target/${PROFILE}/subdao-node /subdao-node

# build polkadot and subdao  frontend
FROM paritytech/ci-linux:production as frontend_builder

ARG FRONT_REPO="https://github.com/polkadot-js/apps.git"
ARG FRONT_BRANCH="v0.87.1"

RUN git clone -b ${FRONT_BRANCH} ${FRONT_REPO}
WORKDIR /builds/apps
RUN apt update && apt install gnupg2 curl ca-certificates -y
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - && echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN apt update && apt install yarn -y
RUN YARN_CHECKSUM_BEHAVIOR=update yarn && yarn build:www

FROM ubuntu
ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update && apt-get install curl ca-certificates npm nodejs -y
RUN npm install -g serve
COPY --from=builder /subdao-node /

COPY --from=frontend_builder /builds/apps/packages/apps /apps

EXPOSE 30333 9933 9944

