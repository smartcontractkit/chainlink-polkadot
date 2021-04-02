#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;

use pallet_chainlink_feed::{FeedInterface, FeedOracle, MutableFeedInterface, RoundData, RoundId};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	type Oracle: FeedOracle<Self>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Something get(fn something): Option<u32>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Trait>::AccountId,
		BlockNumber = <T as frame_system::Trait>::BlockNumber,
		Value = <<<T as Trait>::Oracle as FeedOracle<T>>::Feed as FeedInterface<T>>::Value,
	{
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, AccountId),
		NewData(Value),
		RoundData(Value, BlockNumber, RoundId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		FeedMissing,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and return a DispatchResult (explicitly
// or implicitly).
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		/// Reads the latest value from oracle feed 0 and emits it as an event.
		/// Reads the value 3 rounds ago and emits it as an event.
		#[weight = 10_000 + T::DbWeight::get().reads(2)]
		fn read_value(origin) {
			let _sender = ensure_signed(origin)?;
			let feed = T::Oracle::feed(0.into()).ok_or(Error::<T>::FeedMissing)?;

			// we can read the latest data
			let RoundData { answer, .. } = feed.latest_data();

			Self::deposit_event(RawEvent::NewData(answer));

			// or read data from a specific round
			let round = feed.latest_round().saturating_sub(3);
			if let Some(RoundData { answer, updated_at, answered_in_round, .. }) = feed.data_at(round) {
				Self::deposit_event(RawEvent::RoundData(answer, updated_at, answered_in_round));
			}
		}

		/// Requests a new round of data for feed 0 for the sender.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		fn request_new_round(origin) {
			let requester = ensure_signed(origin)?;
			// if we get a mutable instance of the feed
			let mut feed = T::Oracle::feed_mut(0.into()).ok_or(Error::<T>::FeedMissing)?;

			// we can request a new round
			feed.request_new_round(requester)?;
		}

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match Something::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					Something::put(new);
					Ok(())
				},
			}
		}
	}
}
