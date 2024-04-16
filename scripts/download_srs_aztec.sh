#!/usr/bin/env bash

set -euo pipefail

if [ -f "$AZTEC_SRS_PATH" ]; then
    echo "SRS file $AZTEC_SRS_PATH exists"
else
    echo "SRS file $AZTEC_SRS_PATH does not exist, downloading ..."
    wget -P "$(dirname $AZTEC_SRS_PATH)" "https://github.com/EspressoSystems/ark-srs/releases/download/v0.2.0/$(basename $AZTEC_SRS_PATH)"
fi
