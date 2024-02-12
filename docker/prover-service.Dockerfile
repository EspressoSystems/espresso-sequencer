FROM ghcr.io/foundry-rs/foundry:latest

ARG TARGETARCH

# copy the contracts
RUN mkdir -p /usr/local/prover-service/contracts
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
