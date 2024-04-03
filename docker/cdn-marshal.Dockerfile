FROM ubuntu:jammy

ARG TARGETARCH

RUN apt-get update \
    &&  apt-get install -y curl libcurl4 wait-for-it tini \
    &&  rm -rf /var/lib/apt/lists/*

COPY target/$TARGETARCH/release/cdn-marshal /bin/cdn-marshal
RUN chmod +x /bin/cdn-marshal

ENV RUST_LOG="info"

ENTRYPOINT ["cdn-marshal"]