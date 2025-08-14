# Dedicated Dockerfile for running database migrations
FROM postgres:15-alpine

WORKDIR /app

# Install bash and other utilities
RUN apk add --no-cache bash coreutils

# Copy migrations directory and migration script
COPY migrations ./migrations
COPY run-migrations.sh /usr/local/bin/run-migrations.sh
RUN chmod +x /usr/local/bin/run-migrations.sh

# Set the default command to run migrations
CMD ["run-migrations.sh"]
