FROM debian:jessie-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        curl \
        dnsutils \
        iptables \
        jq \
        nghttp2 \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY main /usr/local/bin

CMD ["main"]
