FROM ubuntu:jammy

RUN apt-get update \
    &&  rm -rf /var/lib/apt/lists/*

COPY target/x86_64_-unknown-linux-musl/example-rollup /bin/example-rollup
RUN chmod +x /bin/example-l2

CMD [ "/bin/example-l2"]
