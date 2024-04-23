FROM ubuntu:jammy

ARG TARGETARCH

RUN apt-get update \
    &&  apt-get install -y curl libcurl4 wait-for-it tini \
    &&  rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["tini", "--"]

COPY target/$TARGETARCH/release/test-node /bin/test-node
RUN chmod +x /bin/test-node

# When running as a Docker service, we always want a healthcheck endpoint, so set a default for the
# port that the HTTP server will run on. This can be overridden in any given deployment environment.
ENV ESPRESSO_SEQUENCER_API_PORT=40001
HEALTHCHECK --interval=1s --timeout=1s --retries=100 CMD curl --fail http://localhost:${ESPRESSO_SEQUENCER_API_PORT}/status/block-height || exit 1

CMD [ "/bin/test-node"]
