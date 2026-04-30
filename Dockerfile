# Multi-stage production Dockerfile with multi-architecture support
ARG TARGETARCH=amd64
ARG TARGETOS=linux

# Stage 1: Build frontend
FROM --platform=${TARGETOS}/${TARGETARCH} rust:1.94 as frontend-builder

# Install system dependencies for frontend
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    binaryen \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js and npm for Tailwind CSS processing
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash - && \
    apt-get install -y nodejs

# Install trunk for frontend building
RUN cargo install trunk

# Install WebAssembly target for frontend compilation
RUN rustup target add wasm32-unknown-unknown

# Set working directory
WORKDIR /app

# Copy frontend files
COPY frontend/Cargo.toml frontend/Cargo.lock ./frontend/
COPY shared-types/Cargo.toml ./shared-types/

# Create dummy source files to cache dependencies
RUN mkdir -p frontend/src shared-types/src
RUN echo "fn main() {}" > frontend/src/main.rs
RUN echo "fn main() {}" > shared-types/src/lib.rs

# Build frontend dependencies
RUN cd frontend && cargo build
RUN rm -rf frontend/src shared-types/src

# Copy actual source code
COPY frontend ./frontend
COPY shared-types ./shared-types

RUN cd frontend && trunk build --release

# Install Tailwind CSS locally in frontend directory
RUN cd frontend && npm install

# Build frontend with PostCSS
RUN cd frontend && npm run build-css
RUN cd frontend && ls -la src/output.css
RUN cd frontend && cp src/output.css dist/

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
