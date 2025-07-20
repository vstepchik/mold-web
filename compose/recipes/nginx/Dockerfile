##################################################
# Nginx with Quiche (HTTP/3) and Brotli
# Credit: https://www.allsubjectsmatter.nl/docker-nginx-http3-and-brotli/
##################################################
# This build compiles Nginx with Brotli and
# HTTP/3 support. It uses a multi-stage build to
# minimize the final image size by copying only
# the necessary artifacts from the builder image.
# Based on Alpine for a lean and efficient image.
##################################################

# Builder Stage
FROM alpine:3 AS builder

# Set the versions of Nginx, NJS, and BoringSSL
ENV NGINX_VERSION=1.29.0
ENV NJS_VERSION=0.9.1
ENV BORINGSSL_VERSION=0.20250415.0
ENV BORINGSSL="/tmp/boring-nginx"

# Build-time metadata
ARG BUILD_DATE
ARG VCS_REF

# Install dependencies for building Nginx
RUN addgroup -S nginx \
    && adduser -D -S -h /var/cache/nginx -s /sbin/nologin -G nginx nginx \
    && apk update \
    && apk upgrade \
    && apk add --no-cache ca-certificates openssl \
    && update-ca-certificates \
    && apk add --no-cache --virtual .build-deps \
    gcc libc-dev make pcre-dev zlib-dev linux-headers gnupg \
    libxslt-dev gd-dev geoip-dev perl-dev \
    && apk add --no-cache --virtual .brotli-build-deps \
    autoconf libtool automake git g++ cmake go perl rust cargo patch \
    libxml2-dev byacc flex libstdc++ libmaxminddb-dev lmdb-dev file openrc

# Install and build BoringSSL
RUN mkdir $BORINGSSL \
    && cd $BORINGSSL \
    && git clone https://boringssl.googlesource.com/boringssl \
    && cd boringssl \
    && git checkout -q ${BORINGSSL_VERSION} \
    && mkdir build \
    && cd build \
    && cmake -DBUILD_SHARED_LIBS=1 .. \
    && make

# Prepare BoringSSL for Nginx compilation
RUN mkdir -p "$BORINGSSL/boringssl/.openssl/lib" \
    && cd "$BORINGSSL/boringssl/.openssl" \
    && ln -s ../include include \
    && cd "$BORINGSSL/boringssl" \
    && cp "build/crypto/libcrypto.so" ".openssl/lib" \
    && cp "build/ssl/libssl.so" ".openssl/lib" \
    && cp ".openssl/lib/libssl.so" /usr/lib/ \
    && cp ".openssl/lib/libcrypto.so" /usr/lib/

# Clone Nginx modules
RUN mkdir /usr/src \
    && cd /usr/src \
    && git clone --depth=1 --recursive --shallow-submodules https://github.com/google/ngx_brotli \
    && git clone --branch $NJS_VERSION --depth=1 --recursive --shallow-submodules https://github.com/nginx/njs

