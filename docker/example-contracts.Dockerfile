FROM ubuntu:jammy

RUN apt-get update \
    &&  rm -rf /var/lib/apt/lists/*

COPY target/release/deploy-example-contracts /bin/deploy-example-contracts
RUN chmod +x /bin/deploy-example-contracts

CMD [ "/bin/deploy-example-contracts"]
