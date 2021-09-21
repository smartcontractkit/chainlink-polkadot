# Chainlink Price Feed Pallet

This pallet provides Chainlink price feed functionality to Substrate-based chains that include it.

It aims to mostly port
the [FluxAggregator](https://github.com/smartcontractkit/chainlink/blob/dabada25f5dd7bbc49a76ed1d172a83083cdd8f0/evm-contracts/src/v0.6/FluxAggregator.sol)
Solidity contract but changes the design in the following ways:

+ **Storage Pruning:** It is usually not advisable to keep all oracle state around, so a feed supports automatic storage
  pruning.
+ **View Functions:** Substrate does not have an exact equivalent of Solidity view functions. We will instead make the
  client side smarter to read and interpret storage to surface the same information.
+ **Contracts --> Feeds:** Vanilla Substrate does not have contracts. We will thus replace the deployment of a contract
  with the creation of per-feed metadata and a storage prefix.
    - Oracle metadata (admin and withdrawable funds) will be tracked globally (for the whole pallet) per oracle.
+ **On-Chain Interaction:** In Substrate runtimes, pallets interact with other pallets differently from how users
  interact with the pallet. We thus define two interfaces for interaction with the pallet:
    - Rust traits for pallets (`FeedOracle` as well as `FeedInterface` and `MutableFeedInterface`)
    - Extrinsics for users of the chain
+ **Token (Separation of Concerns):** Most of the interaction with the LINK token will happen via a dedicated token
  pallet and thus the token-based interaction with the feed pallet will be minimal.
  (Token changes are done via the configured type implementing the `Currency` trait.)
    - We do introduce a `Debt` storage item to track how much the pallet owes to oracles in cases where the reserves are
      insufficient.

## Integration into Chain

Include the pallet in your runtime and configure it. The following snippet shows an example of a runtime configuration:

```Rust
parameter_types! {
    // Used to generate the fund account that pools oracle payments.
	pub const FeedModule: PalletId = PalletId(*b"linkfeed");
    // The minimum amount of tokens to keep in reserve for oracle payment.
	pub const MinimumReserve: Balance = ExistentialDeposit::get() * 1000;
    // Maximum length of the feed description.
	pub const StringLimit: u32 = 30;
    // Maximum number of oracles per feed.
	pub const OracleCountLimit: u32 = 25;
    // Maximum number of feeds.
	pub const FeedLimit: FeedId = 100;
}

// implement the pallet for the runtime
impl pallet_chainlink_feed::Config for Runtime {
    type Event = Event;
    type FeedId = FeedId;
    type Value = Value;
    // A module that provides currency functionality to manage
    // oracle rewards. Balances in this example.
    type Currency = Balances;
    type PalletId = FeedPalletId;
    type MinimumReserve = MinimumReserve;
    type StringLimit = StringLimit;
    type OracleCountLimit = OracleCountLimit;
    type FeedLimit = FeedLimit;
    // Provide your custom callback that gets called once a new value is available
    // `()` is a noop
    type OnAnswerHandler = ();
    // Implementation of the WeightInfo trait for your runtime.
    // Default weights available in the pallet but not recommended for production.
    type WeightInfo = ChainlinkWeightInfo;
}
```

## Usage in a Pallet

You need to inject the pallet into the consuming pallet in a similar way to how the feed pallet depends on a pallet
implementing the `Currency` trait. Define an associated `Oracle` type implementing the `FeedOracle` trait.

```Rust
pub trait Config: frame_system::Config {
    // -- snip --
    type Oracle: FeedOracle<Self>;
}
```

You can then access a feed by calling the `feed` and `feed_mut` functions in your pallet code:

```rust
let feed = T::Oracle::feed(0.into()).ok_or(Error::<T>::FeedMissing) ?;
let RoundData { answer,..} = feed.latest_data();
do_something_with_answer(answer);
```

## Combine the pallet with a custom adapter pallet

Use case: Write your custom abstraction over the `chainlink-pallet`.

```rust
pub type FeedIdFor<T> = <T as pallet_chainlink_feed::Config>::FeedId;
pub type FeedValueFor<T> = <T as pallet_chainlink_feed::Config>::Value;
pub type MomentOf<T> = <<T as Config>::Time as Time>::Moment;
// Alias for price type
pub type Price = frame_support::sp_runtime::FixedI128;

// we "inherit" here from the `pallet_chainlink_feed::Config`.
// The actual type will be the `Runtime` for which we have to implement both pallet config traits
pub trait Config: frame_system::Config + pallet_chainlink_feed::Config {
    /// Type to keep track of timestamped values
    type Time: frame_support::traits::Time;
    /// Type used to identify the different currency types.
    type CurrencyId: Parameter + Member + MaybeSerializeDeserialize;
    /// The origin which can map a `FeedId` of chainlink oracle to `CurrencyId`.
    type FeedMapOrigin: EnsureOrigin<Self::Origin>;
    /// Convert feed_value type of chainlink to price type
    type Convert: Convert<FeedValueFor<Self>, Option<Price>>;
    // -- snip --
}

/// Stores the timestamp of the latest answer of each feed 
/// (feed) -> Timestamp
#[pallet::storage]
#[pallet::getter(fn latest_answer_timestamp)]
pub type LatestAnswerTimestamp<T: Config> = StorageMap<_, Twox64Concat, FeedIdFor<T>, MomentOf<T>, ValueQuery>;

/// Mapping from currency_id to feed_id
///
/// FeedIdMapping: CurrencyId => FeedId
#[pallet::storage]
#[pallet::getter(fn feed_id_mapping)]
pub type FeedIdMapping<T: Config> = StorageMap<_, Twox64Concat, T::CurrencyId, FeedIdFor<T>, OptionQuery>;


#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Maps the given currency id to an existing chainlink feed.
    ///
    /// The dispatch origin of this call must be `FeedMapOrigin`.
    ///
    /// - `currency_id`: currency_id.
    /// - `feed_id`: feed_id in chainlink oracle.
    #[pallet::weight(< T as Config >::WeightInfo::map_feed_id())]
    pub fn map_feed_id(
        origin: OriginFor<T>,
        currency_id: T::CurrencyId,
        feed_id: FeedIdFor<T>,
    ) -> DispatchResult {
        T::FeedMapOrigin::ensure_origin(origin)?;
        // if already mapped, update
        let old_feed_id = FeedIdMapping::<T>::mutate(&currency_id, |maybe_feed_id| maybe_feed_id.replace(feed_id));
        // -- snip --
        Ok(())
    }
}

impl<T: Config> Pallet<T> {
    fn get_price_from_chainlink_feed(currency_id: &T::CurrencyId) -> Option<Price> {
        Self::feed_id_mapping(currency_id)
            .and_then(<pallet_chainlink_feed::Pallet<T>>::feed)
            .map(|feed| feed.latest_data().answer)
            .map(T::Convert::convert)
    }
    // -- snip --
}

// Implement the `OnAnswerHandler` that gets called by the `chainlink pallet` on every new answer 
impl<T: Config> pallet_chainlink_feed::traits::OnAnswerHandler<T> for Pallet<T> {
    fn on_answer(feed_id: FeedIdFor<T>, _: pallet_chainlink_feed::RoundData<T::BlockNumber, FeedValueFor<T>>) {
        // keep track of the timestamp
        LatestAnswerTimestamp::<T>::insert(feed_id, T::Time::now());
    }
}

// An example for the `Convert` type for `type Value = u128;`
pub struct ConvertU128ToPrice;

impl Convert<u128, Option<Price>> for ConvertU128ToPrice {
    fn convert(value: u128) -> Option<Price> {
        Some(Price::from_inner(value))
    }
}

```

## Architecture

### Storage

Most storage items in the pallet are scoped under a feed identified by an id. The storage layout follows the following
structure.

Scoped to a feed:

```
FeedId => FeedConfig
(FeedId, RoundId) => Round
(FeedId, RoundId) => RoundDetails
(FeedId, requester: AccountId) => Requester
(FeedId, oracle_acc: AccountId) => OracleStatus
```

Associated with an account:

```
oracle_acc: AccountId => OracleMeta
feed_creator: AccountId => ()
```

Pallet-global values:

```
PalletAdmin
PendingPalletAdmin
FeedCounter
```

### Interaction

Access to a feed is done via the `Feed` type which automatically syncs changes to storage on drop
(if the `should_sync` flag is set). This makes it harder to forgot to update the storage with changes but means that
care should be taken with scoping the variable. (E.g. the feed needs to be initialized
*within* the closure passed to `with_transaction_result` in order for the auto-sync writes to be covered by the
transactional write.)
