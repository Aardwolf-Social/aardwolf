FROM alpine:3.12

LABEL org.label-schema.name="aardwolf-alpine" \
    org.label-schema.description="Aardwolf-Social on Alpine" \
    org.label-schema.url="https://example.com/path/to/image/on/docker/hub" \
    org.label-schema.usage="https://github.com/Aardwolf-Social/aardwolf/blob/master/README.md" \
    org.label-schema.vcs-url="https://github.com/Aardwolf-Social/aardwolf" \
    org.label-schema.vendor="Aardwolf-Social" \
    org.label-schema.version="0.w.0" \
    maintainer="https://github.com/Aardwolf-Social"

# Configure a non-root user.
# We specify the UID so that Kubernetes has something for securityContexts.
RUN adduser -h /app -u 1000 -D aardwolf

# Install needed software.
RUN apk -U --no-cache add \
      bash \
      gcc \
      musl-dev \
			curl

# Install rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH

# Install specific rust nightly
RUN rustup toolchain install nightly

# Copy the code.
COPY --chown=aardwolf:aardwolf . /app

# Copy the entrypoint and make it executable.
COPY --chown=aardwolf:aardwolf ./docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh
RUN chmod  u=rx,g=rx,o=rx /usr/local/bin/docker-entrypoint.sh

# Expose the default port. This is often overridden in Docker CLI or Kubernetes.
EXPOSE 8080

# Run as our non-root user.
USER aardwolf

# Set the entrypoint and default command.
ENTRYPOINT ["docker-entrypoint.sh", "cargo run --bin aardwolf-server"]
