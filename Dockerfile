#Official Rust image.
FROM rust:1.66 AS builder
WORKDIR /usr/src/connector
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/connector/target/release/connector /usr/local/bin/connector
CMD ["connector"]