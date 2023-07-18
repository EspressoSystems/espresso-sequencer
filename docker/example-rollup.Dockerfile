FROM ubuntu:jammy

ARG TARGETARCH

RUN apt-get update \
    &&  rm -rf /var/lib/apt/lists/*

COPY target/$TARGETARCH/release/example-l2 /bin/example-l2
COPY target/$TARGETARCH/release/cli /bin/cli
RUN chmod +x /bin/example-l2 /bin/cli

CMD [ "/bin/example-l2"]
