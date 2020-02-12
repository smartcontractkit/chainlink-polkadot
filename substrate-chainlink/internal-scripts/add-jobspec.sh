#!/bin/bash

source ./internal-scripts/common.sh

add_jobspec() {
  title "Adding Jobspec #$1 to Chainlink node..."

  CL_URL="http://localhost:669$1"

  login_cl "$CL_URL"

  payload=$(
    cat <<EOF
{
  "initiators": [
    {
      "type": "external",
      "params": {
        "name": "test-ei",
        "body": {
          "endpoint": "substrate-node",
          "accountIds": ["0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"]
        }
      }
    }
  ],
  "tasks": [
    {
      "type": "httpget"
    },
    {
      "type": "jsonparse"
    },
    {
      "type": "multiply"
    },
    {
      "type": "substrate",
      "params": {
        "type": "uint128"
      }
    }
  ]
}
EOF
  )

  curl -s -b ./tmp/cookiefile -d "$payload" -X POST -H 'Content-Type: application/json' "$CL_URL/v2/specs" &>/dev/null

  echo "Jobspec has been added to Chainlink node"
  title "Done adding jobspec #$1"
}
