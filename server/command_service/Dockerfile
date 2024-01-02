FROM rust:1.74.1 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .
FROM debian:buster-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
CMD ["myapp"]

# docker build -t my-rust-app .
# docker run -it --rm --name my-running-app my-rust-app