FROM ubuntu:jammy

RUN apt-get update \
    &&  rm -rf /var/lib/apt/lists/*

COPY target/x86_64-unknown-linux-musl/release/example-l2 /bin/example-l2
RUN chmod +x /bin/example-l2

CMD [ "/bin/example-l2"]