# syntax=docker/dockerfile:1
ARG TARGETARCH
ARG TARGETOS

# ============================================================
# Stage 1: Frontend builder (Yew WASM via Trunk)
# ============================================================
FROM --platform=${TARGETOS}/${TARGETARCH} rust:1-slim-bookworm AS frontend-builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli --version 0.2.108 --root /usr/local

WORKDIR /app

COPY shared /app/shared

COPY kitchen-box/Cargo.toml kitchen-box/Cargo.lock* /app/kitchen-box/
COPY kitchen-box/backend/Cargo.toml /app/kitchen-box/backend/Cargo.toml
COPY kitchen-box/frontend /app/kitchen-box/frontend
COPY kitchen-box/shared-types /app/kitchen-box/shared-types

RUN mkdir -p /app/kitchen-box/backend/src && echo "fn main() {}" > /app/kitchen-box/backend/src/main.rs

WORKDIR /app/kitchen-box/frontend

ENV API_BASE=""
RUN cargo build --target wasm32-unknown-unknown --release

RUN rm -rf dist && \
    mkdir -p dist && \
    wasm-bindgen --target web --out-dir dist --out-name frontend \
        /app/kitchen-box/target/wasm32-unknown-unknown/release/frontend.wasm --no-typescript && \
    cp styles.css dist/ && \
    cp app.css dist/ && \
    cp public/favicon.svg dist/ && \
    sed -e 's/data-trunk rel="css"/rel="stylesheet"/g' \
        -e 's|</body>|<script type="module">import init from "./frontend.js";init();</script></body>|' \
        index.html > dist/index.html

# ============================================================
# Stage 2: Backend builder (Rust/Rocket)
# ============================================================
FROM --platform=${TARGETOS}/${TARGETARCH} rust:1-slim-bookworm AS backend-builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev libpq-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app/kitchen-box

COPY shared /app/shared

COPY kitchen-box/Cargo.toml kitchen-box/Cargo.lock* ./
COPY kitchen-box/backend/Cargo.toml ./backend/
COPY kitchen-box/shared-types/Cargo.toml ./shared-types/
COPY kitchen-box/frontend/Cargo.toml ./frontend/

RUN mkdir -p backend/src shared-types/src frontend/src
RUN echo "fn main() {}" > backend/src/main.rs
RUN echo "fn main() {}" > shared-types/src/lib.rs
RUN echo "fn main() {}" > frontend/src/main.rs
RUN cargo build -p backend --release 2>/dev/null || true
RUN rm -f backend/src/main.rs shared-types/src/lib.rs frontend/src/main.rs

COPY kitchen-box/backend ./backend
COPY kitchen-box/shared-types ./shared-types
COPY --from=frontend-builder /app/kitchen-box/frontend/dist ./frontend/dist
RUN mkdir -p frontend/src && echo "fn main() {}" > frontend/src/main.rs
RUN find backend/src shared-types/src -name '*.rs' -exec touch {} + && cargo build -p backend --release

# ============================================================
# Stage 3: Runtime
# ============================================================
FROM --platform=${TARGETOS}/${TARGETARCH} debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    postgresql postgresql-client ca-certificates curl sudo \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 appuser
RUN echo "appuser ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

WORKDIR /app

COPY --from=backend-builder /app/kitchen-box/target/release/backend /app/backend
COPY --from=frontend-builder /app/kitchen-box/frontend/dist /app/frontend/dist
COPY kitchen-box/backend/migrations /app/migrations

RUN mkdir -p /app/uploads
COPY kitchen-box/docker-entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh && chown -R appuser:appuser /app

USER appuser

ENV FRONTEND_DIST=/app/frontend/dist
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

EXPOSE 8000

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/ || exit 1

ENTRYPOINT ["/app/entrypoint.sh"]
