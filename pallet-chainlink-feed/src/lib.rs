//! # Chainlink Price Feed Module

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod tests;

use sp_std::prelude::*;

#[warn(unused_imports)]
use codec::{Codec, Decode, Encode};
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, Parameter, RuntimeDebug, dispatch::{DispatchResult, DispatchError}};
use frame_support::storage::{with_transaction, TransactionOutcome};
use frame_support::dispatch::HasCompact;
use frame_support::dispatch::DispatchResultWithPostInfo;
use frame_support::traits::{Get, ReservableCurrency};
use frame_support::weights::Weight;
use frame_system::{ensure_signed, RawOrigin};
use sp_arithmetic::traits::BaseArithmetic;
use sp_runtime::traits::Dispatchable;
use sp_runtime::traits::AtLeast32BitUnsigned;
use sp_runtime::traits::Member;
use sp_runtime::traits::One;
use sp_runtime::traits::Zero;
use sp_runtime::traits::CheckedAdd;
use sp_runtime::traits::Saturating;

/// Execute the supplied function in a new storage transaction.
///
/// All changes to storage performed by the supplied function are discarded if
/// the returned outcome is `Result::Err`.
///
/// Transactions can be nested to any depth. Commits happen to the parent
/// transaction.
// TODO: remove after move to Substrate v3 (once the semantics of #[transactional] work as intended)
pub fn with_transaction_result<R, E>(f: impl FnOnce() -> Result<R, E>) -> Result<R, E> {
	with_transaction(|| {
		let res = f();
		if res.is_ok() {
			TransactionOutcome::Commit(res)
		} else {
			TransactionOutcome::Rollback(res)
		}
	})
}


pub trait WeightInfo {
	fn create_feed() -> Weight;
	fn submit() -> Weight;
}

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	/// The units in which we record balances.
	type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;

	type FeedId: Member + Parameter + Default + Copy + HasCompact + BaseArithmetic;

	type RoundId: Member + Parameter + Default + Copy + HasCompact + BaseArithmetic;

	type Value: Member + Parameter + Default + Copy + HasCompact + PartialEq + PartialOrd;

	type Currency: ReservableCurrency<Self::AccountId>;

	/// Weight information for extrinsics in this pallet.
	type WeightInfo: WeightInfo;
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug)]
pub struct FeedConfig<
	Balance: Parameter,
	BlockNumber: Parameter,
	RoundId: Parameter,
	Value: Parameter,
> {
	payment_amount: Balance,
	timeout: BlockNumber,
	min_submission: Value,
	max_submission: Value,
	decimals: u8,
	description: Vec<u8>,
	restart_delay: RoundId,
	reporting_round_id: RoundId,
	latest_round_id: RoundId,
	oracle_count: u16
}
type FeedConfigOf<T> = FeedConfig<
	<T as Trait>::Balance,
	<T as frame_system::Trait>::BlockNumber,
	<T as Trait>::RoundId,
	<T as Trait>::Value
>;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug)]
pub struct Round<
	BlockNumber: Parameter,
	RoundId: Parameter,
	Value: Parameter,
> {
	answer: Value,
	started_at: BlockNumber,
	updated_at: BlockNumber,
	answered_in_round: RoundId,
}
type RoundOf<T> = Round<
	<T as frame_system::Trait>::BlockNumber,
	<T as Trait>::RoundId,
	<T as Trait>::Value,
>;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug)]
pub struct RoundDetails<
	Balance: Parameter,
	BlockNumber: Parameter,
	Value: Parameter,
> {
	submissions: Vec<Value>,
	max_submissions: u32,
	min_submissions: u32,
	timeout: BlockNumber,
	payment_amount: Balance,
}
type RoundDetailsOf<T> = RoundDetails<
	<T as Trait>::Balance,
	<T as frame_system::Trait>::BlockNumber,
	<T as Trait>::Value,
>;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug)]
pub struct OracleStatus<
	AccountId: Parameter,
	Balance: Parameter,
	RoundId: Parameter,
