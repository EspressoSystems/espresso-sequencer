FROM ubuntu:jammy

RUN apt-get update \
&&  apt-get install -y curl wait-for-it \
&&  rm -rf /var/lib/apt/lists/*

COPY target/x86_64-unknown-linux-musl/release-lto/sequencer /bin/sequencer
RUN chmod +x /bin/sequencer

ENV ESPRESSO_SEQUENCER_CDN_URL

CMD [ "/bin/sequencer"]
