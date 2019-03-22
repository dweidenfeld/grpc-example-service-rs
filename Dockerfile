FROM rust AS build

# Install dependencies
RUN apt update && \
    apt install -y protobuf-compiler
RUN cargo install cargo-watch && \
    rustup component add rustfmt

# App specifics
WORKDIR /app
COPY api api
COPY src src
COPY build.rs .
COPY Cargo.lock .
COPY Cargo.toml .

RUN cargo build --release

ENV RUST_LOG=debug
EXPOSE 50051
CMD ["cargo", "watch", "-x", "'run'"]


FROM debian:stretch AS release

COPY --from=build /app/target/release/grpc-example-service-rs /grpc-example-service-rs

ENV RUST_LOG=info
EXPOSE 50051
CMD ["/grpc-example-service-rs"]
