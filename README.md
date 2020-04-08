# Chainlink-polkadot

This repository contains all relevant project necessary to have [polkadot](https://polkadot.network/) and [substrate](https://www.parity.io/substrate/) chains interact with [chainlink](https://chain.link/).

This is WIP and will evolve frequently.

## How to use `pallet-chainlink`?

Complete documentation is accessible in the pallet [README](pallet-chainlink/README.md).

See the full [example](substrate-node-example/runtime/src/example.rs) for more details.

## Run the example

`rust-toolchain` is used to make sure the correct rust version is used. Make sure to install the WASM target using:

```
rustup target add wasm32-unknown-unknown
```

`substrate-node-example` shows off out to use `pallet-chainlink` end-to-end.
To test:

* start the chain using `make run-chain`
* start the frontend using `make run-front-end`

You are now ready to send test requests and see the result being provided back by an Oracle.

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
{"SpecIndex": "Vec<u8>",
 "RequestIdentifier": "u64",
 "DataVersion": "u64"}
```

Then in `Extrinsincs`, `example` / `sendRequest` can be submitted.