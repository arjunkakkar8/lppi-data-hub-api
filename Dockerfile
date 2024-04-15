# Build stage
FROM rust:bookworm AS builder
 
WORKDIR /app
COPY . .
RUN cargo build --release
 
# Final run stage
FROM debian:bookworm-slim AS runner
 
WORKDIR /app
COPY --from=builder /app/data/processed_2022.parquet /app/data/processed_2022.parquet
COPY --from=builder /app/target/release/lppi-data-hub-api /app/lppi-data-hub-api
EXPOSE 8080
CMD ["/app/lppi-data-hub-api"]
