FROM rust:1.49-buster AS builder

RUN apt-get update && apt-get install -y apt-transport-https
RUN apt-get install -y git

RUN git clone https://github.com/Johan3F/LogToFile.git
WORKDIR /LogToFile
RUN cargo install --path .




FROM debian:buster-slim
MAINTAINER Johan3F <jvandewallemuniz@gmail.com>

COPY --from=builder /usr/local/cargo/bin/LoggerServer /LoggerServer

VOLUME ["/app"]
WORKDIR /app

ENTRYPOINT ["/LoggerServer"]