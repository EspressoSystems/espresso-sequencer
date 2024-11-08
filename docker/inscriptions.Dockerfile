FROM rust:1.82.0 AS builder
COPY builder /app/builder
COPY client /app/client
COPY common /app/common
COPY contract-bindings /app/contract-bindings
COPY contracts /app/contracts
COPY data /app/data
COPY geth-config /app/geth-config
COPY hotshot-state-prover /app/hotshot-state-prover
COPY inscriptions /app/inscriptions
COPY marketplace-builder /app/marketplace-builder
COPY marketplace-solver /app/marketplace-solver
COPY node-metrics /app/node-metrics
COPY scripts /app/scripts
COPY sequencer /app/sequencer
COPY tests /app/tests
COPY types /app/types
COPY utils /app/utils
COPY zkevm-node-additions /app/zkevm-node-additions
COPY .env .envrc Cargo.toml Cargo.lock /app/
WORKDIR /app

RUN RUSTFLAGS="--cfg async_executor_impl=\"async-std\" --cfg async_channel_impl=\"async-std\"" cargo build  --release --all-targets --all-features

FROM ubuntu:jammy
ARG TARGETARCH

RUN apt-get update \
    &&  apt-get install -y curl libcurl4 wait-for-it tini \
    &&  rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["tini", "--"]

COPY --from=builder /app/target/release/inscriptions /bin/inscriptions
RUN chmod +x /bin/inscriptions

# Run a web server on this port by default. Port can be overridden by the container orchestrator.
ENV ESPRESSO_INSCRIPTIONS_PORT=80

CMD [ "/bin/inscriptions"]
HEALTHCHECK --interval=1s --timeout=1s --retries=100 CMD curl --fail http://localhost:${ESPRESSO_INSCRIPTIONS_PORT}/healthcheck  || exit 1
EXPOSE ${ESPRESSO_INSCRIPTIONS_PORT}
