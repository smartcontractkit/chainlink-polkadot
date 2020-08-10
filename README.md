# Chainlink-polkadot

This repository contains all relevant project necessary to have [polkadot](https://polkadot.network/) and [substrate](https://www.parity.io/substrate/) chains interact with [chainlink](https://chain.link/).

This is WIP and will evolve frequently.

## What is Polkadot and Chainlink?

[Chainlink](https://chain.link/) is a decentralized oracle technology that allows off-chain data to be connected to any chain.
[Polkadot](https://polkadot.network/) is a scalable, heterogeneous, multi-chain technology, that can be used to create, maintain, and connect blockchains.

## Run the example

### Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/) version >1.26
- [jq](https://stedolan.github.io/jq/download/)

`rust-toolchain` is used to make sure the correct rust version is used. Make sure to install the WASM target using:

```
rustup target add wasm32-unknown-unknown
```

### Running the example

[You can view a youtube demo here.](https://www.youtube.com/watch?v=0rZghy0TIOQ&feature=emb_title)

`substrate-node-example` shows off out to use `pallet-chainlink` end-to-end. It first makes a local dummy substrate built parachain, and then spins up some local Chainlink nodes that have parameters to connect to the parachain.
To test:

- start the chain using `make run-chain`
- install frontend dependencies `cd substrate-node-example/front-end && yarn install && cd ../..`
- start the frontend using `make run-front-end`

To spin up some simple Chainlink nodes to intereact with the parachain you just created, run the setupcommand:
`cd substrate-chainlink`
`./setup`

This will run a number of docker commands that will spin up your Chainlink node. You can access them by going to your browser at:
`http://localhost:6691/`

The password to this Chainlink node is `notreal@fakeemail.ch` and the password is `twochains`.
You can see in the logs of `./setup` if the address has moved.

_note_
If you'd like to restart you will have to remove all the docker containers. You can do this by running:

```
docker-compose down
docker-compose up
```

You are now ready to send test requests and see the result being provided back by an Oracle. You can continue to follow the [demo youtube video](https://www.youtube.com/watch?v=0rZghy0TIOQ&feature=emb_title) to see how to interact with the GUIs.

### Send a test request using PolkadotJS

```js
const alice = ...;
const txHash = await api.tx._exampleModule_
  .sendRequest("ACCOUNT_ID")
  .signAndSend(alice);
console.log(`Submitted with hash ${txHash}`);
```

### Send a test request using PolkadotJS Apps

Make sure you add the following additional to `Settings/Developer` section:

```json
{ "SpecIndex": "Vec<u8>", "RequestIdentifier": "u64", "DataVersion": "u64" }
```

Then in `Extrinsincs`, `example` / `sendRequest` can be submitted.

## How to use `pallet-chainlink`?

Complete documentation is accessible in the pallet [README](pallet-chainlink/README.md).

See the full [example](substrate-node-example/runtime/src/example.rs) for more details.
