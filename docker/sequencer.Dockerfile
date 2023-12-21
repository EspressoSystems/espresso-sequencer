FROM ubuntu:jammy

ARG TARGETARCH

RUN apt-get update \
    &&  apt-get install -y curl libcurl4 wait-for-it tini \
    &&  rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["tini", "--"]

COPY target/$TARGETARCH/release/sequencer /bin/sequencer
RUN chmod +x /bin/sequencer

COPY target/$TARGETARCH/release/reset-storage /bin/reset-storage
RUN chmod +x /bin/reset-storage

# Set a path to save the consensus config on startup.
#
# Upon restart, the config will be loaded from this file and the node will be able to resume
# progress. The user should connect this path to a Docker volume to ensure persistence of the
# configuration beyond the lifetime of the Docker container itself.
ENV ESPRESSO_SEQUENCER_CONFIG_PATH=/config/hotshot.cfg

CMD ["/bin/sequencer", "--", "http"]
HEALTHCHECK --interval=1s --timeout=1s --retries=100 CMD curl --fail http://localhost:${ESPRESSO_SEQUENCER_API_PORT}/healthcheck  || exit 1
EXPOSE ${ESPRESSO_SEQUENCER_API_PORT}
