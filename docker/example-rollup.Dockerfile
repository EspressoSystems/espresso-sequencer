FROM ubuntu:jammy

RUN apt-get update \
    &&  rm -rf /var/lib/apt/lists/*

#COPY target2/x86_64_-unknown-linux-musl/deploy-example-contracts /bin/deploy-example-contracts
COPY target-3 /bin/example-l2
RUN chmod +x /bin/example-l2

CMD [ "/bin/example-l2"]
