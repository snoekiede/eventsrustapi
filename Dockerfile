FROM ubuntu AS builder
RUN apt install rustc
#RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN source $HOME/.cargo/env
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:bullseye as runner
RUN apt-get update \
     && apt-get -y install libpq-dev gcc
COPY --from=builder /usr/local/cargo/bin/eventsapi /usr/local/bin/eventsapi
EXPOSE 8080
CMD ["eventsapi"]
