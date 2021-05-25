#!/usr/bin/env bash

set -e

echo "*** Run all components of the integration ***"

cd $(dirname ${BASH_SOURCE[0]})/..
./scripts/run-chain.sh
yarn
node ./feedSetup.js
./scripts/run-chainlink.sh
echo "Waiting a bit for the chainlink services to be ready"
sleep 10
./scripts/add-bridges.sh
./scripts/ei-config.sh
./scripts/run-ei.sh
echo "Waiting a bit for the initiator to be ready"
sleep 10
./scripts/add-jobspecs.sh