# Build Nginx with modules
RUN cd /usr/src \
    && wget -qO nginx.tar.gz https://nginx.org/download/nginx-$NGINX_VERSION.tar.gz \
    && tar -zxC /usr/src -f nginx.tar.gz \
    && rm nginx.tar.gz \
    && cd /usr/src/nginx-$NGINX_VERSION \
    && mkdir /root/.cargo \
    && echo $'[net]\ngit-fetch-with-cli = true' > /root/.cargo/config.toml \
    && ./configure --prefix=/etc/nginx \
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
    --with-pcre-jit \
    --with-http_ssl_module \
    --with-http_realip_module \
    --with-http_addition_module \
    --with-http_sub_module \
    --with-http_dav_module \
    --with-http_flv_module \
    --with-http_mp4_module \
    --with-http_gunzip_module \
    --with-http_gzip_static_module \
    --with-http_random_index_module \
    --with-http_secure_link_module \
    --with-http_stub_status_module \
    --with-http_auth_request_module \
    --with-http_xslt_module=dynamic \
    --with-http_image_filter_module=dynamic \
    --with-http_geoip_module=dynamic \
    --with-http_perl_module=dynamic \
    --with-threads \
    --with-stream \
    --with-stream_ssl_module \
    --with-stream_ssl_preread_module \
    --with-stream_realip_module \
    --with-stream_geoip_module=dynamic \
    --with-http_slice_module \
    --with-mail \
    --with-mail_ssl_module \
    --with-compat \
    --with-file-aio \
    --with-http_v2_module \
    --with-http_v3_module \
    --add-module=/usr/src/ngx_brotli \
    --add-module=/usr/src/njs/nginx \
    --with-cc-opt="-I$BORINGSSL/boringssl/include -Wno-error" \
    --with-ld-opt="-L$BORINGSSL/boringssl/build/ssl -L$BORINGSSL/boringssl/build/crypto" \
    --with-select_module \
    --with-poll_module \
    --build="docker-nginx-http3-$VCS_REF-$BUILD_DATE ngx_brotli-$(git --git-dir=/usr/src/ngx_brotli/.git rev-parse --short HEAD) njs-$(git --git-dir=/usr/src/njs/.git rev-parse --short HEAD)" \
    && touch "$BORINGSSL/boringssl/.openssl/include/openssl/ssl.h" \
    && make -j$(getconf _NPROCESSORS_ONLN) \
    && make install \
    && rm -rf /etc/nginx/html/ \
    && mkdir /etc/nginx/conf.d/ \
    && mkdir -p /usr/share/nginx/html/ \
    && install -m644 html/index.html /usr/share/nginx/html/ \
    && install -m644 html/50x.html /usr/share/nginx/html/ \
    && ln -s /usr/lib/nginx/modules /etc/nginx/modules \
    && strip /usr/sbin/nginx* \
    && strip /usr/lib/nginx/modules/*.so \
    && rm -rf /etc/nginx/*.default /etc/nginx/*.so \
    && rm -rf /usr/src \
    && apk add --no-cache --virtual .gettext gettext \
    && mv /usr/bin/envsubst /tmp/ \
    \
    # Determine runtime dependencies
    && runDeps="$( \
    scanelf --needed --nobanner /usr/sbin/nginx /usr/lib/nginx/modules/*.so /tmp/envsubst \
    | awk '{ gsub(/,/, "\nso:", $2); print "so:" $2 }' \
    | sort -u \
    | xargs -r apk info --installed \
    | sort -u \
    )" \
    && echo "$runDeps" > /nginx-run-deps.txt \
    \
    # Cleanup
    && apk del .brotli-build-deps .build-deps .gettext \
    && rm -rf /root/.cargo \
    && rm -rf /var/cache/apk/* \
    && mv /tmp/envsubst /usr/local/bin/ \
    && mkdir -p /etc/ssl/private \
    && openssl req -x509 -newkey rsa:4096 -nodes -keyout /etc/ssl/private/localhost.key -out /etc/ssl/localhost.pem -days 365 -sha256 -subj '/CN=localhost' \
    && ln -sf /dev/stdout /var/log/nginx/access.log \
    && ln -sf /dev/stderr /var/log/nginx/error.log

# Final Stage
FROM alpine:3

# Copy Nginx and related files from the builder stage
COPY --from=builder /usr/sbin/nginx /usr/sbin/nginx
COPY --from=builder /etc/nginx /etc/nginx
COPY --from=builder /usr/lib/nginx /usr/lib/nginx
COPY --from=builder /usr/share/nginx /usr/share/nginx
COPY --from=builder /usr/local/bin/envsubst /usr/local/bin/envsubst
COPY --from=builder /var/log/nginx /var/log/nginx
COPY --from=builder /var/cache/nginx /var/cache/nginx
COPY --from=builder /var/run /var/run
COPY --from=builder /etc/ssl /etc/ssl
COPY --from=builder /usr/lib/libssl.so* /usr/lib/
COPY --from=builder /usr/lib/libcrypto.so* /usr/lib/
COPY --from=builder /nginx-run-deps.txt /nginx-run-deps.txt

# Install runtime dependencies
RUN apk add --no-cache wget ca-certificates openssl libstdc++ libgcc \
    && apk add --no-cache $(cat /nginx-run-deps.txt) \
    && rm /nginx-run-deps.txt \
    && update-ca-certificates \
    \
    # Forward logs to Docker
    && ln -sf /dev/stdout /var/log/nginx/access.log \
    && ln -sf /dev/stderr /var/log/nginx/error.log

# Create nginx user and group
RUN addgroup -S nginx \
    && adduser -D -S -h /var/cache/nginx -s /sbin/nologin -G nginx nginx

# Expose ports
EXPOSE 80 443

# Stop signal
STOPSIGNAL SIGQUIT

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost/ || exit 1

CMD ["nginx", "-g", "daemon off;"]
