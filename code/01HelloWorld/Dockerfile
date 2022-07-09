# Create the build container to compile the hello world program
FROM rust:latest as builder
ENV USER root
RUN cargo new hello
WORKDIR hello
RUN cargo build --release

# Create the execution container by copying the compiled hello world to it and running it
FROM scratch
COPY --from=builder /hello/target/release/hello /hello
CMD ["/hello"]
