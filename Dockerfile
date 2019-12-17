# NB: Alpine Linux won't like dynamically linked binaries, so we use MUSL.
# Build everything in an Ubuntu base image. It has decent MUSL support.
FROM ubuntu:xenial as build
WORKDIR /tmp
RUN apt-get update \
    && apt-get install -y \
        build-essential \
        curl \
        make \
        musl-dev \
        musl-tools
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh  -s -- --default-toolchain stable -y \
    && $HOME/.cargo/bin/rustup target add x86_64-unknown-linux-musl
COPY Cargo.toml . 
COPY Cargo.lock . 
COPY src/ ./src
RUN $HOME/.cargo/bin/cargo build --release --target x86_64-unknown-linux-musl

# ...and copy it into minimal Alpine Linux container.
FROM alpine:3.7
WORKDIR /
COPY --from=build /tmp/target/x86_64-unknown-linux-musl/release/testcontainer /
CMD /testcontainer

