FROM ubuntu:jammy

ARG TARGETARCH

# install foundry toolchain
RUN apt-get update \
    &&  apt-get install -y curl git cargo\
    &&  rm -rf /var/lib/apt/lists/*
RUN curl -L https://foundry.paradigm.xyz | bash
RUN cat /root/.bashrc
ENV PATH "$PATH:/root/.foundry/bin"
RUN foundryup
RUN forge --version

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

WORKDIR /usr/local/prover-service
ENTRYPOINT ["/usr/local/prover-service/launch-prover-service"]
