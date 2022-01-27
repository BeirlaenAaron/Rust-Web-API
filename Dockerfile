# build
FROM rustlang/rust:nightly as builder
WORKDIR /usr/app
COPY src ./src
COPY ["Cargo.toml", "migrations", "diesel.toml", "./"]
RUN cargo build --release

# prod
FROM debian:bullseye-slim
RUN apt-get update && apt-get install libpq5 -y
COPY --from=builder /usr/app/target/release/rust-web-api .
CMD ["./rust-web-api"]