# Test Production Deployment

This document explains how to test the OPSA production deployment locally using the `test-deployment.sh` script.

## Prerequisites

1. **Docker and Docker Compose** - Ensure Docker is running
2. **Environment File** - Create a `.env` file with required variables
3. **Slack Archive** (Optional) - Have a Slack export zip file ready

## Required Environment Variables

Create a `.env` file in the project root with the following variables:

```bash
# Database Configuration (Required)
TUMMY_USERNAME=postgres
TUMMY_PASSWORD=your_secure_password_here
TUMMY_PORT=5432

# Backend Configuration (Required)  
EXCRETOR_PORT=3000

# Optional: Slack Archive URL for automatic download
SLACK_ARCHIVE_URL=https://example.com/path/to/slack-archive.zip

# Optional: Other configuration
STATIC_ASSETS_DIR=assets/
SLACK_AUTH_ENABLE=false
```

## Usage

### Basic Test (No Slack Archive)

```bash
# Run the test deployment script
./test-deployment.sh
```

This will:
1. Build all Docker images
2. Start the database
3. Run migrations (if available)
4. Start all services
5. Perform health checks

### Test with Local Slack Archive

```bash
# Place your Slack archive in the project root
cp /path/to/your/slack-export.zip ./slack-archive.zip

# Run the test deployment
./test-deployment.sh
```

### Test with Remote Slack Archive

```bash
# Set the SLACK_ARCHIVE_URL in your .env file
echo "SLACK_ARCHIVE_URL=https://your-domain.com/slack-archive.zip" >> .env

# Run the test deployment
./test-deployment.sh
```

## What the Script Does

The `test-deployment.sh` script simulates the production deployment workflow by:

1. **Validating Prerequisites**
   - Checks if Docker is running
   - Validates `.env` file exists
   - Verifies required environment variables

2. **Building Docker Images**
   - `opsa-digester:latest` - Processes Slack archives
   - `opsa-excretor:latest` - Backend API server
   - `opsa-garnisher:latest` - Frontend web application

3. **Setting Up Deployment Directory**
   - Creates `./test-deployment/` directory
   - Copies necessary files (docker-compose.yml, .env, migrations, etc.)

4. **Handling Slack Archive**
   - Downloads from URL if `SLACK_ARCHIVE_URL` is set
   - Uses local `slack-archive.zip` if available
   - Skips if no archive is found

5. **Database Setup**
   - Starts PostgreSQL database (tummy service)
   - Runs database migrations using sqlx-cli
   - Waits for database to be healthy

6. **Processing Slack Archive**
   - Checks if database already has messages
   - Runs digester to process Slack archive if database is empty
   - Skips processing if database already contains data

7. **Starting Services**
   - Starts all services (tummy, excretor, garnisher)
   - Waits for services to initialize

8. **Health Checks**
   - Verifies all services are running
   - Shows container status
   - Displays service URLs and logs

## Service URLs

After successful deployment, services will be available at:

- **Frontend (Garnisher)**: http://localhost:3000
- **Backend (Excretor)**: http://localhost:${EXCRETOR_PORT}
- **Database (Tummy)**: localhost:${TUMMY_PORT}

## Useful Commands

While services are running:

```bash
# View logs for all services
cd test-deployment && docker compose logs -f

# View logs for specific service
cd test-deployment && docker compose logs -f excretor

# Check service status
cd test-deployment && docker compose ps

# Stop all services
cd test-deployment && docker compose down

# Stop services and remove volumes (clean slate)
cd test-deployment && docker compose down -v
```

## Troubleshooting

### Common Issues

1. **Port Conflicts**
   - Change ports in `.env` file if default ports are in use
   - Ensure no other services are using ports 3000, 5432, etc.

2. **Database Connection Issues**
   - Check if PostgreSQL container is running: `docker compose ps tummy`
   - View database logs: `docker compose logs tummy`

3. **Slack Archive Processing Fails**
   - Ensure archive file is a valid Slack export zip
   - Check digester logs: `docker compose logs digester`

4. **Frontend Can't Connect to Backend**
   - Verify excretor service is running
   - Check if nginx configuration is properly set up
   - View garnisher logs: `docker compose logs garnisher`

### Clean Start

To start fresh (removes all data):

```bash
cd test-deployment
docker compose down -v
docker system prune -f --volumes
cd .. && rm -rf test-deployment
./test-deployment.sh
```

## Differences from Production

This test script simulates production deployment but runs locally:

- Uses local Docker instead of remote server
- Builds images locally instead of transferring them
- Uses local file system instead of SSH transfers
- Skips some production-specific steps (SSH setup, webhooks, etc.)

The core deployment logic and service configuration remain identical to production.
