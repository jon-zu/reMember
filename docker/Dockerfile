# Use a minimal base image, such as Alpine Linux
FROM alpine:latest

ENV DATA_DIR="/usr/local/shroom"
ENV RUST_LOG=info

# Copy your pre-built game server binary into the container
COPY target/x86_64-unknown-linux-musl/release/mono /usr/local/bin/shroom_mono
RUN chmod +x /usr/local/bin/shroom_mono 

# tuf repo
RUN mkdir -p /usr/local/shroom/client_repo/data/tuf-repo
COPY client_repo/data/tuf-repo /usr/local/shroom/client_repo/data/tuf-repo

# game files
RUN mkdir -p /usr/local/shroom/game_data/rbin
COPY game_data/rbin /usr/local/shroom/game_data/rbin

#config
RUN mkdir -p /usr/local/shroom/config
COPY config /usr/local/shroom/config


# Set the binary as the entry point
ENTRYPOINT ["/usr/local/bin/shroom_mono"]