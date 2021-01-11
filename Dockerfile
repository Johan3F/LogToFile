FROM rust:1.49-buster AS builder

RUN apt-get update && apt-get install -y apt-transport-https
RUN apt-get install -y git

RUN git clone https://github.com/Johan3F/LogToFile.git
WORKDIR /LogToFile
RUN cargo install --path .


FROM debian:buster-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /cargo/bin/loggerServer /usr/local/bin/loggerServer
CMD ["loggerServer"]