# Multi-stage production Dockerfile with multi-architecture support
ARG TARGETARCH=aarch64
ARG TARGETOS=linux

# Stage 1: Build frontend with Trunk
FROM --platform=${TARGETOS}/${TARGETARCH} rust:1.94 as frontend-builder

# Install wasm target and Trunk
RUN rustup target add wasm32-unknown-unknown && \
    cargo install trunk

WORKDIR /app/frontend

# Copy frontend source
COPY frontend/Cargo.toml frontend/Cargo.lock* ./
COPY shared-types /app/shared-types
COPY frontend/src ./src
COPY frontend/index.html ./index.html
COPY frontend/styles.css ./styles.css
COPY frontend/Trunk.toml ./

# Build WASM
RUN trunk build --release

# Stage 2: Build backend
FROM --platform=${TARGETOS}/${TARGETARCH} rust:1.94 as backend-builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml ./backend/
COPY shared-types/Cargo.toml ./shared-types/
COPY frontend/Cargo.toml ./frontend/

# Create dummy source files to cache dependencies
RUN mkdir -p backend/src shared-types/src frontend/src
RUN echo "fn main() {}" > backend/src/main.rs
RUN echo "fn main() {}" > shared-types/src/lib.rs
RUN echo "fn main() {}" > frontend/src/main.rs

# Build dependencies
RUN cargo build --bin backend
RUN rm -rf backend/src shared-types/src frontend/src

# Copy source code
COPY backend ./backend
COPY shared-types ./shared-types
COPY frontend ./frontend

# Build backend
RUN cargo build --bin backend --release

# Stage 3: Production runtime
FROM --platform=${TARGETOS}/${TARGETARCH} debian:12-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    postgresql \
    postgresql-client \
    ca-certificates \
    netcat-traditional \
    curl \
    sudo \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 appuser

# Configure sudo for appuser to run postgres commands
RUN echo "appuser ALL=(postgres) NOPASSWD: /usr/bin/pg_createcluster, /usr/bin/pg_ctlcluster, /usr/bin/pg_isready, /usr/bin/psql" >> /etc/sudoers

# Set working directory
WORKDIR /app

# Copy built applications from builder stages
COPY --from=backend-builder /app/target/release/backend /app/backend
COPY --from=frontend-builder /app/frontend/dist /app/frontend/dist

# Copy database migrations
COPY backend/migrations /app/migrations

# Create necessary directories
RUN mkdir -p /app/uploads /app/var/lib/postgresql/data /app/var/run/postgresql

# Set ownership
RUN chown -R appuser:appuser /app

# Switch to app user
USER appuser

# Expose port
EXPOSE 8000

# Copy startup script and make executable
COPY --chmod=0755 docker-entrypoint.sh /app/docker-entrypoint.sh

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/ || exit 1

# Start script
ENTRYPOINT ["/app/docker-entrypoint.sh"]
CMD ["postgres"]
