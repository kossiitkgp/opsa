#!/bin/bash
set -e

echo "🔄 Running database migrations..."

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "❌ ERROR: DATABASE_URL environment variable is not set"
    exit 1
fi

# Check if migrations directory exists
if [ ! -d "migrations" ]; then
    echo "⚠️  WARNING: No migrations directory found"
    exit 0
fi

# Count SQL files
sql_files=$(find migrations -name "*.sql" | wc -l)
if [ "$sql_files" -eq 0 ]; then
    echo "⚠️  WARNING: No SQL migration files found in migrations/"
    exit 0
fi

echo "📁 Found $sql_files migration file(s)"

# Create migrations tracking table if it doesn't exist
echo "🗄️  Ensuring migrations tracking table exists..."
psql "$DATABASE_URL" -c "
CREATE TABLE IF NOT EXISTS _sqlx_migrations (
    version BIGINT PRIMARY KEY,
    description TEXT NOT NULL,
    installed_on TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    success BOOLEAN NOT NULL,
    checksum BYTEA NOT NULL,
    execution_time BIGINT NOT NULL
);"

# Run each migration file
for migration_file in migrations/*.sql; do
    if [ -f "$migration_file" ]; then
        filename=$(basename "$migration_file")
        echo "🚀 Running migration: $filename"
        
        # Extract version from filename (assuming format: YYYYMMDDHHMMSS_description.sql)
        version=$(echo "$filename" | grep -o '^[0-9]\+')
        
        # Check if migration was already applied
        already_applied=$(psql "$DATABASE_URL" -t -c "SELECT COUNT(*) FROM _sqlx_migrations WHERE version = $version;" | xargs)
        
        if [ "$already_applied" -eq 0 ]; then
            start_time=$(date +%s%N)
            
            # Run the migration
            if psql "$DATABASE_URL" -f "$migration_file"; then
                end_time=$(date +%s%N)
                execution_time=$(( (end_time - start_time) / 1000000 )) # Convert to milliseconds
                
                # Calculate checksum (simple approach)
                checksum=$(md5sum "$migration_file" | cut -d' ' -f1)
                description=$(echo "$filename" | sed 's/^[0-9]*_//' | sed 's/\.sql$//')
                
                # Record successful migration
                psql "$DATABASE_URL" -c "
INSERT INTO _sqlx_migrations (version, description, success, checksum, execution_time) 
VALUES ($version, '$description', true, decode('$checksum', 'hex'), $execution_time);"
                
                echo "✅ Migration $filename completed successfully (${execution_time}ms)"
            else
                echo "❌ Migration $filename failed"
                exit 1
            fi
        else
            echo "⏭️  Migration $filename already applied, skipping"
        fi
    fi
done

echo "🎉 All migrations completed successfully!"
