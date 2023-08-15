FROM ubuntu:jammy

ARG TARGETARCH

RUN apt-get update &&  rm -rf /var/lib/apt/lists/*

COPY target/$TARGETARCH/release/commitment-task /bin/commitment-task
RUN chmod +x /bin/commitment-task

CMD [ "/bin/commitment-task"]