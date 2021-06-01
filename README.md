# Chainlink-polkadot

This repository contains the [Chainlink](https://chain.link/) feed pallet as well as an example node showing how to integrate
it in [Substrate](https://www.substrate.io/)-based chains.

It also includes the (outdated) `pallet-chainlink` for interacting with the Chainlink job-based oracle system.

## How to integrate the Chainlink feed pallet into a runtime?
The pallet is added to the runtime like any regular pallet (see [tutorial](https://substrate.dev/docs/en/tutorials/add-a-pallet/)).
It then needs to be configured. See the [pallet readme](./pallet-chainlink-feed/README.md) for details.

The usage is simple:
```Rust
let feed = T::Oracle::feed(0.into()).ok_or(Error::<T>::FeedMissing)?;
let RoundData { answer, .. } = feed.latest_data();
do_something_with_answer(answer);
```
See [the template pallet](./substrate-node-example/pallets/template/src/lib.rs) for a full example showing how to access a price feed.


## Run the example

`substrate-node-example` demonstrates how to use `pallet-chainlink-feed` end-to-end.
To test:

* start the chain using `make run-temp` (for a temporary node which cleans up after itself)
* connect to the chain by pointing https://polkadot.js.org/apps/ (or a locally hosted version) to the local dev node 
* specify the types by copying `substrate-node-example/types.json` into the input at `Settings > Developer`

You are now ready to send extrinsics to the pallet.

