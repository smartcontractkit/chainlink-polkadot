//! # A pallet to interact with Chainlink nodes
//!
//! \## Overview
//!
//! `pallet-chainlink` allows to request external data from chainlink operators. This is done by emitting a well-known event (`OracleEvent`)
//! embedding all required data. This event is listened by operators that in turns call back the `callback` function with the associated data.
//!
//! To initiate a request, users call `initiate_request` with the relevant details, the `operator` AccountId and the `fee` they agree to spend to get the result.
//!
//! To be valid, an operator must register its AccountId first hand via `register_operator`.
//!
//! \## Terminology
//! Operator: a member of chainlink that provides result to requests, in exchange of a fee payment
//! Request: details about what the user expects as result. Must match a Specification supported by an identified Operator
//! Fee: the amount of token a users pays to an operator

#![cfg_attr(not(feature = "std"), no_std)]

#[warn(unused_imports)]
use codec::Codec;
use frame_support::traits::{
	BalanceStatus, Currency, Get, ReservableCurrency, UnfilteredDispatchable,
};
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
	pallet_prelude::*, sp_runtime::traits::Zero, Parameter,
};
use frame_system::ensure_signed;
use sp_std::prelude::*;

#[cfg(test)]
mod tests;

// A trait allowing to inject Operator results back into the specified Call
pub trait CallbackWithParameter {
	fn with_result(&self, result: Vec<u8>) -> Option<Self>
	where
		Self: core::marker::Sized;
}

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	type Currency: ReservableCurrency<Self::AccountId>;

	// A reference to an Extrinsic that can have a result injected. Used as Chainlink callback
	type Callback: Parameter
		+ UnfilteredDispatchable<Origin = Self::Origin>
		+ Codec
		+ Eq
		+ CallbackWithParameter;

	// Period during which a request is valid
	type ValidityPeriod: Get<Self::BlockNumber>;
}

// REVIEW: Use this for transferring currency.
pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

// Uniquely identify a request's specification understood by an Operator
pub type SpecIndex = Vec<u8>;
// Uniquely identify a request for a considered Operator
pub type RequestIdentifier = u64;
// The version of the serialized data format
pub type DataVersion = u64;

