FROM rust:1.49 as builder

WORKDIR /usr/src/LogToFile
COPY . .
RUN cargo install --path .


FROM alpine:3.12
COPY --from=builder /cargo/bin/LoggerServer /usr/local/bin/LoggerServer
CMD ["LoggerServer"]