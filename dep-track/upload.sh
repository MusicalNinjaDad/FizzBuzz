#!/bin/bash

# set API Key with `set-key` first
# then call upload PROJECT-UUID FILE

curl -X "PUT" "http://localhost:8081/api/v1/bom" \
     -H 'Content-Type: application/json' \
     -H 'X-API-Key: ' $DEP_TRACK_KEY \
     -d $'{"project": ' "$1" ',"bom": "' "$(base64 $2)" '"}'