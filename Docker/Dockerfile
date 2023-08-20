FROM rustlang/rust:nightly 

LABEL org.label-schema.name="aardwolf-rust-builder" \
    org.label-schema.description="Aardwolf-Social on Debian" \
    org.label-schema.url="https://example.com/path/to/image/on/docker/hub" \
    org.label-schema.usage="https://github.com/Aardwolf-Social/aardwolf/blob/master/README.md" \
    org.label-schema.vcs-url="https://github.com/Aardwolf-Social/aardwolf" \
    org.label-schema.vendor="Aardwolf-Social" \
    org.label-schema.version="0.w.0" \
    maintainer="https://github.com/Aardwolf-Social"

# Configure a non-root user.
# We specify the UID so that Kubernetes has something for securityContexts.
RUN adduser -h /app -u 1000 -D aardwolf
# Not sure why but debian complains about aardwolf not existing using the above command??
RUN adduser aardwolf

# Update
RUN apt-get update

# Install needed software.
RUN apt-get -y install \
      bash \
      gcc \
      musl-dev \
			postgresql-client \
			gettext \
			curl

# Copy the code.
COPY --chown=aardwolf:aardwolf . /app
COPY --chown=aardwolf:aardwolf ./config/example.toml /app/aardwolf.toml

# This is for splitting into builder and app
# Use nightly, install diesel 
#RUN rustup override set nightly
#RUN cargo install -f diesel_cli --no-default-features --features postgres
# Copmile Aardwolf
#RUN cargo build --bin setup 

# Copy the entrypoint and make it executable.
COPY --chown=aardwolf:aardwolf ./docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh
RUN chmod  u=rx,g=rx,o=rx /usr/local/bin/docker-entrypoint.sh

# Expose the default port. This is often overridden in Docker CLI or Kubernetes.
EXPOSE 8080

# Run as our non-root user.
USER aardwolf

# Set the entrypoint and default command.
ENTRYPOINT ["docker-entrypoint.sh", "cargo run --bin aardwolf-server"]