decl_storage! {
	trait Store for Module<T: Config> as Chainlink {
		// A set of all registered Operator
		// TODO migrate to 'natural' hasher once migrated to 2.0
		pub Operators get(fn operator): map hasher(blake2_128_concat) T::AccountId => bool;

		// A running counter used internally to identify the next request
		pub NextRequestIdentifier get(fn request_identifier): RequestIdentifier;

		// A map of details of each running request
		// TODO migrate to 'natural' hasher once migrated to 2.0
		// REVIEW: Consider using a struct for the Requests instead of a tuple to increase
		//         readability.
		pub Requests get(fn request): map hasher(blake2_128_concat) RequestIdentifier => (T::AccountId, Vec<T::Callback>, T::BlockNumber, BalanceOf<T>);
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
		Balance = BalanceOf<T>,
	{
		// A request has been accepted. Corresponding fee payment is reserved
		OracleRequest(
			AccountId,
			SpecIndex,
			RequestIdentifier,
			AccountId,
			DataVersion,
			Vec<u8>,
			Vec<u8>,
			Balance,
		),

		// A request has been answered. Corresponding fee payment is transferred
		OracleAnswer(AccountId, RequestIdentifier, AccountId, Vec<u8>, Balance),

		// A new operator has been registered
		OperatorRegistered(AccountId),

		// An existing operator has been unregistered
		OperatorUnregistered(AccountId),

		// A request didn't receive any result in time
		KillRequest(RequestIdentifier),
	}
);

decl_error! {
	// Error for the ChainLink module.
	pub enum Error for Module<T: Config> {
		// Manipulating an unknown operator
		UnknownOperator,
		// Manipulating an unknown request
		UnknownRequest,
		// Not the expected operator
		WrongOperator,
		// An operator is already registered.
		OperatorAlreadyRegistered,
		// Callback cannot be deserialized
		UnknownCallback,
		// Fee provided does not match minimum required fee
		InsufficientFee,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {

		fn deposit_event() = default;

		// REVIEW: Use `///` instead of `//` to make these doc comments that are part of the crate documentation.
		// Register a new Operator.
		// Fails with `OperatorAlreadyRegistered` if this Operator (identified by `origin`) has already been registered.
		#[weight = 10_000]
		pub fn register_operator(origin) -> DispatchResult {
			let who : <T as frame_system::Config>::AccountId = ensure_signed(origin)?;

			ensure!(!<Operators<T>>::get(&who), Error::<T>::OperatorAlreadyRegistered);

			Operators::<T>::insert(&who, true);

			Self::deposit_event(RawEvent::OperatorRegistered(who));

			Ok(())
		}

		// Unregisters an existing Operator
		// TODO check weight
		#[weight = 10_000]
		pub fn unregister_operator(origin) -> DispatchResult {
			let who : <T as frame_system::Config>::AccountId = ensure_signed(origin)?;

			if Operators::<T>::take(who.clone()) {
				Self::deposit_event(RawEvent::OperatorUnregistered(who));
				Ok(())
			} else {
				Err(Error::<T>::UnknownOperator.into())
			}
		}

		// Hint specified Operator (via its `AccountId`) of a request to be performed.
		// Request details are encapsulated in `data` and identified by `spec_index`.
		// `data` must be SCALE encoded.
		// If provided fee is sufficient, Operator must send back the request result in `callback` Extrinsic which then will dispatch back to the request originator callback identified by `callback`.
		// The fee is `reserved` and only actually transferred when the result is provided in the callback.
		// Operators are expected to listen to `OracleRequest` events. This event contains all the required information to perform the request and provide back the result.
		// REVIEW: Use a `BalanceOf` type for the fee instead of `u32` as shown here: https://substrate.dev/recipes/3-entrees/currency.html
		// TODO check weight
		#[weight = 10_000]
		pub fn initiate_request(origin, operator: T::AccountId, spec_index: SpecIndex, data_version: DataVersion, data: Vec<u8>, fee: BalanceOf<T>, callback: <T as Config>::Callback) -> DispatchResult {
			let who : <T as frame_system::Config>::AccountId = ensure_signed(origin)?;

			ensure!(<Operators<T>>::get(&operator), Error::<T>::UnknownOperator);
			// REVIEW: Should probably be at least `ExistentialDeposit`
			ensure!(fee > BalanceOf::<T>::zero(), Error::<T>::InsufficientFee);

			T::Currency::reserve(&who, fee)?;

			let request_id = NextRequestIdentifier::get();
			// REVIEW: This can overflow. You can make a maximum of `u64::max_value()` requests.
			//         Default behavior for `u64` is to wrap around to 0, but you might want to
			//         make this explicit.
			//         I think using `wrapping_add` could be fine here, because it should be fine to
			//         start at 0 when you reach `u64::max_value()`.
			NextRequestIdentifier::put(request_id + 1);

			// REVIEW: Is it intentional that requests are only valid during the current block?
			let now = frame_system::Pallet::<T>::block_number();
			// REVIEW: You might want to think about and document that your requests can be overwritten
			//         as soon as the request id wraps around.
			// REVIEW: Is the `Vec` intended for forward compatibility? It seems superfluous here.
			Requests::<T>::insert(request_id, (operator.clone(), vec![callback], now, fee));

			Self::deposit_event(RawEvent::OracleRequest(operator, spec_index, request_id, who, data_version, data, "Chainlink.callback".into(), fee));

			Ok(())
		}

		// The callback used to be notified of all Operators results.
		// Only the Operator responsible for an identified request can notify back the result.
		// Result is then dispatched back to the originator's callback.
		// The fee reserved during `initiate_request` is transferred as soon as this callback is called.
		// TODO check weight
		#[weight = 10_000]
		fn callback(origin, request_id: RequestIdentifier, result: Vec<u8>) -> DispatchResult {
			let who : <T as frame_system::Config>::AccountId = ensure_signed(origin)?;

			ensure!(<Requests<T>>::contains_key(&request_id), Error::<T>::UnknownRequest);
			let (operator, callback, _, fee) = <Requests<T>>::get(&request_id);
			ensure!(operator == who, Error::<T>::WrongOperator);

			// REVIEW: This does not make sure that the fee is payed. `repatriate_reserved` removes
			//         *up to* the amount passed. [See here](https://substrate.dev/rustdocs/master/frame_support/traits/trait.ReservableCurrency.html#tymethod.repatriate_reserved)
			//         Check `reserved_balance()` to make sure that the fee is payable via this method.
			//         Maybe use a different payment method and check `total_balance()`. I don't know
			//         Substrate's Currency module well enough to tell.
			// REVIEW: This happens *after* the request is `take`n from storage. Is that intended?
			//         See ["verify first, write last"](https://substrate.dev/recipes/2-appetizers/1-hello-substrate.html#inside-a-dispatchable-call) motto.
			// TODO check whether to use BalanceStatus::Reserved or Free?
			T::Currency::repatriate_reserved(&who, &operator, fee, BalanceStatus::Free)?;

			// Dispatch the result to the original callback registered by the caller
			// TODO fix the "?" - not sure how to proceed there
			callback[0].with_result(result.clone()).ok_or(Error::<T>::UnknownCallback)?.dispatch_bypass_filter(frame_system::RawOrigin::Root.into()).ok();
			// callback[0].with_result(result.clone()).ok_or(Error::<T>::UnknownCallback)?.dispatch(frame_system::RawOrigin::Root.into())?;

			Self::deposit_event(RawEvent::OracleAnswer(operator, request_id, who, result, fee));

			Ok(())
		}

		// Identify requests that are considered dead and remove them
		fn on_finalize(n: T::BlockNumber) {
			for (request_identifier, (_account_id, _data, block_number, _fee)) in Requests::<T>::iter() {
				if n > block_number + T::ValidityPeriod::get() {
					// No result has been received in time
					Requests::<T>::remove(request_identifier);

					Self::deposit_event(RawEvent::KillRequest(request_identifier));
				}
			}
		}

	}
}
