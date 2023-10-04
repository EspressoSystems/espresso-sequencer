FROM ubuntu:jammy

ARG TARGETARCH

RUN apt-get update && apt-get install -y curl libcurl4 tini
ENTRYPOINT ["tini", "--"]

COPY target/$TARGETARCH/release/commitment-task /bin/commitment-task
RUN chmod +x /bin/commitment-task

# When running as a Docker service, we always want a healthcheck endpoint, so set a default for the
# port that the HTTP server will run on. This can be overridden in any given deployment environment.
ENV ESPRESSO_COMMITMENT_TASK_PORT=80
HEALTHCHECK --interval=1s --timeout=1s --retries=100 CMD curl --fail http://localhost:${ESPRESSO_COMMITMENT_TASK_PORT}/healthcheck || exit 1

CMD [ "/bin/commitment-task"]
