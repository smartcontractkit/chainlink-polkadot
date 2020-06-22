//! # A pallet to interact with Chainlink nodes
//!
//! \## Overview
//!
//! `pallet-chainlink` allows to request external data from chainlink operators. This is done by emiting a well-known event (`OracleEvent`)
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

use sp_std::if_std;
#[warn(unused_imports)]
use codec::{Codec, Decode, Encode};
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, Parameter, dispatch::DispatchResult};
use frame_support::traits::{Get, ReservableCurrency, Currency};
use sp_std::prelude::*;
use sp_runtime::traits::Dispatchable;
use frame_system::{ensure_signed, RawOrigin};
use frame_system as system;

// A trait allowing to inject Operator results back into the specified Call
pub trait CallbackWithParameter {
	fn with_result(&self, result: Vec<u8>) -> Option<Self> where Self: core::marker::Sized;
}

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type Currency: ReservableCurrency<Self::AccountId>;

	// A reference to an Extrinsic that can have a result injected. Used as Chainlink callback
	type Callback: Parameter + Dispatchable<Origin = Self::Origin> + Codec + Eq + CallbackWithParameter;

	// Period during which a request is valid
	type ValidityPeriod: Get<Self::BlockNumber>;
}

// REVIEW: Use this for transfering currency.
type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

// Uniquely identify a request's specification understood by an Operator
pub type SpecIndex = Vec<u8>;
// Uniquely identify a request for a considered Operator
pub type RequestIdentifier = u64;
// The version of the serialized data format
pub type DataVersion = u64;

