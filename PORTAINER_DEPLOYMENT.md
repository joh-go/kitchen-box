# Portainer Deployment Guide

This guide explains how to deploy the Recipes application on a server with Portainer using multi-architecture Docker images.

## Overview

The application supports both ARM64 (for local development on Mac) and x86_64 (for server deployment) architectures.

## Files Created/Modified

1. **Dockerfile** - Extended with multi-architecture support
2. **build-multiarch.sh** - Build script for both architectures
3. **docker-compose.portainer.yml** - Production compose file for Portainer

## Build Process

### Option 1: Local Build (Testing)

```bash
# Build only x86_64 image for testing
./build-multiarch.sh

# Build with custom registry
REGISTRY=your-registry.com/ ./build-multiarch.sh
```

### Option 2: Automated Build with Registry

1. Set your registry in `build-multiarch.sh`
2. Run the build script:
   ```bash
   REGISTRY=your-registry.com/ ./build-multiarch.sh
   ```

## Portainer Deployment

### Step 1: Setup Registry

1. In Portainer, go to **Registries**
2. Add your container registry credentials
3. Ensure registry is accessible

### Step 2: Deploy Application

1. In Portainer, go to **Stacks** → **Add stack**
2. Choose **Web editor**
3. Paste the contents of `docker-compose.portainer.yml`
4. Update the image name to match your registry:
   ```yaml
   image: your-registry.com/recipes-recipes:latest
   ```
5. Click **Deploy the stack**

### Step 3: Verify Deployment

1. Check stack status in Portainer
2. Access application at `http://your-server:8000`
3. Check logs for any issues

## Architecture Support

- **ARM64**: For development on Mac with Apple Silicon
- **x86_64**: For production servers
- **Multi-arch**: Single image supports both architectures

## Environment Variables

- `DATABASE_URL`: PostgreSQL connection string
- `RUST_LOG`: Logging level (info/debug)
- `POSTGRES_*`: PostgreSQL configuration

## Volumes

- **recipes_uploads**: User uploaded files
- **recipes_pgdata**: PostgreSQL data
- **recipes_logs**: PostgreSQL logs

## Troubleshooting

### Build Issues
```bash
# Check buildx is installed
docker buildx version

# Install if missing
docker buildx install
```

### Architecture Issues
```bash
# Verify image architecture
docker manifest inspect your-registry.com/recipes-recipes:latest
```

### Portainer Issues
- Check registry credentials
- Verify network connectivity
- Review container logs
- Ensure ports are not blocked

## Production Considerations

1. **Security**: Use HTTPS with reverse proxy
2. **Backups**: Configure volume backups
3. **Monitoring**: Set up health checks
4. **Updates**: Use semantic versioning
5. **Scaling**: Consider load balancer for high traffic

## Local Development

For local development on Mac ARM64:
```bash
docker-compose -f docker-compose.portainer.yml up --build
```

This will use the ARM64 image for optimal performance on Apple Silicon.
