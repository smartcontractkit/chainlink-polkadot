# Chainlink pallet

## Purpose

This pallet allows to interract with [chainlink](https://chain.link/).

## Installation

### Runtime `Cargo.toml`

To add this pallet to your runtime, simply include the following to your runtime's `Cargo.toml` file:

```TOML
[dependencies.chainlink]
default_features = false
git = 'TODO'
```

and update your runtime's `std` feature to include this pallet:

```TOML
std = [
    # --snip--
    'chainlink/std',
]
```

### Runtime `lib.rs`

You should implement it's trait like so:

```rust
/// Used for test_module
impl chainlink::Trait for Runtime {
	type Event = Event;
}
```

and include it in your `construct_runtime!` macro:

```rust
Chainlink: chainlink::{Module, Call, Storage, Event<T>},
```

### Genesis Configuration

This template pallet does not have any genesis configuration.

## Reference Docs

You can view the reference docs for this pallet by running:

```
cargo doc --open
```
