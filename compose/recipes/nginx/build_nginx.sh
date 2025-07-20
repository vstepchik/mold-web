#!/bin/sh

set -e

# Source the versions file
. /versions.env

# Install necessary packages for Alpine Linux v3
apk add --no-cache \
    build-base \
    cmake \
    git \
    go \
    linux-headers \
    pcre2-dev \
    zlib-dev \
    perl

# Create build directory
mkdir -p /build
cd /build

# Download and extract Nginx source code
echo "Downloading Nginx ${NGINX_VERSION}..."
wget https://nginx.org/download/nginx-${NGINX_VERSION}.tar.gz
tar -xzf nginx-${NGINX_VERSION}.tar.gz

# Clone BoringSSL
echo "Cloning BoringSSL ${BORINGSSL_VERSION}..."
git clone --depth 1 --branch ${BORINGSSL_VERSION} https://github.com/google/boringssl.git

# Build BoringSSL
echo "Building BoringSSL..."
cd boringssl
mkdir -p build
cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
make -j$(nproc)
cd ..

# Set BoringSSL paths
mkdir -p .openssl/lib
cp build/crypto/libcrypto.a .openssl/lib/
cp build/ssl/libssl.a .openssl/lib/
mkdir -p .openssl/include
cp -r include/* .openssl/include/

cd /build

# Clone Brotli
echo "Cloning Brotli..."
git clone https://github.com/google/ngx_brotli.git
cd ngx_brotli
git checkout ${BROTLI_COMMIT}
git submodule update --init

cd /build

# Configure and build Nginx
echo "Configuring Nginx..."
cd nginx-${NGINX_VERSION}

./configure \
    --prefix=/etc/nginx \
    --sbin-path=/usr/sbin/nginx \
    --modules-path=/usr/lib/nginx/modules \
    --conf-path=/etc/nginx/nginx.conf \
    --error-log-path=/var/log/nginx/error.log \
    --http-log-path=/var/log/nginx/access.log \
    --pid-path=/var/run/nginx.pid \
    --lock-path=/var/run/nginx.lock \
    --http-client-body-temp-path=/var/cache/nginx/client_temp \
    --http-proxy-temp-path=/var/cache/nginx/proxy_temp \
    --http-fastcgi-temp-path=/var/cache/nginx/fastcgi_temp \
    --http-uwsgi-temp-path=/var/cache/nginx/uwsgi_temp \
    --http-scgi-temp-path=/var/cache/nginx/scgi_temp \
    --user=nginx \
    --group=nginx \
    --with-compat \
    --with-file-aio \
    --with-threads \
    --with-http_addition_module \
    --with-http_auth_request_module \
    --with-http_dav_module \
    --with-http_flv_module \
    --with-http_gunzip_module \
    --with-http_gzip_static_module \
    --with-http_mp4_module \
    --with-http_random_index_module \
    --with-http_realip_module \
    --with-http_secure_link_module \
    --with-http_slice_module \
    --with-http_ssl_module \
    --with-http_stub_status_module \
    --with-http_sub_module \
    --with-http_v2_module \
    --with-http_v3_module \
    --with-stream \
    --with-stream_realip_module \
    --with-stream_ssl_module \
    --with-stream_ssl_preread_module \
    --with-cc-opt="-g -O2 -fPIE -fstack-protector-strong -Wformat -Werror=format-security -Wdate-time -D_FORTIFY_SOURCE=2" \
    --with-ld-opt="-Wl,-Bsymbolic-functions -fPIE -pie -Wl,-z,relro -Wl,-z,now" \
    --with-openssl=/build/boringssl \
    --add-module=/build/ngx_brotli

echo "Building Nginx..."
make -j$(nproc)

echo "Installing Nginx..."
make install

# Create nginx user and group
addgroup -g 101 -S nginx
adduser -u 101 -D -S -h /var/cache/nginx -s /sbin/nologin -G nginx nginx

# Create necessary directories
mkdir -p /var/cache/nginx/client_temp
mkdir -p /var/cache/nginx/proxy_temp
mkdir -p /var/cache/nginx/fastcgi_temp
mkdir -p /var/cache/nginx/uwsgi_temp
mkdir -p /var/cache/nginx/scgi_temp

# Set proper permissions
chown -R nginx:nginx /var/cache/nginx
chown -R nginx:nginx /var/log/nginx

# Create a simple nginx.conf if it doesn't exist
if [ ! -f /etc/nginx/nginx.conf ]; then
cat > /etc/nginx/nginx.conf << 'EOF'
user nginx;
worker_processes auto;
error_log /var/log/nginx/error.log warn;
pid /var/run/nginx.pid;

events {
    worker_connections 1024;
}

http {
    include /etc/nginx/mime.types;
    default_type application/octet-stream;

    log_format main '$remote_addr - $remote_user [$time_local] "$request" '
                    '$status $body_bytes_sent "$http_referer" '
                    '"$http_user_agent" "$http_x_forwarded_for"';

    access_log /var/log/nginx/access.log main;

    sendfile on;
    tcp_nopush on;
    tcp_nodelay on;
    keepalive_timeout 65;
    types_hash_max_size 2048;

    gzip on;
    brotli on;
    brotli_comp_level 4;
    brotli_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript;

    server {
        listen 80;
        listen [::]:80;
        server_name localhost;

        location / {
            root /usr/share/nginx/html;
            index index.html index.htm;
        }

        error_page 500 502 503 504 /50x.html;
        location = /50x.html {
            root /usr/share/nginx/html;
        }
    }
}
EOF
fi

# Create default index.html
mkdir -p /usr/share/nginx/html
cat > /usr/share/nginx/html/index.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <title>Welcome to nginx!</title>
</head>
<body>
    <h1>Welcome to nginx!</h1>
    <p>If you see this page, the nginx web server is successfully installed and working.</p>
    <p>Built with BoringSSL and Brotli support.</p>
</body>
</html>
EOF

echo "Nginx build completed successfully!"
