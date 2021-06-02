#!/usr/bin/env bash

set -e

echo "*** Run all components of the integration ***"

cd $(dirname ${BASH_SOURCE[0]})/..
docker-compose down --remove-orphans --volumes
./scripts/run-chain.sh
yarn
node ./feedSetup.js
./scripts/run-chainlink.sh
echo "Waiting a bit for the chainlink services to be ready"
sleep 15
./scripts/add-bridges.sh
./scripts/ei-config.sh
./scripts/run-ei.sh
echo "Waiting a bit for the initiator to be ready"
sleep 10
./scripts/add-jobspecs.sh
echo "Waiting a bit for the answer to be written on chain"
# TODO: make it variable related to heartbeat
sleep 60
node ./checkAnswer.js






