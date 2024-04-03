FROM ubuntu:jammy

ARG TARGETARCH

RUN apt-get update \
    &&  apt-get install -y curl libcurl4 wait-for-it tini \
    &&  rm -rf /var/lib/apt/lists/*

COPY target/$TARGETARCH/release/cdn-broker /bin/cdn-broker
RUN chmod +x /bin/cdn-broker

ENV RUST_LOG="info"

ENTRYPOINT ["cdn-broker"]