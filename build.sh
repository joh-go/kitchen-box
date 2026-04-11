#!/bin/bash

set -e

echo "Building Recipes Application..."

# Build frontend
echo "Building frontend..."
cd frontend
trunk build --release
cd ..

# Build backend
echo "Building backend..."
cd backend
cargo build --release
cd ..

echo "Build completed successfully!"
echo "Frontend assets are in: frontend/dist/"
echo "Backend binary is in: backend/target/release/backend"
