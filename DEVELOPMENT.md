# Development Setup

This document provides instructions for setting up the Recipes application for local development.

## Prerequisites

- Docker and Docker Compose
- Rust (latest stable version)
- PostgreSQL (if running locally without Docker)

## Quick Start with Docker

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd recipes
   ```

2. **Start the development environment**
   ```bash
   docker-compose up --build
   ```

   This will start:
   - PostgreSQL database on port 5432
   - Backend API server on port 8000

3. **Access the application**
   - Backend API: http://localhost:8000
   - Database: localhost:5432 (postgres/postgres)

## Local Development Setup

### Backend Setup

1. **Install dependencies**
   ```bash
   cd backend
   cargo build
   ```

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   ```
   
   Edit `.env` with your configuration:
   ```env
   DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
   RUST_LOG=info
   ROCKET_ADDRESS=0.0.0.0
   ROCKET_PORT=8000
   JWT_SECRET=your-super-secret-jwt-key-here
   ```

3. **Start PostgreSQL**
   ```bash
   # Using Docker
   docker run -d --name postgres-dev \
     -e POSTGRES_USER=postgres \
     -e POSTGRES_PASSWORD=postgres \
     -e POSTGRES_DB=postgres \
     -p 5432:5432 \
     postgres:12
   ```

4. **Run the backend**
   ```bash
   cd backend
   cargo run
   ```

### Frontend Setup (Yew)

1. **Install dependencies**
   ```bash
   cd frontend
   cargo build
   ```

2. **Install trunk for web development**
   ```bash
   cargo install trunk
   ```

3. **Build for development**
   ```bash
   trunk build
   ```

4. **Serve the frontend**
   ```bash
   trunk serve
   ```

## Development Commands

### Backend
```bash
# Run backend with hot reload (requires cargo-watch)
cargo install cargo-watch
cargo watch -x 'run --bin backend'

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run --bin backend
```

### Frontend
```bash
# Build and serve with hot reload
trunk serve

# Build for production
trunk build --release

# Clean build artifacts
trunk clean
```

### Database
```bash
# Start database with Docker
docker-compose up db

# Connect to database
docker exec -it recipes_db psql -U postgres -d postgres

# Reset database
docker-compose down -v
docker-compose up db
```

## Project Structure

```
recipes/
├── backend/                 # Rocket backend API
│   ├── src/
│   │   ├── main.rs         # Main application entry point
│   │   ├── auth.rs         # Authentication handlers
│   │   ├── db.rs           # Database operations
│   │   ├── handlers/       # API route handlers
│   │   └── models.rs       # Data models
│   ├── migrations/         # Database migrations
│   └── .env.example        # Environment variables template
├── frontend/               # Yew frontend application
│   ├── src/
│   │   ├── main.rs         # Frontend entry point
│   │   ├── components/    # Reusable components
│   │   └── pages/          # Page components
│   └── dist/              # Built frontend assets
├── shared-types/          # Shared types between frontend and backend
├── compose.yml            # Docker Compose configuration
├── Dockerfile.dev         # Development Dockerfile
└── DEVELOPMENT.md         # This file
```

## Environment Variables

### Backend (.env)
- `DATABASE_URL`: PostgreSQL connection string
- `RUST_LOG`: Logging level (debug, info, warn, error)
- `ROCKET_ADDRESS`: Server bind address
- `ROCKET_PORT`: Server port
- `JWT_SECRET`: Secret key for JWT tokens

## API Endpoints

The backend API runs on `http://localhost:8000` and includes:

### Authentication
- `POST /login` - User login
- `POST /logout` - User logout
- `GET /current_user` - Get current user
- `PUT /current_user` - Update current user

### Recipes
- `GET /recipes` - List recipes
- `POST /recipes` - Create recipe
- `GET /recipes/:id` - Get recipe
- `PUT /recipes/:id` - Update recipe
- `DELETE /recipes/:id` - Delete recipe

### Categories
- `GET /categories` - List categories
- `POST /categories` - Create category

## Troubleshooting

### Database Connection Issues
1. Ensure PostgreSQL is running
2. Check the `DATABASE_URL` in your `.env` file
3. Verify database exists and credentials are correct

### Build Issues
1. Clear cargo cache: `cargo clean`
2. Update dependencies: `cargo update`
3. Ensure Rust is up to date: `rustup update`

### Docker Issues
1. Rebuild containers: `docker-compose up --build`
2. Clear volumes: `docker-compose down -v`
3. Check logs: `docker-compose logs -f`

## Production Deployment

For production deployment, see `PRODUCTION.md` (when created) or use the production Dockerfile and configuration.
