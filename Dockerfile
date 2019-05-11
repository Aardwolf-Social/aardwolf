FROM buildpack-deps:stretch

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
    \
    url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain nightly; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

RUN cargo +nightly install -f diesel_cli --no-default-features --features "postgres"
RUN mkdir --parents /src/aardwolf

COPY ./config/example.toml /src/aardwolf/aardwolf.toml
COPY . /src/aardwolf

RUN chmod -R +x /src/aardwolf/wait-for-it.sh
RUN rustup override set nightly

WORKDIR /src/aardwolf
RUN apt-get update  && apt-get install -y \
    gettext

RUN cargo build -Z unstable-options --out-dir /src/aardwolf --bin setup

RUN cargo build -Z unstable-options --out-dir /src/aardwolf --bin aardwolf-server --features rocket --verbose

# NODE
RUN curl -sL https://deb.nodesource.com/setup_12.x | bash
RUN apt-get install -y nodejs

# NPM
RUN curl -L https://www.npmjs.com/install.sh | sh

# WEBPACK for Aardwolf
RUN chmod -R +x /src/aardwolf/docker-npm.sh
RUN ./docker-npm.sh
