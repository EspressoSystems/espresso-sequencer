FROM ubuntu:jammy

ARG TARGETARCH

RUN apt-get update \
    &&  apt-get install -y curl git libcurl4 wait-for-it tini jq \
    &&  rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["tini", "--"]

# copy the binaries
COPY target/$TARGETARCH/release/state-prover /usr/local/bin/state-prover
RUN chmod +x /usr/local/bin/state-prover

# When running as a Docker service, we always want a healthcheck endpoint, so set a default for the
# port that the HTTP server will run on. This can be overridden in any given deployment environment.
ENV ESPRESSO_COMMITMENT_TASK_PORT=80
HEALTHCHECK --interval=1s --timeout=1s --retries=100 CMD curl --fail http://localhost:${ESPRESSO_COMMITMENT_TASK_PORT}/healthcheck || exit 1

CMD [ "state-prover", "-d" ]
