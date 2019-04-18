FROM clux/muslrust AS base
RUN apt update && \
    apt install -y \
        unzip
RUN curl -OL https://github.com/google/protobuf/releases/download/v3.2.0/protoc-3.2.0-linux-x86_64.zip && \
    unzip protoc-3.2.0-linux-x86_64.zip -d protoc3 &&\
    mv protoc3/bin/* /usr/local/bin/ && \
    mv protoc3/include/* /usr/local/include/
RUN rustup component add \
        rustfmt \
        clippy && \
    cargo install \
        cargo-watch
#        cargo-tarpaulin
WORKDIR /app

FROM base AS dev
COPY . .
ENV RUST_LOG="example_service=debug"
EXPOSE 50051
CMD ["cargo", "watch", "-x", "run"]

FROM base AS build
COPY --from=dev /app /app
RUN cargo build --release && \
    mv /app/target/x86_64-unknown-linux-musl/release/example-service /bin/example-service
RUN strip /bin/example-service

FROM scratch AS release
COPY --from=build /bin/example-service /bin/example-service
ENV RUST_LOG="example_service=info"
EXPOSE 50051
CMD ["example-service"]
