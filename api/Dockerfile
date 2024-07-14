FROM rust:1.79.0-slim-bullseye

WORKDIR /app

COPY . .
RUN cargo build --locked --release
RUN cp ./target/release/jellyfish-api /bin/app

RUN adduser --system --uid 1001 appuser
USER appuser

CMD ["/bin/app"]