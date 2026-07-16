#!/bin/bash
set -e

# --- PostgreSQL setup ---
PGDATA="${PGDATA:-/var/lib/postgresql/data}"
POSTGRES_USER="${POSTGRES_USER:-postgres}"
POSTGRES_PASSWORD="${POSTGRES_PASSWORD:-postgres}"

PG_BINDIR=$(ls -d /usr/lib/postgresql/*/bin 2>/dev/null | head -1)
[ -n "$PG_BINDIR" ] || { echo "ERROR: PostgreSQL not found"; exit 1; }

PATH="$PG_BINDIR:$PATH"

mkdir -p /var/run/postgresql /var/log/postgresql
sudo chown postgres:postgres /var/run/postgresql /var/log/postgresql
sudo chown -R postgres:postgres "$PGDATA"

if ! sudo -u postgres env "PATH=$PATH" test -f "$PGDATA/PG_VERSION" 2>/dev/null; then
    echo "==> Initializing PostgreSQL..."
    sudo -u postgres env "PATH=$PATH" initdb -D "$PGDATA"
fi

sudo tee "$PGDATA/pg_hba.conf" > /dev/null <<-EOF
local all all trust
host all all 127.0.0.1/32 trust
host all all ::1/128 trust
EOF

if ! sudo -u postgres env "PATH=$PATH" pg_isready -q 2>/dev/null; then
    echo "==> Starting PostgreSQL..."
    sudo -u postgres env "PATH=$PATH" pg_ctl -D "$PGDATA" -l /var/log/postgresql/startup.log -w start
fi

for i in $(seq 1 15); do
    if sudo -u postgres env "PATH=$PATH" pg_isready -q 2>/dev/null; then break; fi
    echo "    Waiting for PostgreSQL... ($i/15)"
    sleep 1
done

sudo -u postgres env "PATH=$PATH" psql -c "ALTER USER $POSTGRES_USER WITH PASSWORD '$POSTGRES_PASSWORD'" 2>/dev/null || true

export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432/postgres"
export ROCKET_ADDRESS=0.0.0.0
export ROCKET_PORT=8000
export ROCKET_SECRET_KEY="${ROCKET_SECRET_KEY:-$(head -c 32 /dev/urandom | base64 | tr -d '\n')}"

# Migrations are embedded in the binary (diesel embed_migrations)
echo "==> Starting Kitchen Box backend..."
exec /app/backend
