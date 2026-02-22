# Stage 1: Build Frontend
FROM oven/bun:1.3.9 AS frontend-builder
WORKDIR /app/frontend
COPY frontend/package.json frontend/bun.lockb ./
RUN bun install
COPY frontend/ ./
RUN bun run build

# Stage 2: Build Backend
FROM rust:1.80-slim AS backend-builder
WORKDIR /app/backend
# Install dependencies for compilation
RUN apt-get update && apt-get install -y pkg-config libssl-dev
COPY backend/Cargo.toml backend/Cargo.lock ./
# Create dummy main to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src
COPY backend/src ./src
RUN cargo build --release

# Stage 3: Final Image
FROM debian:bookworm-slim
WORKDIR /app

# Create a non-root user and group, install dependencies, and set permissions
RUN groupadd -r appgroup && useradd -r -g appgroup appuser && \
    apt-get update && apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/* && \
    mkdir -p /app && chown -R appuser:appgroup /app

COPY --from=backend-builder /app/backend/target/release/backend ./server
COPY --from=frontend-builder /app/frontend/.output ./frontend/.output
COPY backend/.env .env

USER appuser

EXPOSE 8080
CMD ["./server"]
