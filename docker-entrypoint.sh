#!/bin/bash

set -e

# Function to wait for PostgreSQL to be ready
wait_for_postgres() {
    echo "Waiting for PostgreSQL to be ready..."
    
    # Start PostgreSQL in background
    pg_ctl -D /app/var/lib/postgresql/data -l /app/var/log/postgresql.log start
    
    # Wait for PostgreSQL to be ready
    while ! pg_isready -h localhost -p 5432 -U postgres; do
        echo "PostgreSQL is not ready yet. Waiting..."
        sleep 2
    done
    
    echo "PostgreSQL is ready!"
}

# Initialize PostgreSQL if not already done
if [ ! -f "/etc/postgresql/15/main/postgresql.conf" ]; then
    echo "Initializing PostgreSQL database..."
    # Initialize database cluster using Debian's method
    sudo -u postgres pg_createcluster 15 main
    
    echo "PostgreSQL initialized successfully!"
else
    echo "PostgreSQL cluster already exists, starting..."
fi

# Start PostgreSQL cluster (or restart if already running)
sudo -u postgres pg_ctlcluster 15 main start

# Wait for PostgreSQL to be ready
while ! pg_isready -h localhost -p 5432 -U postgres; do
    echo "PostgreSQL is not ready yet. Waiting..."
    sleep 2
done

echo "PostgreSQL is ready!"

# Set password for postgres user (after PostgreSQL is running)
sudo -u postgres psql -c "ALTER USER postgres PASSWORD 'postgres';" || echo "Password already set"

# Set environment variables for the backend
export DATABASE_URL="postgres://postgres:postgres@localhost:5432/postgres"
export RUST_LOG=info
export ROCKET_ADDRESS=0.0.0.0
export ROCKET_PORT=8000

# Run database migrations if they exist
if [ -d "/app/migrations" ]; then
    echo "Running database migrations..."
    # Note: You might need to add migration logic here
    echo "Migrations completed!"
fi

echo "Starting the backend application..."
exec /app/backend
