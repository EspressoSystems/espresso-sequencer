FROM ubuntu:jammy

ARG TARGETARCH

RUN apt-get update \
    &&  apt-get install -y curl libcurl4 wait-for-it tini \
    &&  rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["tini", "--"]

COPY target/$TARGETARCH/release/permissionless-builder /bin/permissionless-builder
RUN chmod +x /bin/permissionless-builder

ENV BUILDER_SERVER_PORT=60003

CMD [ "/bin/permissionless-builder"]
