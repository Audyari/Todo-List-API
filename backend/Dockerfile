FROM rust:1.83 as builder

WORKDIR /app

# Copy source files
COPY . .

# Build the application
RUN cargo build --release

# Final stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/backend ./backend

# Expose port 3000
EXPOSE 3000

# Run the application
CMD ["./backend"]