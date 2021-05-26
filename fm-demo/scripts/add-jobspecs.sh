#!/bin/bash

source ./scripts/common.sh

echo "Adding JobSpecs to Chainlink node..."

CL_URL="http://localhost:6688"

login_cl "$CL_URL"

payload=$(
cat <<EOF
{
  "initiators": [
    {
      "type": "external",
      "params": {
        "name": "substrate",
        "body": {
          "endpoint": "substrate",
          "feed_id": 0,
          "account_id": "0x7c522c8273973e7bcf4a5dbfcc745dba4a3ab08c1e410167d7b1bdf9cb924f6c",
          "fluxmonitor": {
            "requestData": {
              "data": { "from": "DOT", "to": "USD" }
            },
            "feeds": [{ "url": "http://adapter1:8080" }],
            "threshold": 0.5,
            "absoluteThreshold": 0,
            "precision": 8,
            "pollTimer": { "period": "5s" },
            "idleTimer": { "duration": "15s" }
          }
        }
      }
    }
  ],
  "tasks": [
    {
      "type": "substrate-adapter1",
      "params": { "multiply": 1e8 }
    }
  ]
}
EOF
)

payload2=$(
cat <<EOF
{
  "initiators": [
    {
      "type": "external",
      "params": {
        "name": "substrate",
        "body": {
          "endpoint": "substrate",
          "feed_id": 0,
          "account_id": "0x06f0d58c43477508c0e5d5901342acf93a0208088816ff303996564a1d8c1c54",
          "fluxmonitor": {
            "requestData": {
              "data": { "from": "DOT", "to": "USD" }
            },
            "feeds": [{ "url": "http://adapter1:8080" }],
            "threshold": 0.5,
            "absoluteThreshold": 0,
            "precision": 8,
            "pollTimer": { "period": "5s" },
            "idleTimer": { "duration": "15s" }
          }
        }
      }
    }
  ],
  "tasks": [
    {
      "type": "substrate-adapter2",
      "params": { "multiply": 1e8 }
    }
  ]
}
EOF
)

JOBID=$(curl -s -b ./tmp/cookiefile -d "$payload" -X POST -H 'Content-Type: application/json' "$CL_URL/v2/specs")
echo $JOBID

JOBID=$(curl -s -b ./tmp/cookiefile -d "$payload2" -X POST -H 'Content-Type: application/json' "$CL_URL/v2/specs")
echo $JOBID
echo "Jobspecs has been added to Chainlink node"
