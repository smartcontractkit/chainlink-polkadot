# Chainlink-polkadot

This repository contains all relevant project necessary to have [polkadot](https://polkadot.network/) and [substrate](https://www.parity.io/substrate/) chains interact with [chainlink](https://chain.link/).

This is WIP and will evolve frequently.

## How to use `pallet-chainlink`?

Using `pallet-chainlink` is fairly straightforward and requires a couple easy steps:

* add the correct dependency to your runtime
* use some of the pallet bundled functions

### Add the pallet dependency

Update `Cargo.toml` to reference `pallet-chainlink`.

Add the following section:

```toml
[dependencies.chainlink]
default_features = false
package = 'pallet-chainlink'
```

And amend the `std` section so that it shows like this:

```toml
std = [
    ... // all the existing pallets
    'chainlink/std'
]
```

### Use provided functions

Edit `lib.rs` to that it references `pallet-chainlink`:

```rust
...
// Add the chainlink Trait
impl chainlink::Trait for Runtime {
	type Event = Event;
}
...
// In construct_runtime!, add the pallet
...
construct_runtime!(
    ...
    Chainlink: chainlink::{Module, Event<T>},
  }
);
```

Add necessary `use` declarations:

```rust
use chainlink::{Event, create_get_parse_request};
```

You can now deposit chainlink events:

```rust
Self::deposit_event(create_get_parse_request::<T>(who));
```

Those will be picked up by chainlink nodes that will in turn 

See the full [example](substrate-node-example/runtime/src/example.rs) for more details.