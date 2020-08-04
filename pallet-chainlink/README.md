# Chainlink pallet

## Purpose

This pallet allows your substrate built parachain/blockchain to interract with [chainlink](https://chain.link/). [Pallets](https://substrate.dev/docs/en/tutorials/build-a-dapp/pallet) are modular pieces of the Polkadot Substrate that make it easier for your parachain to interact with technologies. This is essential for working with any kind of external data API from outside your blockchain.

Essentially, a pallet is a lego piece you can add to another blockchain built on the Substrate/Polkadot infrastructure.

## Installation

Using `pallet-chainlink` is fairly straightforward and requires a couple easy steps:

- add the correct dependency to your runtime
- use some of the pallet bundled functions

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
  type Currency = balances::Module<Runtime>;
  type Callback = example_module::Call<Runtime>;
  type ValidityPeriod = ValidityPeriod;
}

parameter_types! {
	pub const ValidityPeriod: u32 = 50;
}
...
// In construct_runtime!, add the pallet
...
construct_runtime!(
    ...
    Chainlink: chainlink::{Module, Call, Storage, Event<T>},
  }
);
```

Add necessary `use` declarations:

```rust
use chainlink::{CallbackWithParameter, Event, Trait as ChainlinkTrait};

pub trait Trait: chainlink::Trait + ChainlinkTrait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type Callback: From<Call<Self>> + Into<<Self as ChainlinkTrait>::Callback>;
}
```

You can now call the right chainlink Extrinsic:

```rust
pub fn send_request(origin, operator: T::AccountId) -> DispatchResult {
    let parameters = ("get", "https://min-api.cryptocompare.com/data/pricemultifull?fsyms=ETH&tsyms=USD", "path", "RAW.ETH.USD.PRICE", "times", "100000000");
    let call: <T as Trait>::Callback = Call::callback(vec![]).into();
    <chainlink::Module<T>>::initiate_request(origin, operator, 1, 0, parameters.encode(), 100, call.into())?;

    Ok(())
}
```

This call refers to a callback Extrinsic that mut be define in the pallet. It will receive back the chainlink Operator's result:

```rust
pub fn callback(origin, result: u128) -> DispatchResult {
    ensure_root(origin)?;

    let r : u128 = u128::decode(&mut &result[..]).map_err(|err| err.what())?;
    <Result>::put(r);
    Ok(())
}

impl <T: Trait> CallbackWithParameter for Call<T> {
    fn with_result(&self, result: u128) -> Option<Self> {
        match *self {
            Call::callback(_) => Some(Call::callback(result)),
            _ => None
        }
    }
}
```

Under the hood a specific event will be picked up by chainlink nodes that will in turn call be a well-known Extrinsic.

### Genesis Configuration

This template pallet does not have any genesis configuration.

## Reference Docs

You can view the reference docs for this pallet by running:

```
cargo doc --open
```
