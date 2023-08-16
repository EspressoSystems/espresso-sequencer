FROM ubuntu:jammy

ARG TARGETARCH

COPY target/$TARGETARCH/release/commitment-task /bin/commitment-task
RUN chmod +x /bin/commitment-task

CMD [ "/bin/commitment-task"]
