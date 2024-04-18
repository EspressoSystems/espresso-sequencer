#!/usr/bin/env bash

set -euo pipefail

export AZTEC_SRS_PATH="${AZTEC_SRS_PATH:-$PWD/data/aztec20/kzg10-aztec20-srs-1048584.bin}"

if [ -f "$AZTEC_SRS_PATH" ]; then
    echo "SRS file $AZTEC_SRS_PATH exists"
else
    echo "SRS file $AZTEC_SRS_PATH does not exist, downloading ..."
    wget -q -P "$(dirname $AZTEC_SRS_PATH)" "https://github.com/EspressoSystems/ark-srs/releases/download/v0.2.0/$(basename $AZTEC_SRS_PATH)"
fi
