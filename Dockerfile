FROM rust:latest as builder

WORKDIR /app
COPY . .
RUN apt-get update 
RUN apt-get install musl-tools -y
RUN apt-get install openssl -y
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/rust-list-site /
EXPOSE 8080
CMD ["/app/target/release/rust-list-site"]
