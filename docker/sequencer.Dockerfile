FROM ubuntu:jammy

ARG TARGETARCH

RUN apt-get update \
    &&  apt-get install -y curl libcurl4 wait-for-it tini \
    &&  rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["tini", "--"]

# Download an SRS file to avoid download at runtime
ENV AZTEC_SRS_PATH=/kzg10-aztec20-srs-1048584.bin
RUN curl -LO https://github.com/EspressoSystems/ark-srs/releases/download/v0.2.0/$AZTEC_SRS_PATH

COPY target/$TARGETARCH/release/sequencer /bin/sequencer
RUN chmod +x /bin/sequencer

COPY target/$TARGETARCH/release/reset-storage /bin/reset-storage
RUN chmod +x /bin/reset-storage

COPY target/$TARGETARCH/release/keygen /bin/keygen
RUN chmod +x /bin/keygen

COPY target/$TARGETARCH/release/pub-key /bin/pub-key
RUN chmod +x /bin/pub-key

# Install genesis files for all supported configurations. The desired configuration can be chosen by
# setting `ESPRESSO_SEQUENCER_GENESIS_FILE`.
COPY data/genesis /genesis

# Allow injecting a genesis file with aws secretsmanager
# Set `ESPRESSO_SEQUENCER_GENESIS_SECRET`
COPY docker/scripts/sequencer-awssecretsmanager.sh /bin/sequencer-awssecretsmanager.sh

# Set a path to save the consensus config on startup.
#
# Upon restart, the config will be loaded from this file and the node will be able to resume
# progress. The user should connect this path to a Docker volume to ensure persistence of the
# configuration beyond the lifetime of the Docker container itself.
ENV ESPRESSO_SEQUENCER_STORAGE_PATH=/store/sequencer

# We run the additional `status` and `catchup` modules by default. These are modules that require
# minimal resources (no persistent storage) but improve the functionality of the network.
CMD ["/bin/sequencer", "--", "http", "--", "status", "--", "catchup"]
HEALTHCHECK --interval=1s --timeout=1s --retries=100 CMD curl --fail http://localhost:${ESPRESSO_SEQUENCER_API_PORT}/healthcheck  || exit 1
EXPOSE ${ESPRESSO_SEQUENCER_API_PORT}
