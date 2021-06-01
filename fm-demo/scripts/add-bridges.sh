#!/bin/bash

source ./scripts/common.sh

echo "Adding Bridges to Chainlink node..."

CL_URL="http://localhost:6688"

login_cl "$CL_URL"

payload=$(
  cat <<EOF
{
"name": "substrate-adapter1",
"url": "http://substrate-adapter1:8080/"
}
EOF
)

payload2=$(
cat <<EOF
{
"name": "substrate-adapter2",
"url": "http://substrate-adapter2:8080/"
}
EOF
)

curl -s -b ./cookiefile -d "$payload" -X POST -H 'Content-Type: application/json' "$CL_URL/v2/bridge_types" &>/dev/null
curl -s -b ./cookiefile -d "$payload2" -X POST -H 'Content-Type: application/json' "$CL_URL/v2/bridge_types" &>/dev/null

echo "Bridges has been added to Chainlink node"
