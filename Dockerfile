FROM rust AS base
WORKDIR /app
RUN apt update && \
    apt install -y protobuf-compiler libssl-dev pkg-config cmake zlib1g-dev
RUN rustup component add rustfmt clippy && \
    RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-watch cargo-tarpaulin

FROM base AS dev
COPY api api
COPY Cargo.toml .
COPY src src
COPY build.rs .
ENV RUST_LOG="example_service=debug"
EXPOSE 50051
CMD ["cargo", "watch", "-x", "run"]

FROM base AS build
COPY --from=dev /app /app
RUN cargo fmt && \
    cargo clippy && \
    cargo build --release
RUN strip /app/target/release/example-service

FROM bitnami/minideb:stretch AS release
COPY --from=build /app/target/release/example-service /app
ENV RUST_LOG="example_service=info"
EXPOSE 50051
CMD ["/app"]
