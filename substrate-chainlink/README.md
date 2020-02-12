# Chainlink components for Substrate

This tool automates the setup and running of Chainlink components to read/write from a Substrate chain.

## Prerequisites

The Chainlink components assume that you already have a Substrate node running with the Chainlink pallet.

_See the directory above for more instructions on how to run this._

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
