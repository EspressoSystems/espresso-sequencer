FROM ubuntu:jammy

RUN apt-get update \
&&  apt-get install -y curl wait-for-it \
&&  rm -rf /var/lib/apt/lists/*

COPY target/release/cdn-server /bin/cdn-server
RUN chmod +x /bin/cdn-server

ENV ESPRESSO_CDN_SERVER_PORT=50000

# Set up view timing for optimal performance in high volume conditions. We set a fairly long minimum
# propose time to wait to build up a large block before proposing.
ENV ESPRESSO_CDN_SERVER_MIN_PROPOSE_TIME=10s
# We set the minimum block size to 1, limiting the number of empty blocks but allowing us to propose
# a block immediately after the minimum propose time, keeping latency low in low-volume cases.
ENV ESPRESSO_CDN_SERVER_MIN_TRANSACTIONS=1
# Since min transactions is 1, max propose time only controls the frequency of empty blocks. We set
# this not too much higher than min propose time, for reasonable latency in low volume conditions,
# when empty blocks are required to push non-empty blocks through the pipeline.
ENV ESPRESSO_CDN_SERVER_MAX_PROPOSE_TIME=30s
# The view timeout is larger, since it should only matter when something goes wrong (i.e. leader
# failure).
ENV ESPRESSO_CDN_SERVER_NEXT_VIEW_TIMEOUT=5m

CMD [ "/bin/cdn-server"]
