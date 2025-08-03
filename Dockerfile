FROM rust:bookworm as build

# create a new empty shell project
RUN USER=root cargo new --bin scryfall_archiver
WORKDIR /scryfall_archiver

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/scryfall_archiver*
RUN cargo build --release

# our final base
FROM debian:bookworm-slim

RUN apt-get update && apt install -y openssl

# copy the build artifact from the build stage
COPY --from=build /scryfall_archiver/target/release/scryfall-archiver .

RUN apt-get install curl ca-certificates --no-install-recommends -yqq

RUN mkdir -p /data

# set the startup command to run your binary
CMD ["./scryfall-archiver"]


