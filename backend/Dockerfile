# --------------------------------------
# Stage 1: Build the app
# --------------------------------------
FROM rust:1.84-slim as builder

# Install needed build libraries
RUN apt-get update && apt-get install -y libpq-dev pkg-config build-essential

# Set working directory at the workspace root
WORKDIR /usr/src/smithereens

# Copy the whole workspace (excluding files from .dockerignore)
COPY . .

# Build the real application
RUN cargo build --manifest-path ./backend/Cargo.toml --release

# --------------------------------------
# Stage 2: Create runtime container
# --------------------------------------
FROM debian:bookworm-slim

# Install only runtime libraries needed
RUN apt-get update && apt-get install -y libpq5 ca-certificates && apt-get clean

# Set working directory
WORKDIR /usr/src/smithereens

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/smithereens/target/release/smithe_backend .

# Expose the port Rocket will listen on
EXPOSE 8000

# Command to run the app
CMD ["./smithe_backend"]