> {
    withdrawable: Balance,
    starting_round: RoundId,
    ending_round: Option<RoundId>,
    last_reported_round: Option<RoundId>,
    last_started_round: Option<RoundId>,
    admin: AccountId,
}
type OracleStatusOf<T> = OracleStatus<
	<T as frame_system::Trait>::AccountId,
	<T as Trait>::Balance,
	<T as Trait>::RoundId,
>;

decl_storage! {
    trait Store for Module<T: Trait> as ChainlinkFeed {
		/// A running counter used internally to determine the next feed id
		pub FeedCounter get(fn feed_counter): T::FeedId;

		/// Configuration for a feed.
		pub FeedConfigs get(fn feed_config): map hasher(twox_64_concat) T::FeedId => Option<FeedConfigOf<T>>;

		/// User-facing round data.
		pub Rounds get(fn round): double_map hasher(twox_64_concat) T::FeedId, hasher(twox_64_concat) T::RoundId => Option<RoundOf<T>>;

		/// Operator-facing round data.
		pub Details get(fn round_details): double_map hasher(twox_64_concat) T::FeedId, hasher(twox_64_concat) T::RoundId => Option<RoundDetailsOf<T>>;

		pub Oracles get(fn oracle): map hasher(blake2_128_concat) T::AccountId => Option<OracleStatusOf<T>>;

		// TODO store latest submission per feed?
		// latest_submission: Option<Value>,

		pub PendingAdmin get(fn pending_admin): map hasher(blake2_128_concat) T::AccountId => Option<T::AccountId>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId, FeedId = <T as Trait>::FeedId {
		/// A new oracle feed was created. \[feed_id, creator\]
		FeedCreated(FeedId, AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		/// A math operation lead to an overflow.
		Overflow,
		/// Given account id is not an oracle
		NotOracle,
		/// The oracle cannot submit as it is not enabled yet.
		OracleNotEnabled,
		/// The oracle has an ending round lower than the current round.
		OracleDisabled,
		/// The oracle cannot report for past rounds.
		ReportingOrder,
		/// Requested feed not present.
		FeedNotFound,
		/// Requested round not present.
		RoundNotFound,
		/// Oracle submission is below the minimum value.
		SubmissionBelowMinimum,
		/// Oracle submission is above the maximum value.
		SubmissionAboveMaximum,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		// Creates a new oracle feed with the given config values.
		// TODO: weights
		#[weight = 100]
		pub fn create_feed(
			origin,
			payment_amount: T::Balance,
			timeout: T::BlockNumber,
			min_submission: T::Value,
			max_submission: T::Value,
			decimals: u8,
			description: Vec<u8>,
			restart_delay: T::RoundId,
			oracles: Vec<T::AccountId>,
		) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;

			let id: T::FeedId = FeedCounter::<T>::get();
			let new_id = id.checked_add(&One::one()).ok_or(Error::<T>::Overflow)?;
			FeedCounter::<T>::put(new_id);

			FeedConfigs::<T>::insert(id, FeedConfig {
				payment_amount,
				timeout,
				min_submission,
				max_submission,
				decimals,
				description,
				restart_delay,
				latest_round_id: T::RoundId::zero(),
				reporting_round_id: T::RoundId::zero(),
				oracle_count: oracles.len() as u16
			});
			Self::deposit_event(RawEvent::FeedCreated(id, owner));
			Ok(().into())
		}

		// TODO: unfinished
		#[weight = 100]
		pub fn submit(
			origin,
			feed_id: T::FeedId,
			round_id: T::RoundId,
			submission: T::Value,
		) -> DispatchResultWithPostInfo {
			let oracle = ensure_signed(origin)?;

			Self::ensure_round_valid_for(feed_id, &oracle, round_id)?;

			let feed = FeedConfigs::<T>::get(feed_id).ok_or(Error::<T>::FeedNotFound)?;
			ensure!(submission >= feed.min_submission, Error::<T>::SubmissionBelowMinimum);
			ensure!(submission <= feed.max_submission, Error::<T>::SubmissionAboveMaximum);

			let new_round_id = feed.reporting_round_id.checked_add(&One::one()).ok_or(Error::<T>::Overflow)?;
			let mut oracle_status = Self::oracle(&oracle).ok_or(Error::<T>::NotOracle)?;
			let next_eligible_round = oracle_status.last_started_round
				.unwrap_or(Zero::zero())
				.checked_add(&feed.restart_delay).ok_or(Error::<T>::Overflow)?
				.checked_add(&One::one()).ok_or(Error::<T>::Overflow)?;

			with_transaction_result(|| -> DispatchResultWithPostInfo {
				if round_id == new_round_id
					&& oracle_status.last_started_round.is_some()
					&& round_id >= next_eligible_round {

					Self::initialize_round(feed_id, round_id)?;

					oracle_status.last_started_round = Some(round_id);
					Oracles::<T>::insert(&oracle, oracle_status);
				}

				Ok(().into())
			})
		}
	}
}

impl<T: Trait> Module<T> {
	fn ensure_round_valid_for(_feed: T::FeedId, acc: &T::AccountId, round_id: T::RoundId) -> DispatchResult {
		let o = Self::oracle(acc).ok_or(Error::<T>::NotOracle)?;

		ensure!(o.starting_round > round_id, Error::<T>::OracleNotEnabled);
		ensure!(o.ending_round.map(|e| e < round_id).unwrap_or(true), Error::<T>::OracleDisabled);
		ensure!(o.last_reported_round.map(|l| l > round_id).unwrap_or(false), Error::<T>::ReportingOrder);
		// TODO: port solidity
		// 	if (_roundId != rrId && _roundId != rrId.add(1) && !previousAndCurrentUnanswered(_roundId, rrId)) return "invalid round to report";
		// if (_roundId != 1 && !supersedable(_roundId.sub(1))) return "previous round not supersedable";
		Ok(())
	}

	/// Initialize a new round.
	///
	/// **Warning:** Fallible function that changes storage.
	fn initialize_round(feed: T::FeedId, new_round: T::RoundId) -> DispatchResult {

		let prev_round = new_round.saturating_sub(One::one());
		if Self::timed_out(feed, prev_round) {
			Self::close_timed_out_round(feed, prev_round)?;
		}

		Ok(())
	}

	/// Check whether a round is timed out. Returns `false` for
	/// rounds not present in storage.
	fn timed_out(feed: T::FeedId, round_id: T::RoundId) -> bool {
		// Assumption: returning false for non-existent rounds is fine.
		let started_at = Self::round(feed, round_id).map(|r| r.started_at).unwrap_or(Zero::zero());
		let timeout = Self::round_details(feed, round_id).map(|d| d.timeout).unwrap_or(Zero::zero());
		let block_num = frame_system::Module::<T>::block_number();

		started_at > Zero::zero() && timeout > Zero::zero()
			&& started_at.checked_add(&timeout)
				.expect("started_at and timeout should have sane values -> no overflow; qed") < block_num
	}

	/// Close a timed out round and remove its details.
	///
	/// **Warning:** Fallible function that changes storage.
	// TODO: use [require_transactional](https://github.com/paritytech/substrate/issues/7004) after migrating to Substrate v3.
	fn close_timed_out_round(feed: T::FeedId, timed_out: T::RoundId) -> DispatchResult {
		let prev_id = timed_out.saturating_sub(One::one());
		let prev_round = Self::round(feed, prev_id).ok_or(Error::<T>::RoundNotFound)?;
		let mut timed_out_round = Self::round(feed, timed_out).ok_or(Error::<T>::RoundNotFound)?;
		timed_out_round.answer = prev_round.answer;
		timed_out_round.answered_in_round = prev_round.answered_in_round;
		timed_out_round.updated_at = frame_system::Module::<T>::block_number();

		Details::<T>::remove(feed, timed_out);

		Ok(())
	}
}
