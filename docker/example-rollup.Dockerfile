FROM ubuntu:jammy

RUN apt-get update \
    &&  rm -rf /var/lib/apt/lists/*

COPY target/release/example-l2 /bin/example-l2
COPY target/release/cli /bin/cli
RUN chmod +x /bin/example-l2 /bin/cli

CMD [ "/bin/example-l2"]
