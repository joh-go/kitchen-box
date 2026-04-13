#!/bin/bash

# Multi-architecture build script for Portainer deployment
# This script builds both ARM64 and AMD64 images for cross-platform deployment

set -e

# Configuration
IMAGE_NAME="kitchen-box"
REGISTRY="joesweb/"  # Docker Hub registry
TAG="latest"

echo "🏗️  Building multi-architecture Docker images..."

# Build and push multi-architecture image directly
if [ ! -z "$REGISTRY" ]; then
    echo "🚀 Building and pushing multi-architecture image..."
    
    # Get API_URL from environment (required for deployment)
    if [ -z "$API_URL" ]; then
        echo "Error: API_URL environment variable is required for deployment"
        echo "Usage: API_URL=https://your-domain.com ./build-multiarch.sh"
        exit 1
    fi
    echo "Using API_URL: $API_URL"
    
    # Build and push multi-architecture image in one step
    docker buildx build \
        --platform linux/amd64,linux/arm64 \
        --tag ${REGISTRY}${IMAGE_NAME}:${TAG} \
        --build-arg API_URL="$API_URL" \
        --push \
        -f Dockerfile \
        .
    
    echo "✅ Multi-architecture image pushed successfully!"
else
    echo "� Building locally only..."
    
    # Build AMD64 image locally
    echo "📦 Building AMD64 image..."
    docker buildx build \
        --platform linux/amd64 \
        --tag ${IMAGE_NAME}:${TAG}-amd64 \
        --load \
        -f Dockerfile \
        --build-arg TARGETARCH=amd64 \
        .
    
    # Build ARM64 image locally
    echo "� Building ARM64 image..."
    docker buildx build \
        --platform linux/arm64 \
        --tag ${IMAGE_NAME}:${TAG}-arm64 \
        --load \
        -f Dockerfile \
        --build-arg TARGETARCH=arm64 \
        .
fi

echo "🎉 Build completed!"
echo ""
echo "📝 Usage in Portainer:"
echo "1. Add your registry to Portainer"
echo "2. Pull image: ${REGISTRY}${IMAGE_NAME}:${TAG}"
echo "3. Deploy with docker-compose.portainer.yml"
