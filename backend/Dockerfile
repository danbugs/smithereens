# Build stage
FROM rust:1.84-alpine AS builder

# Install build dependencies including static OpenSSL
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static

# Set environment variables for static linking
ENV OPENSSL_STATIC=1
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Build dependencies (this is cached if manifests don't change)
# Create dummy files for both lib.rs and main.rs based on your Cargo.toml structure
RUN mkdir src && \
    echo "// dummy lib" > src/lib.rs && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY src ./src

# Build the application
RUN touch src/main.rs src/lib.rs && \
    cargo build --release

# Runtime stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache ca-certificates libgcc

WORKDIR /app

# Copy the binary from builder
# Note: The binary name is "server" based on the error message
COPY --from=builder /app/target/release/server /app/server

# Make it executable
RUN chmod +x /app/server

# Expose port
EXPOSE 3000

# Run the binary
CMD ["./server"]