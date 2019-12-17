FROM rust:1.39 as app-builder
WORKDIR /usr/src/aardwolf
COPY . .
RUN cp config/example.toml aardwolf.toml \
    && apt-get update \
    && apt-get install -y libpq-dev gettext \
    && cargo build --release --bin aardwolf-server --bin setup


FROM rust:1.39 as desiel-builder
RUN cargo install diesel_cli --no-default-features --features "postgres"


FROM node:13.3 as asset-builder
WORKDIR /usr/src/aardwolf
COPY . .
RUN npm install \
  && npm run build


FROM debian:buster-slim
WORKDIR /etc/aardwolf
RUN mkdir -p assets \
  && apt-get update \
  && apt-get install -y libpq-dev openssl \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/
COPY config/example.toml aardwolf.toml
COPY docker-entrypoint.sh /
COPY aardwolf-models/migrations /usr/src/aardwolf/migrations
COPY web assets/web
COPY aardwolf-templates/templates assets/templates
COPY --from=asset-builder /usr/src/aardwolf/dist assets/dist
COPY --from=app-builder /usr/src/aardwolf/target/release/aardwolf-server /usr/local/bin/
COPY --from=app-builder /usr/src/aardwolf/target/release/setup /usr/local/bin/aardwolf-setup
COPY --from=desiel-builder /usr/local/cargo/bin/diesel /usr/local/bin/
EXPOSE 8080
ENTRYPOINT /docker-entrypoint.sh
