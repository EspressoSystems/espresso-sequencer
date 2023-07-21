FROM ubuntu:jammy

ARG TARGETARCH

RUN apt-get update \
    &&  apt-get install -y curl wait-for-it \
    &&  rm -rf /var/lib/apt/lists/*

COPY target/$TARGETARCH/release/sequencer /bin/sequencer
RUN chmod +x /bin/sequencer

CMD ["/bin/sequencer", "--", "http"]
HEALTHCHECK --interval=1s --timeout=1s --retries=100 CMD curl --fail http://localhost:${ESPRESSO_SEQUENCER_API_PORT}/healthcheck  || exit 1
EXPOSE ${ESPRESSO_SEQUENCER_API_PORT}
