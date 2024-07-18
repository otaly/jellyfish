FROM rust:1.79.0-slim-bookworm AS build

WORKDIR /build

COPY Cargo.lock Cargo.toml ./
RUN mkdir src \
    && echo "// dummy file" > src/lib.rs \
    && cargo build --release

COPY src src
RUN cargo build --locked --release

FROM debian:bookworm-slim

RUN adduser --system --uid 1001 appuser
USER appuser
EXPOSE 4000

COPY --from=build /build/target/release/jellyfish-api /bin/app
CMD ["/bin/app"]