# Production Deployment Guide

This guide covers deploying the Recipes application to production using Docker.

## Quick Start

1. **Build and run with Docker Compose**
   ```bash
   # Set your JWT secret
   export JWT_SECRET="your-super-secure-random-jwt-secret-here"
   
   # Build and start the application
   docker-compose -f docker-compose.prod.yml up --build -d
   ```

2. **Access the application**
   - Application URL: http://localhost:8000
   - Health check: http://localhost:8000/

## Configuration

### Environment Variables

Create a `.env.production` file with the following variables:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
RUST_LOG=info
ROCKET_ADDRESS=0.0.0.0
ROCKET_PORT=8000
JWT_SECRET=your-super-secure-random-jwt-secret-here
CORS_ORIGINS=https://yourdomain.com,https://www.yourdomain.com
```

**Important**: Always generate a secure random JWT secret for production!

### Security Considerations

1. **JWT Secret**: Generate a cryptographically secure random string:
   ```bash
   openssl rand -base64 64
   ```

2. **CORS Origins**: Update `CORS_ORIGINS` with your actual domain(s)

3. **Database**: The database runs inside the container, but you can mount external volumes for persistence

## Deployment Options

### Option 1: Docker Compose (Recommended)

```bash
# Production deployment
docker-compose -f docker-compose.prod.yml up -d

# View logs
docker-compose -f docker-compose.prod.yml logs -f

# Stop the application
docker-compose -f docker-compose.prod.yml down
```

### Option 2: Docker Standalone

```bash
# Build the image
docker build -t recipes-app .

# Run the container
docker run -d \
  --name recipes-app \
  -p 8000:8000 \
  -e JWT_SECRET="your-super-secure-jwt-secret" \
  -v recipes_data:/app/var/lib/postgresql/data \
  -v recipes_uploads:/app/uploads \
  recipes-app
```

## Architecture

The production deployment uses a single container that includes:

- **PostgreSQL Database**: Runs on localhost:5432 inside the container
- **Rocket Backend**: Serves API on port 8000
- **Yew Frontend**: Compiled static assets served by Rocket
- **File Uploads**: Stored in `/app/uploads` directory

## Data Persistence

The following volumes are mounted for data persistence:

- `recipes_data`: PostgreSQL data directory
- `recipes_uploads`: User uploaded files
- `recipes_logs`: Application logs

### Volume Storage Locations

**Named Volumes (default)**:
- **Linux/Mac**: `/var/lib/docker/volumes/recipes_*`
- **Windows**: `C:\ProgramData\Docker\volumes\`

**Host Path Mounts (alternative)**:
Use `docker-compose.prod.host-mounts.yml` for explicit host paths:
- Database: `./data/postgres/`
- Uploads: `./data/uploads/`
- Logs: `./data/logs/`

### Volume Management

```bash
# List all volumes
docker volume ls | grep recipes

# Inspect volume location
docker volume inspect recipes_data

# Backup named volumes
docker run --rm -v recipes_data:/source -v $(pwd):/backup alpine tar czf /backup/recipes_data_backup.tar.gz -C /source .

# Restore to named volumes
docker run --rm -v recipes_data:/target -v $(pwd):/backup alpine tar xzf /backup/recipes_data_backup.tar.gz -C /target
```

## Health Checks

The container includes a health check that verifies:
- PostgreSQL is running and accepting connections
- Backend application is responding on port 8000

## Monitoring

### Logs

```bash
# View application logs
docker-compose -f docker-compose.prod.yml logs -f recipes

# View PostgreSQL logs
docker exec recipes_app tail -f /app/var/log/postgresql.log
```

### Database Access

```bash
# Connect to the database
docker exec -it recipes_app psql -U postgres -d postgres
```

## Performance Optimization

### Production Build

The Dockerfile uses production optimizations:
- Frontend built with `--release` flag
- Backend compiled in release mode
- Minimal Debian base image for runtime

### Resource Requirements

Minimum recommended resources:
- **CPU**: 1 core
- **Memory**: 1GB RAM
- **Storage**: 10GB (for data and uploads)

## Backup and Recovery

### Database Backup

```bash
# Create a backup
docker exec recipes_app pg_dump -U postgres postgres > backup.sql

# Restore from backup
docker exec -i recipes_app psql -U postgres postgres < backup.sql
```

### File Backup

```bash
# Backup uploads
docker cp recipes_app:/app/uploads ./uploads-backup
```

## Troubleshooting

### Common Issues

1. **Container fails to start**
   - Check JWT_SECRET is set
   - Verify port 8000 is not in use
   - Review logs: `docker-compose logs`

2. **Database connection errors**
   - Wait for PostgreSQL to initialize (may take 30+ seconds)
   - Check health check status: `docker ps`

3. **Frontend not loading**
   - Verify frontend was built during Docker build
   - Check static files in container: `docker exec ls /app/frontend/dist`

### Debug Mode

For debugging, you can run the container with elevated logging:

```bash
docker run -it --rm \
  -e RUST_LOG=debug \
  -p 8000:8000 \
  recipes-app
```

## SSL/TLS

For production HTTPS deployment, consider:

1. **Reverse Proxy**: Use Nginx or Caddy as a reverse proxy
2. **Let's Encrypt**: Automated SSL certificates
3. **Cloud Load Balancer**: Cloud provider SSL termination

Example Nginx configuration:

```nginx
server {
    listen 443 ssl;
    server_name yourdomain.com;
    
    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;
    
    location / {
        proxy_pass http://localhost:8000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## Scaling

For horizontal scaling:

1. **External Database**: Move PostgreSQL to external service
2. **Load Balancer**: Distribute traffic across multiple containers
3. **Shared Storage**: Use network storage for uploads

## Updates

To update the application:

```bash
# Pull latest code
git pull

# Rebuild and restart
docker-compose -f docker-compose.prod.yml up --build -d
```

The database data and uploads will persist across updates due to Docker volumes.
