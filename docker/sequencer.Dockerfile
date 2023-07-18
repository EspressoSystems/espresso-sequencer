FROM ubuntu:jammy

ARG TARGETARCH

RUN apt-get update \
    &&  apt-get install -y curl wait-for-it \
    &&  rm -rf /var/lib/apt/lists/*

COPY target/$TARGETARCH/release/sequencer /bin/sequencer
RUN chmod +x /bin/sequencer

CMD [ "/bin/sequencer"]
