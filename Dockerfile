# Multi-stage production Dockerfile
# Build: docker build -t kitchen-box -f Dockerfile ..
ARG TARGETARCH=aarch64
ARG TARGETOS=linux

# Stage 1: Build frontend with Trunk
FROM --platform=${TARGETOS}/${TARGETARCH} rust:1.94 as frontend-builder

RUN rustup target add wasm32-unknown-unknown && \
    cargo install trunk

WORKDIR /app

# Copy shared library (parent context)
COPY shared /app/shared

# Copy kitchen-box workspace
COPY kitchen-box/Cargo.toml kitchen-box/Cargo.lock* /app/kitchen-box/
COPY kitchen-box/frontend /app/kitchen-box/frontend
COPY kitchen-box/shared-types /app/kitchen-box/shared-types

WORKDIR /app/kitchen-box/frontend
RUN trunk build --release

# Stage 2: Build backend
FROM --platform=${TARGETOS}/${TARGETARCH} rust:1.94 as backend-builder

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev postgresql-client && rm -rf /var/lib/apt/lists/*

WORKDIR /app/kitchen-box

COPY kitchen-box/Cargo.toml kitchen-box/Cargo.lock ./
COPY kitchen-box/backend/Cargo.toml ./backend/
COPY kitchen-box/shared-types/Cargo.toml ./shared-types/
COPY kitchen-box/frontend/Cargo.toml ./frontend/

RUN mkdir -p backend/src shared-types/src frontend/src
RUN echo "fn main() {}" > backend/src/main.rs
RUN echo "fn main() {}" > shared-types/src/lib.rs
RUN echo "fn main() {}" > frontend/src/main.rs
RUN cargo build --bin backend
RUN rm -rf backend/src shared-types/src frontend/src

COPY kitchen-box/backend ./backend
COPY kitchen-box/shared-types ./shared-types
COPY kitchen-box/frontend ./frontend
RUN cargo build --bin backend --release

# Stage 3: Production runtime
FROM --platform=${TARGETOS}/${TARGETARCH} debian:12-slim

RUN apt-get update && apt-get install -y \
    postgresql postgresql-client ca-certificates netcat-traditional curl sudo \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 appuser
RUN echo "appuser ALL=(postgres) NOPASSWD: /usr/bin/pg_createcluster, /usr/bin/pg_ctlcluster, /usr/bin/pg_isready, /usr/bin/psql" >> /etc/sudoers

WORKDIR /app

COPY --from=backend-builder /app/kitchen-box/target/release/backend /app/backend
COPY --from=frontend-builder /app/kitchen-box/frontend/dist /app/frontend/dist
COPY kitchen-box/backend/migrations /app/migrations

RUN mkdir -p /app/uploads /app/var/lib/postgresql/data /app/var/run/postgresql
RUN chown -R appuser:appuser /app

USER appuser
EXPOSE 8000

COPY kitchen-box/docker-entrypoint.sh /app/docker-entrypoint.sh

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/ || exit 1

ENTRYPOINT ["/app/docker-entrypoint.sh"]
CMD ["postgres"]
