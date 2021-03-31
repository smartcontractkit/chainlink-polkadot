# Chainlink Price Feed Module

A pallet providing chainlink price feed functionality to Substrate-based chains that include it.

## Integration into Chain
Include the pallet in your runtime and configure it.
The following snippet shows an example:

```Rust
parameter_types! {
    // Used to generate the fund account that pools oracle payments.
	pub const FeedModule: ModuleId = ModuleId(*b"linkfeed");
    // The minimum amount of tokens to keep in reserve for oracle payment.
	pub const MinimumReserve: Balance = ExistentialDeposit::get() * 1000;
    // Maximum length of the feed description.
	pub const StringLimit: u32 = 30;
    // Maximum number of oracles per feed.
	pub const OracleCountLimit: u32 = 25;
    // Maximum number of feeds.
	pub const FeedLimit: FeedId = 100;
    // Minimum amount of rounds to keep when pruning.
	pub const PruningWindow: RoundId = 15;
}

impl pallet_chainlink_feed::Trait for Runtime {
	type Event = Event;
	type FeedId = FeedId;
	type Value = Value;
    // A module that provides currency functionality to manage
    // oracle rewards. Balances in this example.
	type Currency = Balances;
	type ModuleId = FeedModule;
	type MinimumReserve = MinimumReserve;
	type StringLimit = StringLimit;
	type OracleCountLimit = OracleCountLimit;
	type FeedLimit = FeedLimit;
	type PruningWindow = PruningWindow;
    // Implementation of the WeightInfo trait for your runtime.
    // Default weights available in the pallet but not recommended for production.
	type WeightInfo = ChainlinkWeightInfo;
}
```

## Usage in Pallet
You need to inject the pallet into the consuming pallet in a similar way to how the feed pallet
depends on a pallet implementing the `Currency` trait.
Define an associated `Oracle` type implementing the `FeedOracle` trait.

```Rust
pub trait Trait: frame_system::Trait {
    // -- snip --
    type Oracle: FeedOracle<Self>;
}
```

You can then access a feed by calling the `feed` and `feed_mut` functions in your pallet code:
```Rust
let feed = T::Oracle::feed(0.into()).ok_or(Error::<T>::FeedMissing)?;
let RoundData { answer, .. } = feed.latest_data();
do_something_with_answer(answer);
```
