#!/usr/bin/env bash

set -e

echo "*** Run External Initiator ***"

cd $(dirname ${BASH_SOURCE[0]})/..
touch ./external_initiator.env

docker-compose up -d external-initiator
