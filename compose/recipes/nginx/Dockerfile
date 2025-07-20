FROM alpine:3 AS build

COPY build_nginx.sh /build_nginx.sh
COPY versions.env /versions.env

RUN chmod +x /build_nginx.sh && sh /build_nginx.sh

# Build the final image
FROM alpine:3

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    pcre2 \
    zlib \
    tzdata

# Create nginx user and group (use different IDs to avoid conflicts)
RUN addgroup -g 101 -S nginx && \
    adduser -u 101 -D -S -h /var/cache/nginx -s /sbin/nologin -G nginx nginx

# Copy nginx binary and configuration from build stage
COPY --from=build /usr/sbin/nginx /usr/sbin/nginx
COPY --from=build /etc/nginx /etc/nginx
COPY --from=build /usr/share/nginx/html /usr/share/nginx/html

# Create necessary directories
RUN mkdir -p /var/log/nginx && \
    mkdir -p /var/cache/nginx/client_temp && \
    mkdir -p /var/cache/nginx/proxy_temp && \
    mkdir -p /var/cache/nginx/fastcgi_temp && \
    mkdir -p /var/cache/nginx/uwsgi_temp && \
    mkdir -p /var/cache/nginx/scgi_temp && \
    chown -R nginx:nginx /var/cache/nginx && \
    chown -R nginx:nginx /var/log/nginx && \
    chmod -R 755 /var/cache/nginx && \
    chmod -R 755 /var/log/nginx

# Forward request and error logs to docker log collector
RUN ln -sf /dev/stdout /var/log/nginx/access.log && \
    ln -sf /dev/stderr /var/log/nginx/error.log

# Expose ports
EXPOSE 80 443

# Stop signal
STOPSIGNAL SIGQUIT

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost/ || exit 1

# Start nginx
CMD ["nginx", "-g", "daemon off;"]
