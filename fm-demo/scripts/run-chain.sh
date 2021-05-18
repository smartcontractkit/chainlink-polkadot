#!/usr/bin/env bash

set -e

echo "*** Run substrate chain ***"

cd $(dirname ${BASH_SOURCE[0]})/..
touch ./external_initiator.env

docker-compose down --remove-orphans
docker-compose up -d chain
