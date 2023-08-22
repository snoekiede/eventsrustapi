FROM rust:1.70 AS builder
WORKDIR /app
COPY . .
RUN cargo install --path .
RUN cargo build --release

FROM debian:buster-slim as runner
RUN apt-get update \
    && apt-get -y install libpq-dev gcc
COPY --from=builder /usr/local/cargo/bin/eventsapi /usr/local/bin/eventsapi
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["eventsapi"]