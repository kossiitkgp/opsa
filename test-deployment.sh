#!/bin/bash

# Test Production Deployment Script
# This script simulates the production deployment workflow locally
# Based on the deploy-homelab.yaml GitHub Actions workflow

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DEPLOYMENT_DIR="./test-deployment"
REQUIRED_ENV_VARS="TUMMY_USERNAME TUMMY_PASSWORD TUMMY_PORT EXCRETOR_PORT"

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

cleanup() {
    log_info "Cleaning up..."
    rm -rf "$DEPLOYMENT_DIR"
    docker compose down --remove-orphans 2>/dev/null || true
    docker system prune -f --volumes 2>/dev/null || true
    log_success "Cleanup completed"
}

# Trap cleanup on script exit
trap cleanup EXIT

main() {
    log_info "🚀 Starting OPSA Production Deployment Test"
    echo "================================================="
    
    # Step 1: Validate prerequisites
    log_info "Step 1: Validating prerequisites..."
    
    # Check if Docker is running
    if ! docker info >/dev/null 2>&1; then
        log_error "Docker is not running. Please start Docker and try again."
        exit 1
    fi
    
    # Check if .env file exists
    if [ ! -f .env ]; then
        log_error ".env file not found. Please create one with required environment variables."
        echo "Required variables: $REQUIRED_ENV_VARS"
        exit 1
    fi
    
    # Validate environment variables
    log_info "Validating environment configuration..."
    source .env
    for var in $REQUIRED_ENV_VARS; do
        if [ -z "${!var}" ]; then
            log_error "Required environment variable $var not found in .env"
            exit 1
        fi
    done
    log_success "Environment validation passed"
    
    # Step 2: Create deployment directory
    log_info "Step 2: Creating deployment directory..."
    mkdir -p "$DEPLOYMENT_DIR"
    log_success "Deployment directory created: $DEPLOYMENT_DIR"
    
    # Step 3: Build Docker images
    log_info "Step 3: Building Docker images..."
    
    log_info "Building digester service..."
    docker build -t opsa-digester:latest ./digester
    
    log_info "Building excretor service..."
    docker build -f excretor/Dockerfile -t opsa-excretor:latest .
    
    log_info "Building garnisher service..."
    docker build -t opsa-garnisher:latest ./garnisher
    
    log_info "Building migrations service (for debugging/manual use)..."
    docker build -f migrations.Dockerfile -t opsa-migrations:latest .
    
    log_success "All Docker images built successfully"
    
    # Step 4: Copy necessary files to deployment directory
    log_info "Step 4: Copying deployment files..."
    cp docker-compose.yml "$DEPLOYMENT_DIR/"
    cp .env "$DEPLOYMENT_DIR/"
    
    # Copy migrations directory if it exists
    if [ -d "migrations" ]; then
        cp -r migrations/ "$DEPLOYMENT_DIR/"
        log_info "Migrations directory copied"
    else
        log_warning "No migrations directory found"
    fi
    
    # Copy tummy directory if it exists
    if [ -d "tummy" ]; then
        mkdir -p "$DEPLOYMENT_DIR/tummy"
        if [ -f "tummy/init.sql" ]; then
            cp "tummy/init.sql" "$DEPLOYMENT_DIR/tummy/"
            log_info "Tummy directory and init.sql copied"
        else
            log_warning "tummy/init.sql file not found"
        fi
    else
        log_warning "No tummy directory found"
    fi
    
    # Add ZIPFILE_PATH to .env if not present to avoid warnings
    if ! grep -q "ZIPFILE_PATH=" "$DEPLOYMENT_DIR/.env"; then
        echo "ZIPFILE_PATH=" >> "$DEPLOYMENT_DIR/.env"
    fi
    
    log_success "Deployment files copied"
    
    # Step 5: Handle Slack archive (optional)
    log_info "Step 5: Checking for Slack archive..."
    if [ -n "${SLACK_ARCHIVE_URL:-}" ]; then
        log_info "Downloading Slack archive from: $SLACK_ARCHIVE_URL"
        curl -L -o "$DEPLOYMENT_DIR/slack-archive.zip" "$SLACK_ARCHIVE_URL"
        if [ -f "$DEPLOYMENT_DIR/slack-archive.zip" ]; then
            log_success "Slack archive downloaded successfully"
        else
            log_error "Failed to download Slack archive"
            exit 1
        fi
    elif [ -f "slack-archive.zip" ]; then
        log_info "Using local slack-archive.zip"
        cp slack-archive.zip "$DEPLOYMENT_DIR/"
        log_success "Local Slack archive copied"
    else
        log_warning "No Slack archive found. Deployment will proceed with empty database."
    fi
    
    # Step 6: Start deployment
    log_info "Step 6: Starting deployment..."
    cd "$DEPLOYMENT_DIR"
    
    # Stop any existing services and clean volumes for fresh start
    log_info "Stopping existing services and cleaning volumes..."
    docker compose down --remove-orphans -v 2>/dev/null || true
    
    # Start database first and wait for it to be healthy
    log_info "Starting database service..."
    docker compose up tummy -d --wait
    
    # Step 7: Initialize database schema and run migrations
    log_info "Step 7: Initializing database schema and running migrations..."
    
    # First, check if the database schema is already initialized
    schema_exists=$(docker run --rm --network digestive-tract \
        -e PGPASSWORD="$(grep TUMMY_PASSWORD .env | cut -d= -f2)" \
        postgres:latest psql -h tummy -U "$(grep TUMMY_USERNAME .env | cut -d= -f2)" -d tummy -t -c "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'messages');" 2>/dev/null | xargs || echo "f")
    
    if [ "$schema_exists" = "f" ]; then
        log_info "Database schema not found. Initializing with base schema..."
        # The base schema should be automatically loaded by the tummy container via init.sql
        # Let's wait a bit more for the initialization to complete
        sleep 5
        
        # Check again if schema exists
        schema_exists=$(docker run --rm --network digestive-tract \
            -e PGPASSWORD="$(grep TUMMY_PASSWORD .env | cut -d= -f2)" \
            postgres:latest psql -h tummy -U "$(grep TUMMY_USERNAME .env | cut -d= -f2)" -d tummy -t -c "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'messages');" 2>/dev/null | xargs || echo "f")
        
        if [ "$schema_exists" = "f" ]; then
            log_warning "Database schema not found. Attempting manual initialization..."
            if [ -f "tummy/init.sql" ]; then
                log_info "Running init.sql manually..."
                docker run --rm --network digestive-tract \
                    -v "$(pwd)/tummy/init.sql:/tmp/init.sql" \
                    -e PGPASSWORD="$(grep TUMMY_PASSWORD .env | cut -d= -f2)" \
                    postgres:latest psql -h tummy -U "$(grep TUMMY_USERNAME .env | cut -d= -f2)" -d tummy -f /tmp/init.sql
                
                # Check again if schema exists after manual initialization
                schema_exists=$(docker run --rm --network digestive-tract \
                    -e PGPASSWORD="$(grep TUMMY_PASSWORD .env | cut -d= -f2)" \
                    postgres:latest psql -h tummy -U "$(grep TUMMY_USERNAME .env | cut -d= -f2)" -d tummy -t -c "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'messages');" 2>/dev/null | xargs || echo "f")
                
                if [ "$schema_exists" = "t" ]; then
                    log_success "Database schema initialized successfully via manual init"
                else
                    log_error "Manual database initialization failed"
                    exit 1
                fi
            else
                log_error "tummy/init.sql not found. Database cannot be initialized."
                exit 1
            fi
        else
            log_success "Database schema already exists"
        fi
    else
        log_info "Database schema already exists"
    fi
    
    # Clean up any conflicting migration records from previous runs
    log_info "🧹 Cleaning up any conflicting migration records..."
    docker run --rm --network digestive-tract \
        -e PGPASSWORD="$(grep TUMMY_PASSWORD .env | cut -d= -f2)" \
        postgres:latest psql -h tummy -U "$(grep TUMMY_USERNAME .env | cut -d= -f2)" -d tummy -c \
        "DROP TABLE IF EXISTS _sqlx_migrations;" 2>/dev/null || true
    
    # Note: Migrations are handled by the excretor service on startup
    # This avoids checksum mismatches between separate migration runs
    log_info "Database migrations will be handled by excretor service on startup"
    
    # Step 8: Process Slack archive if available
    log_info "Step 8: Processing Slack archive..."
    if [ -f "slack-archive.zip" ]; then
        log_info "Processing Slack archive using digester..."
        
        # Check if database already has messages (avoid reprocessing)
        message_count=$(docker run --rm --network digestive-tract \
            -e PGPASSWORD="$(grep TUMMY_PASSWORD .env | cut -d= -f2)" \
            postgres:latest psql -h tummy -U "$(grep TUMMY_USERNAME .env | cut -d= -f2)" -d tummy -t -c "SELECT COUNT(*) FROM messages;" 2>/dev/null | xargs | tr -d ' ' || echo "0")
        
        # Ensure message_count is a valid number
        if ! [[ "$message_count" =~ ^[0-9]+$ ]]; then
            message_count=0
        fi
        
        if [ "$message_count" -gt "0" ]; then
            log_warning "Database already contains $message_count messages. Skipping archive processing."
            log_info "To reprocess the archive, clean the database first with: docker compose down -v"
        else
            log_info "Database is empty. Processing Slack archive..."
            docker run --rm --network digestive-tract \
                -v "$(pwd)/slack-archive.zip:/app/slack-archive.zip" \
                -e ZIPFILE_PATH="/app/slack-archive.zip" \
                -e TUMMY_USERNAME="$(grep TUMMY_USERNAME .env | cut -d= -f2)" \
                -e TUMMY_PASSWORD="$(grep TUMMY_PASSWORD .env | cut -d= -f2)" \
                -e TUMMY_PORT="$(grep TUMMY_PORT .env | cut -d= -f2)" \
                -e TUMMY_HOST="tummy" \
                -e TUMMY_DB="tummy" \
                opsa-digester:latest
            log_success "Slack archive processing completed!"
        fi
    else
        log_warning "No Slack archive found. Application will start with empty database."
    fi
    
    # Step 9: Start all services (excluding tummy-dev and digester)
    # Note: digester is a one-time job, not a persistent service
    # Note: tummy-dev is for local development only
    log_info "Step 9: Starting all services..."
    docker compose up -d tummy excretor garnisher
    
    # Wait a bit for services to start
    log_info "Waiting for services to start..."
    sleep 10
    
    # Step 10: Health checks
    log_info "Step 10: Performing health checks..."
    services="tummy excretor garnisher"
    all_healthy=true
    
    for service in $services; do
        if docker compose ps "$service" | grep -q "Up"; then
            log_success "✅ $service is running"
        else
            log_error "❌ $service failed to start"
            echo "Logs for $service:"
            docker compose logs --tail=20 "$service"
            all_healthy=false
        fi
    done
    
    if [ "$all_healthy" = true ]; then
        log_success "✅ All services are running successfully"
    else
        log_error "❌ Some services failed to start"
        exit 1
    fi
    
    # Step 11: Show running containers and useful information
    log_info "Step 11: Deployment summary"
    echo "================================================="
    log_success "🎉 OPSA deployment test completed successfully!"
    echo ""
    log_info "Running containers:"
    docker compose ps
    echo ""
    log_info "Service URLs:"
    echo "  • Frontend (Garnisher): http://localhost:3000"
    echo "  • Backend (Excretor):   http://localhost:$(grep EXCRETOR_PORT .env | cut -d= -f2)"
    echo "  • Database (Tummy):     localhost:$(grep TUMMY_PORT .env | cut -d= -f2)"
    echo ""
    log_info "Useful commands:"
    echo "  • View logs:           cd $DEPLOYMENT_DIR && docker compose logs -f [service]"
    echo "  • Stop services:       cd $DEPLOYMENT_DIR && docker compose down"
    echo "  • Clean everything:    cd $DEPLOYMENT_DIR && docker compose down -v"
    echo ""
    log_warning "Note: This is a test deployment. Clean up when done testing."
    
    # Optional: Keep services running
    read -p "Keep services running for testing? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        log_info "Services will keep running. Use 'docker compose down' in $DEPLOYMENT_DIR to stop them."
        # Don't run cleanup on exit
        trap - EXIT
    else
        log_info "Stopping services..."
        docker compose down
    fi
}

# Check if script is being sourced or executed
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
