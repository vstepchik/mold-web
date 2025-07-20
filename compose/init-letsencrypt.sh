#!/bin/bash

# Initialize Let's Encrypt certificates for nginx
# This script should be run once before starting the services

set -e

# Configuration
DOMAIN="your-domain.com"  # Replace with your actual domain
EMAIL="your-email@example.com"  # Replace with your email
STAGING=0  # Set to 1 for staging certificates (for testing)

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting Let's Encrypt certificate initialization...${NC}"

# Check if domain and email are still default values
if [ "$DOMAIN" = "your-domain.com" ] || [ "$EMAIL" = "your-email@example.com" ]; then
    echo -e "${RED}Error: Please update DOMAIN and EMAIL variables in this script${NC}"
    echo "Edit the script and set:"
    echo "  DOMAIN=\"yourdomain.com\""
    echo "  EMAIL=\"youremail@example.com\""
    exit 1
fi

# Check if certificates already exist
if [ -d "./certbot/live/$DOMAIN" ]; then
    echo -e "${YELLOW}Certificate for $DOMAIN already exists${NC}"
    echo "Remove ./certbot/live/$DOMAIN to regenerate"
    exit 0
fi

# Create directory structure
mkdir -p ./certbot/www

echo -e "${GREEN}Creating dummy certificate for $DOMAIN...${NC}"
mkdir -p ./certbot/live/$DOMAIN
docker run --rm -v "$(pwd)/certbot:/etc/letsencrypt" \
    --entrypoint openssl certbot/certbot \
    req -x509 -nodes -newkey rsa:4096 \
    -keyout "/etc/letsencrypt/live/$DOMAIN/privkey.pem" \
    -out "/etc/letsencrypt/live/$DOMAIN/fullchain.pem" \
    -subj "/CN=$DOMAIN" -days 1

echo -e "${GREEN}Starting nginx with dummy certificate...${NC}"
docker compose up -d nginx

echo -e "${GREEN}Removing dummy certificate...${NC}"
docker run --rm -v "$(pwd)/certbot:/etc/letsencrypt" \
    --entrypoint rm certbot/certbot \
    -rf /etc/letsencrypt/live/$DOMAIN

echo -e "${GREEN}Requesting Let's Encrypt certificate for $DOMAIN...${NC}"

# Set staging flag if needed
if [ $STAGING = 1 ]; then
    STAGING_ARG="--staging"
    echo -e "${YELLOW}Using staging server (test certificates)${NC}"
else
    STAGING_ARG=""
    echo -e "${GREEN}Using production server (real certificates)${NC}"
fi

# Request certificate
docker run --rm \
    -v "$(pwd)/certbot:/etc/letsencrypt" \
    -v "$(pwd)/certbot/www:/var/www/certbot" \
    --network compose_mold-net \
    certbot/certbot \
    certonly \
    --webroot \
    --webroot-path=/var/www/certbot \
    --email $EMAIL \
    --agree-tos \
    --no-eff-email \
    $STAGING_ARG \
    -d $DOMAIN

echo -e "${GREEN}Certificate obtained successfully!${NC}"
echo -e "${GREEN}Restarting nginx to use new certificate...${NC}"
docker compose restart nginx

echo -e "${GREEN}Certificate initialization complete!${NC}"
echo "Your certificates are located in ./certbot/live/$DOMAIN/"
echo "Certificates will auto-renew every 12 hours via the certbot service"

# Display certificate info
echo -e "\n${GREEN}Certificate information:${NC}"
docker run --rm -v "$(pwd)/certbot:/etc/letsencrypt" \
    --entrypoint openssl certbot/certbot \
    x509 -in "/etc/letsencrypt/live/$DOMAIN/fullchain.pem" \
    -text -noout | grep -E "(Subject:|Issuer:|Not Before:|Not After:)"
