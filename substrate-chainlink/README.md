# Chainlink components for Substrate

This tool automates the setup and running of Chainlink components to read/write from a Substrate chain.

## Prerequisites

### Substrate node
The Chainlink components assume that you already have a Substrate node running with the Chainlink pallet.

_See the directory above for more instructions on how to run this._

### Chainlink environment

Fill in `chainlink.env` with the correct project ID, as specifed [here](https://github.com/smartcontractkit/chainlink-polkadot/issues/36#issuecomment-694701872)

## Running

### Initial setup

_Note: Make sure you have cd-ed into this directory_

If you are running the Substrate node locally, simply run:

```bash
./setup
```

If you are running the Substrate node externally (with `--ws-external`), run:

```bash
./setup "ws://your_substrate_node_ip:9944/"
```

This will create and start 3 Chainlink nodes, with an adapter and EI connected to each.

It will instruct you to fund 3 different addresses, which you can do through the Substrate UI.

### Start/stop

To stop the nodes, run:

```bash
docker-compose down
```

And to start them again, run:

```bash
docker-compose up
```

The env var `SA_ENDPOINT` needs to be set before bringing the services up.
`./setup` will default to `ws://host.docker.internal:9944/`, but you need to set this again if it is unset.

## Troubleshooting

### Stuck at "waiting for localhost:669X"

Check the logs of your docker container running the chainlink node:
`docker logs -f substrate-chainlink_chainlink-node1_1`

You need to make sure you followed the setup section

### cat jobids.txt is null

The external initiator needs to be up and running before you can create jobs.

It might be the case that it wasn't operational yet, in this case simply re-execute the
part of job creation from setup:
```bash
source ./internal-scripts/add-jobspec.sh

add_jobspec ...
```
