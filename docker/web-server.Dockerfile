FROM ubuntu:jammy

ARG TARGETARCH

RUN apt-get update \
    &&  apt-get install -y curl libcurl4 wait-for-it tini \
    &&  rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["tini", "--"]

COPY target/$TARGETARCH/release/web-server /bin/web-server
RUN chmod +x /bin/web-server

ENV ESPRESSO_WEB_SERVER_PORT=50000
HEALTHCHECK --interval=1s --timeout=1s --retries=100 CMD curl --fail http://localhost:${ESPRESSO_WEB_SERVER_PORT}/healthcheck  || exit 1

EXPOSE ${ESPRESSO_WEB_SERVER_PORT}

CMD [ "/bin/web-server"]
