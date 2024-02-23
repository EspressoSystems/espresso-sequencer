FROM ubuntu:jammy

ARG TARGETARCH

RUN apt-get update \
    &&  apt-get install -y curl git libcurl4 wait-for-it tini jq \
    &&  rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["tini", "--"]

# install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
ENV PATH "$PATH:/root/.cargo/bin"

# install foundry toolchain
RUN curl -L https://foundry.paradigm.xyz | bash
ENV PATH "$PATH:/root/.foundry/bin"
RUN foundryup

# copy the contracts
RUN mkdir /work
WORKDIR /work
COPY foundry.toml /work/foundry.toml
COPY contracts/ /work/contracts/

# copy the binaries
COPY target/$TARGETARCH/release/state-prover /usr/local/bin/state-prover
COPY target/$TARGETARCH/release/gen-demo-contract /usr/local/bin/gen-demo-contract
COPY scripts/launch-prover-service /usr/local/bin/launch-prover-service
RUN chmod +x /usr/local/bin/state-prover
RUN chmod +x /usr/local/bin/launch-prover-service

# When running as a Docker service, we always want a healthcheck endpoint, so set a default for the
# port that the HTTP server will run on. This can be overridden in any given deployment environment.
ENV ESPRESSO_COMMITMENT_TASK_PORT=80
HEALTHCHECK --interval=1s --timeout=1s --retries=100 CMD curl --fail http://localhost:${ESPRESSO_COMMITMENT_TASK_PORT}/healthcheck || exit 1

CMD [ "launch-prover-service" ]
