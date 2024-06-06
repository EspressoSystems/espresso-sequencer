FROM postgres

ARG TARGETARCH

RUN apt-get update \
    &&  apt-get install -y curl libcurl4 wait-for-it tini \
    &&  rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["tini", "--"]

# Download an SRS file to avoid download at runtime
ENV AZTEC_SRS_PATH=/kzg10-aztec20-srs-1048584.bin
RUN curl -LO https://github.com/EspressoSystems/ark-srs/releases/download/v0.2.0/$AZTEC_SRS_PATH

COPY target/$TARGETARCH/release/espresso-dev-node /bin/espresso-dev-node
RUN chmod +x /bin/espresso-dev-node

COPY target/$TARGETARCH/release/anvil /bin/anvil
RUN chmod +x /bin/anvil

COPY launch-dev-node-with-postgres /bin/launch-dev-node-with-postgres
RUN chmod +x /bin/launch-dev-node-with-postgres

# When running as a Docker service, we always want a healthcheck endpoint, so set a default for the
# port that the HTTP server will run on. This can be overridden in any given deployment environment.
ENV ESPRESSO_SEQUENCER_API_PORT=8770
HEALTHCHECK --interval=1s --timeout=1s --retries=100 CMD curl --fail http://localhost:${ESPRESSO_SEQUENCER_API_PORT}/status/block-height || exit 1

EXPOSE 8770
EXPOSE 8771
EXPOSE 8772

CMD [ "/bin/launch-dev-node-with-postgres"]
