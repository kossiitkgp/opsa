#!/bin/sh

# Substitute environment variables in nginx configuration
envsubst '${EXCRETOR_HOST} ${EXCRETOR_PORT}' < /etc/nginx/conf.d/default.conf.template > /etc/nginx/conf.d/default.conf

# Start nginx
exec nginx -g 'daemon off;'
