FROM ubuntu:jammy

ARG TARGETARCH

RUN apt-get update \
    &&  apt-get install -y curl git cargo libcurl4 wait-for-it tini jq \
    &&  rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["tini", "--"]

# install foundry toolchain
RUN curl -L https://foundry.paradigm.xyz | bash
ENV PATH "$PATH:/root/.foundry/bin"
RUN foundryup

# copy the contracts
WORKDIR /usr/local/prover-service/contracts
COPY foundry.toml /usr/local/prover-service/foundry.toml
COPY contracts/ /usr/local/prover-service/contracts/
# copy the binaries
COPY target/$TARGETARCH/release/state-prover /usr/local/prover-service/state-prover
COPY target/$TARGETARCH/release/gen-demo-contract /usr/local/prover-service/gen-demo-contract
RUN chmod +x /usr/local/prover-service/state-prover
RUN chmod +x /usr/local/prover-service/gen-demo-contract
# copy the launching script
COPY scripts/launch-prover-service /usr/local/prover-service/launch-prover-service
RUN chmod +x /usr/local/prover-service/launch-prover-service

# When running as a Docker service, we always want a healthcheck endpoint, so set a default for the
# port that the HTTP server will run on. This can be overridden in any given deployment environment.
ENV ESPRESSO_COMMITMENT_TASK_PORT=80
HEALTHCHECK --interval=1s --timeout=1s --retries=100 CMD curl --fail http://localhost:${ESPRESSO_COMMITMENT_TASK_PORT}/healthcheck || exit 1

WORKDIR /usr/local/prover-service
CMD ["/usr/local/prover-service/launch-prover-service"]
