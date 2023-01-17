FROM ubuntu:jammy

RUN apt-get update \
&&  apt-get install -y curl wait-for-it \
&&  rm -rf /var/lib/apt/lists/*

COPY target/x86_64-unknown-linux-musl/release/hermez-adaptor /bin/hermez-adaptor
RUN chmod +x /bin/hermez-adaptor

CMD [ "/bin/hermez-adaptor"]
