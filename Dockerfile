FROM rust:1.96 as builder
WORKDIR /build

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
RUN touch src/main.rs
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app
COPY --from=builder /build/target/release/ephemeria-service /app/ephemeria-service
USER nonroot:nonroot
CMD ["/app/ephemeria-service"]
