FROM ghcr.io/espressosystems/geth-l1:main

COPY ./geth-config/genesis-default.json /genesis.json
COPY ./geth-config/test-jwt-secret.txt /config/test-jwt-secret.txt
