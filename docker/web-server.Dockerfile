FROM ubuntu:jammy

RUN apt-get update \
    &&  apt-get install -y curl wait-for-it \
    &&  rm -rf /var/lib/apt/lists/*

COPY target/release/web-server /bin/web-server
RUN chmod +x /bin/web-server

ENV ESPRESSO_WEB_SERVER_PORT=50000

CMD [ "/bin/web-server"]
