FROM ubuntu:jammy

ARG TARGETARCH

RUN apt-get update \
    &&  apt-get install -y curl libcurl4 wait-for-it tini \
    &&  rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["tini", "--"]

COPY target/$TARGETARCH/release/submit-transactions /bin/submit-transactions
RUN chmod +x /bin/submit-transactions

CMD [ "/bin/submit-transactions"]