decl_storage! {
    trait Store for Module<T: Trait> as Chainlink {
		// A set of all registered Operator
		// TODO migrate to 'natural' hasher once migrated to 2.0
		pub Operators get(fn operator): map hasher(blake2_256) T::AccountId => bool;

		// A running counter used internally to identify the next request
		pub NextRequestIdentifier get(fn request_identifier): RequestIdentifier;

		// A map of details of each running request
		// TODO migrate to 'natural' hasher once migrated to 2.0
		// REVIEW: Consider using a struct for the Requests instead of a tuple to increase
		//         readability.
		pub Requests get(fn request): linked_map hasher(blake2_256) RequestIdentifier => (T::AccountId, Vec<T::Callback>, T::BlockNumber, BalanceOf<T>);
    }
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId, Balance = BalanceOf<T> {
		// A request has been accepted. Corresponding fee paiement is reserved
		OracleRequest(AccountId, SpecIndex, RequestIdentifier, AccountId, DataVersion, Vec<u8>, Vec<u8>, Balance),

		// A request has been answered. Corresponding fee paiement is transfered
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
	pub enum Error for Module<T: Trait> {
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
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		fn deposit_event() = default;

		// REVIEW: Use `///` instead of `//` to make these doc comments that are part of the crate documentation.
		// Register a new Operator.
		// Fails with `OperatorAlreadyRegistered` if this Operator (identified by `origin`) has already been registered.
		pub fn register_operator(origin) -> DispatchResult {
			let who : <T as frame_system::Trait>::AccountId = ensure_signed(origin)?;

			ensure!(!<Operators<T>>::exists(&who), Error::<T>::OperatorAlreadyRegistered);

			Operators::<T>::insert(&who, true);

			Self::deposit_event(RawEvent::OperatorRegistered(who));

			Ok(())
		}

		// Unregisters an existing Operator
		pub fn unregister_operator(origin) -> DispatchResult {
			let who : <T as frame_system::Trait>::AccountId = ensure_signed(origin)?;

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
		pub fn initiate_request(origin, operator: T::AccountId, spec_index: SpecIndex, data_version: DataVersion, data: Vec<u8>, fee: BalanceOf<T>, callback: <T as Trait>::Callback) -> DispatchResult {
			let who : <T as frame_system::Trait>::AccountId = ensure_signed(origin.clone())?;

			ensure!(<Operators<T>>::exists(&operator), Error::<T>::UnknownOperator);
			// REVIEW: Should probably be at least `ExistentialDeposit`
			ensure!(fee > BalanceOf::<T>::from(0), Error::<T>::InsufficientFee);

			T::Currency::reserve(&who, fee.into())?;

			let request_id = NextRequestIdentifier::get();
			// REVIEW: This can overflow. You can make a maximum of `u64::max_value()` requests.
			//         Default behavior for `u64` is to wrap around to 0, but you might want to
			//         make this explicit.
			//         I think using `wrapping_add` could be fine here, because it should be fine to
			//         start at 0 when you reach `u64::max_value()`.
			NextRequestIdentifier::put(request_id + 1);

			// REVIEW: Is it intentional that requests are only valid during the current block?
			let now = frame_system::Module::<T>::block_number();
			// REVIEW: You might want to think about and document that your requests can be overwritten
			//         as soon as the request id wraps around.
			// REVIEW: Is the `Vec` intended for forward compatibility? It seems superfluous here.
			Requests::<T>::insert(request_id.clone(), (operator.clone(), vec![callback], now, fee));

			Self::deposit_event(RawEvent::OracleRequest(operator, spec_index, request_id, who, data_version, data, "Chainlink.callback".into(), fee));

			Ok(())
		}

		// The callback used to be notified of all Operators results.
		// Only the Operator responsible for an identified request can notify back the result.
		// Result is then dispatched back to the originator's callback.
		// The fee reserved during `initiate_request` is transferred as soon as this callback is called.
        fn callback(origin, request_id: RequestIdentifier, result: Vec<u8>) -> DispatchResult {
			let who : <T as frame_system::Trait>::AccountId = ensure_signed(origin.clone())?;

			ensure!(<Requests<T>>::exists(&request_id), Error::<T>::UnknownRequest);
                        let (operator, callback, _, fee) = <Requests<T>>::get(&request_id);
			ensure!(operator == who, Error::<T>::WrongOperator);

			// REVIEW: This does not make sure that the fee is payed. `repatriate_reserved` removes
			//         *up to* the amount passed. [See here](https://substrate.dev/rustdocs/master/frame_support/traits/trait.ReservableCurrency.html#tymethod.repatriate_reserved)
			//         Check `reserved_balance()` to make sure that the fee is payable via this method.
			//         Maybe use a different payment method and check `total_balance()`. I don't know
			//         Substrate's Currency module well enough to tell.
			// REVIEW: This happens *after* the request is `take`n from storage. Is that intended?
			//         See ["verify first, write last"](https://substrate.dev/recipes/2-appetizers/1-hello-substrate.html#inside-a-dispatchable-call) motto.
			T::Currency::repatriate_reserved(&who, &operator, fee.into())?;

			// Dispatch the result to the original callback registered by the caller
			callback[0].with_result(result.clone()).ok_or(Error::<T>::UnknownCallback)?.dispatch(frame_system::RawOrigin::Root.into())?;

			Self::deposit_event(RawEvent::OracleAnswer(operator, request_id, who, result, fee));

            Ok(())
		}

		// Identify requests that are considered dead and remove them
		fn on_finalize(n: T::BlockNumber) {
			for (request_identifier, (_account_id, _data, block_number, _fee)) in Requests::<T>::enumerate() {
				if n > block_number + T::ValidityPeriod::get() {
					// No result has been received in time
					Requests::<T>::remove(request_identifier);

					Self::deposit_event(RawEvent::KillRequest(request_identifier));
				}
			}
		}

	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use frame_support::{impl_outer_event, impl_outer_origin, parameter_types, weights::Weight};
	use sp_core::H256;
	// The testing primitives are very useful for avoiding having to work with signatures
	// or public keys. `u64` is used as the `AccountId` and no `Signature`s are required.
	use sp_runtime::{
		Perbill,
		testing::Header,
		traits::{BlakeTwo256, OnFinalize, IdentityLookup},
	};
	use frame_system::{EventRecord, Phase};

	impl_outer_origin! {
		pub enum Origin for Runtime {}
	}

	#[derive(Clone, Eq, PartialEq, Debug)]
	pub struct Runtime;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::one();
	}
	impl frame_system::Trait for Runtime {
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Call = ();
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = TestEvent;
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
		type ModuleToIndex = ();
	}
	parameter_types! {
		pub const ExistentialDeposit: u64 = 1;
		pub const TransferFee: u64 = 0;
		pub const CreationFee: u64 = 0;
	}
	impl balances::Trait for Runtime {
		type Balance = u64;
		type OnFreeBalanceZero = ();
		type OnNewAccount = ();
		type DustRemoval = ();
		type TransferPayment = ();
		type Event = TestEvent;
		type ExistentialDeposit = ExistentialDeposit;
		type TransferFee = TransferFee;
		type CreationFee = CreationFee;
	}
	impl Trait for Runtime {
		type Event = TestEvent;
		type Currency = balances::Module<Runtime>;
		type Callback = module2::Call<Runtime>;
		type ValidityPeriod = ValidityPeriod;
	}
	impl module2::Trait for Runtime {
	}
	parameter_types! {
		pub const ValidityPeriod: u64 = 10;
	}

	mod chainlink {
		pub use crate::Event;
	}

	impl_outer_event! {
		pub enum TestEvent for Runtime {
			balances<T>,
			chainlink<T>,
		}
	}

	type System = frame_system::Module<Runtime>;

	fn new_test_ext() -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
		balances::GenesisConfig::<Runtime>{
			balances: vec![(1, 10), (2, 20)],
			vesting: vec![],
		}.assimilate_storage(&mut t).unwrap();
		balances::GenesisConfig::<Runtime>::default().assimilate_storage(&mut t).unwrap();
		t.into()
	}

	mod module2 {
		use super::*;

		pub trait Trait: frame_system::Trait {}

		frame_support::decl_module! {
			pub struct Module<T: Trait> for enum Call
				where origin: <T as frame_system::Trait>::Origin
			{
				pub fn callback(_origin, result: Vec<u8>) -> frame_support::dispatch::DispatchResult {
					let r : u128 = u128::decode(&mut &result[..]).map_err(|err| err.what())?;
					<Result>::put(r);
					Ok(())
				}
			}
		}

		frame_support::decl_storage! {
			trait Store for Module<T: Trait> as TestStorage {
				pub Result: u128;
			}
		}

		impl <T: Trait> CallbackWithParameter for Call<T> {
			fn with_result(&self, result: Vec<u8>) -> Option<Self> {
				match *self {
					Call::callback(_) => Some(Call::callback(result)),
					_ => None
				}
			}
		}

	}

	#[test]
	fn operators_can_be_registered() {
		new_test_ext().execute_with(|| {
			assert!(!<Operators<Runtime>>::exists(1));
			assert!(<Module<Runtime>>::register_operator(Origin::signed(1)).is_ok());
			assert!(<Operators<Runtime>>::exists(1));
			assert!(<Module<Runtime>>::unregister_operator(Origin::signed(1)).is_ok());
			assert!(!<Operators<Runtime>>::exists(1));
		});

		new_test_ext().execute_with(|| {
			assert!(<Module<Runtime>>::unregister_operator(Origin::signed(1)).is_err());
			assert!(!<Operators<Runtime>>::exists(1));
		});

	}

	#[test]
	fn initiate_requests() {

		new_test_ext().execute_with(|| {
			assert!(<Module<Runtime>>::register_operator(Origin::signed(1)).is_ok());
			assert!(<Module<Runtime>>::initiate_request(Origin::signed(2), 1, vec![], 1, vec![], 0, module2::Call::<Runtime>::callback(vec![]).into()).is_err());
		});

		new_test_ext().execute_with(|| {
			assert!(<Module<Runtime>>::initiate_request(Origin::signed(2), 1, vec![], 1, vec![], 1, module2::Call::<Runtime>::callback(vec![]).into()).is_err());
		});

		new_test_ext().execute_with(|| {
			assert!(<Module<Runtime>>::register_operator(Origin::signed(1)).is_ok());
			assert!(<Module<Runtime>>::initiate_request(Origin::signed(2), 1, vec![], 1, vec![], 2, module2::Call::<Runtime>::callback(vec![]).into()).is_ok());
			assert!(<Module<Runtime>>::callback(Origin::signed(3), 0, 10.encode()).is_err());
		});

		new_test_ext().execute_with(|| {
			assert!(<Module<Runtime>>::callback(Origin::signed(1), 0, 10.encode()).is_err());
		});

		new_test_ext().execute_with(|| {
			assert!(<Module<Runtime>>::register_operator(Origin::signed(1)).is_ok());

			assert_eq!(
				*System::events().last().unwrap(),
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: TestEvent::chainlink(RawEvent::OperatorRegistered(1)),
					topics: vec![],
				}
			);

			let parameters = ("a", "b");
			let data = parameters.encode();
			assert!(<Module<Runtime>>::initiate_request(Origin::signed(2), 1, vec![], 1, data.clone(), 2, module2::Call::<Runtime>::callback(vec![]).into()).is_ok());

			assert_eq!(
				*System::events().last().unwrap(),
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: TestEvent::chainlink(RawEvent::OracleRequest(1, vec![], 0, 2, 1, data.clone(), "Chainlink.callback".into(), 2)),
					topics: vec![],
				}
			);

			let r = <(Vec<u8>, Vec<u8>)>::decode(&mut &data[..]).unwrap().0;
			assert_eq!("a", std::str::from_utf8(&r).unwrap());

			let result = 10;
			assert!(<Module<Runtime>>::callback(Origin::signed(1), 0, result.encode()).is_ok());
			assert_eq!(module2::Result::get(), result);
		});

	}

	#[test]
	pub fn on_finalize() {

		new_test_ext().execute_with(|| {
			assert!(<Module<Runtime>>::register_operator(Origin::signed(1)).is_ok());
			assert!(<Module<Runtime>>::initiate_request(Origin::signed(2), 1, vec![], 1, vec![], 2, module2::Call::<Runtime>::callback(vec![]).into()).is_ok());
			<Module<Runtime> as OnFinalize<u64>>::on_finalize(20);
			// Request has been killed, too old
			assert!(<Module<Runtime>>::callback(Origin::signed(1), 0, 10.encode()).is_err());
		});

	}

}
