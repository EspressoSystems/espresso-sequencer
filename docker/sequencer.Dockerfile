FROM ubuntu:jammy

RUN apt-get update \
&&  apt-get install -y curl wait-for-it \
&&  rm -rf /var/lib/apt/lists/*

COPY target/x86_64-unknown-linux-musl/release/sequencer /bin/sequencer
RUN chmod +x /bin/sequencer

CMD [ "/bin/sequencer"]
