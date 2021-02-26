//! # Chainlink Price Feed Module

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod tests;

use sp_std::prelude::*;

use codec::{Decode, Encode};
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, Parameter, RuntimeDebug, dispatch::{DispatchResult, DispatchError}};
use frame_support::storage::{with_transaction, TransactionOutcome};
use frame_support::dispatch::HasCompact;
use frame_support::dispatch::DispatchResultWithPostInfo;
use frame_support::traits::{Get, ReservableCurrency};
use frame_support::weights::Weight;
use frame_system::ensure_signed;
use sp_arithmetic::traits::BaseArithmetic;
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

/// Determine the median of a slice of values.
pub(crate) fn median<T: Copy + BaseArithmetic>(numbers: &mut [T]) -> T {
	numbers.sort_unstable();

	let mid = numbers.len() / 2;
	if numbers.len() % 2 == 0 {
		numbers[mid - 1].saturating_add(numbers[mid]) / 2.into()
	} else {
		numbers[mid]
	}
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

	type RoundId: Member + Parameter + Default + Copy + HasCompact + BaseArithmetic + Into<u64>;

	type Value: Member + Parameter + Default + Copy + HasCompact + PartialEq + BaseArithmetic;

	type Currency: ReservableCurrency<Self::AccountId>;

	/// Maximum allowed string length.
	type StringLimit: Get<u32>;

	type OracleCountLimit: Get<u32>;

	/// Weight information for extrinsics in this pallet.
	type WeightInfo: WeightInfo;
}

#[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug)]
pub struct FeedConfig<
	AccountId: Parameter,
	Balance: Parameter,
	BlockNumber: Parameter,
	RoundId: Parameter,
	Value: Parameter,
> {
	owner: AccountId,
	pending_owner: Option<AccountId>,
	submission_value_bounds: (Value, Value),
	submission_count_bounds: (u32, u32),
	payment_amount: Balance,
	timeout: BlockNumber,
	decimals: u8,
	description: Vec<u8>,
	restart_delay: RoundId,
	reporting_round: RoundId,
	latest_round: RoundId,
	oracle_count: u32
}
type FeedConfigOf<T> = FeedConfig<
	<T as frame_system::Trait>::AccountId,
	<T as Trait>::Balance,
	<T as frame_system::Trait>::BlockNumber,
	<T as Trait>::RoundId,
	<T as Trait>::Value
>;

#[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug)]
pub struct Round<
	BlockNumber: Parameter,
	RoundId: Parameter,
	Value: Parameter,
> {
	started_at: BlockNumber,
	answer: Option<Value>,
	updated_at: Option<BlockNumber>,
	answered_in_round: Option<RoundId>,
}
type RoundOf<T> = Round<
	<T as frame_system::Trait>::BlockNumber,
	<T as Trait>::RoundId,
	<T as Trait>::Value,
>;

#[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug)]
pub struct RoundDetails<
	Balance: Parameter,
	BlockNumber: Parameter,
	Value: Parameter,
> {
	submissions: Vec<Value>,
	submission_count_bounds: (u32, u32),
	payment_amount: Balance,
	timeout: BlockNumber,
}
type RoundDetailsOf<T> = RoundDetails<
	<T as Trait>::Balance,
	<T as frame_system::Trait>::BlockNumber,
	<T as Trait>::Value,
>;

#[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug)]
pub struct OracleMeta<
	AccountId: Parameter,
	Balance: Parameter,
> {
	withdrawable: Balance,
	admin: AccountId,
	pending_admin: Option<AccountId>,
}
type OracleMetaOf<T> = OracleMeta<
	<T as frame_system::Trait>::AccountId,
	<T as Trait>::Balance,
>;

#[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug)]
pub struct OracleStatus<
	RoundId: Parameter,
	Value: Parameter
> {
	starting_round: RoundId,
	ending_round: Option<RoundId>,
	last_reported_round: Option<RoundId>,
	last_started_round: Option<RoundId>,
	latest_submission: Option<Value>,
}
type OracleStatusOf<T> = OracleStatus<
	<T as Trait>::RoundId,
	<T as Trait>::Value,
>;

#[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug)]
pub struct Requester<RoundId: Parameter> {
	delay: RoundId,
	last_started_round: Option<RoundId>,
}
type RequesterOf<T> = Requester<<T as Trait>::RoundId>;

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

		pub Oracles get(fn oracle): map hasher(blake2_128_concat) T::AccountId => Option<OracleMetaOf<T>>;

		pub OracleStati get(fn oracle_status): double_map hasher(twox_64_concat) T::FeedId, hasher(blake2_128_concat) T::AccountId => Option<OracleStatusOf<T>>;

		pub Requesters get(fn requester): double_map hasher(twox_64_concat) T::FeedId, hasher(blake2_128_concat) T::AccountId => Option<RequesterOf<T>>;
	}
}

decl_event!(
	pub enum Event<T> where
		AccountId = <T as frame_system::Trait>::AccountId,
		BlockNumber = <T as frame_system::Trait>::BlockNumber,
		FeedId = <T as Trait>::FeedId,
		RoundId = <T as Trait>::RoundId,
		Value = <T as Trait>::Value,
	{
		/// A new oracle feed was created. \[feed_id, creator\]
		FeedCreated(FeedId, AccountId),
		/// A new round was started. \[new_round_id, initiator, started_at\]
		NewRound(FeedId, RoundId, AccountId, BlockNumber),
		/// A submission was recorded. \[feed_id, round_id, submission, oracle\]
		SubmissionReceived(FeedId, RoundId, Value, AccountId),
		/// The answer for the round was updated. \[feed_id, round_id, new_answer, updated_at_block\]
		AnswerUpdated(FeedId, RoundId, Value, BlockNumber),
		/// An admin change was requested for the given oracle. \[oracle, admin, pending_admin\]
		OracleAdminUpdateRequested(AccountId, AccountId, AccountId),
		/// The admin change was executed. \[oracle, new_admin\]
		OracleAdminUpdated(AccountId, AccountId),
		/// The submission permissions for the given feed and oralce have been updated. \[feed, oracle, enabled\]
		OraclePermissionsUpdated(FeedId, AccountId, bool),
		/// The requester permissions have been updated (set or removed). \[feed, requester, authorized, delays\]
		RequesterPermissionsSet(FeedId, AccountId, bool, RoundId),
		/// \[feed, old_owner, new_owner\]
		OwnerUpdateRequested(FeedId, AccountId, AccountId),
		/// \[feed, new_owner\]
		OwnerUpdated(FeedId, AccountId),
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
		RequesterNotFound,
		/// New round cannot be requested to supersede current round.
		RoundNotSupersedable,
		/// No oracle meta data found for the given account.
		OracleNotFound,
		NotAcceptingSubmissions,
		/// Oracle submission is below the minimum value.
		SubmissionBelowMinimum,
		/// Oracle submission is above the maximum value.
		SubmissionAboveMaximum,
		/// The description string is too long.
		DescriptionTooLong,
		/// Tried to add too many oracles.
		OraclesLimitExceeded,
		/// The oracle was already enabled.
		AlreadyEnabled,
		/// The oracle address cannot change its associated admin.
		OwnerCannotChangeAdmin,
		/// Only the owner of a feed can change the configuration.
		NotFeedOwner,
		NotPendingOwner,
		/// The specified min/max pair was invalid.
		WrongBounds,
		/// The maximum number of oracles cannot exceed the amount of available oracles.
		MaxExceededTotal,
		/// The round initiation delay cannot be equal to or greater
		/// than the number of oracles.
		DelayExceededTotal,
		/// Sender is not admin. Admin privilege can only be transfered by the admin.
		NotAdmin,
		/// Only the pending admin can accept the transfer.
		NotPendingAdmin,
		CannotRequestRoundYet,
		NotAuthorizedRequester,
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
			submission_value_bounds: (T::Value, T::Value),
			submission_count_bounds: (u32, u32),
			decimals: u8,
			description: Vec<u8>,
			restart_delay: T::RoundId,
			oracles: Vec<(T::AccountId, T::AccountId)>,
		) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			ensure!(description.len() as u32 <= T::StringLimit::get(), Error::<T>::DescriptionTooLong);

			with_transaction_result(|| -> DispatchResultWithPostInfo {
				let id: T::FeedId = FeedCounter::<T>::get();
				let new_id = id.checked_add(&One::one()).ok_or(Error::<T>::Overflow)?;
				FeedCounter::<T>::put(new_id);

				let mut new_feed = FeedConfig {
					owner: owner.clone(),
					pending_owner: None,
					payment_amount,
					timeout,
					submission_value_bounds,
					submission_count_bounds,
					decimals,
					description,
					restart_delay,
					latest_round: Zero::zero(),
					reporting_round: Zero::zero(),
					oracle_count: Zero::zero(),
				};
				Self::add_oracles(&mut new_feed, id, oracles)?;
				FeedConfigs::<T>::insert(id, new_feed);
				Self::deposit_event(RawEvent::FeedCreated(id, owner));
				Ok(().into())
			})
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
			let (min_val, max_val) = feed.submission_value_bounds;
			ensure!(submission >= min_val, Error::<T>::SubmissionBelowMinimum);
			ensure!(submission <= max_val, Error::<T>::SubmissionAboveMaximum);

			let new_round_id = feed.reporting_round.checked_add(&One::one()).ok_or(Error::<T>::Overflow)?;
			let mut oracle_status = Self::oracle_status(feed_id, &oracle).ok_or(Error::<T>::NotOracle)?;
			let next_eligible_round = oracle_status.last_started_round
				.unwrap_or(Zero::zero())
				.checked_add(&feed.restart_delay).ok_or(Error::<T>::Overflow)?
				.checked_add(&One::one()).ok_or(Error::<T>::Overflow)?;
			let eligible_to_start = round_id >= next_eligible_round
				|| oracle_status.last_started_round.is_none();
			with_transaction_result(|| -> DispatchResultWithPostInfo {
				// initialize the round if conditions are met
				if round_id == new_round_id && eligible_to_start {
					let started_at = Self::initialize_round(feed_id, &feed, new_round_id)?;

					Self::deposit_event(RawEvent::NewRound(feed_id, new_round_id, oracle.clone(), started_at));

					oracle_status.last_started_round = Some(new_round_id);
				}

				// record submission
				let mut details = Details::<T>::take(feed_id, round_id).ok_or(Error::<T>::NotAcceptingSubmissions)?;
				details.submissions.push(submission);

				oracle_status.last_reported_round = Some(round_id);
				oracle_status.latest_submission = Some(submission);
				OracleStati::<T>::insert(feed_id, &oracle, oracle_status);
				Self::deposit_event(RawEvent::SubmissionReceived(feed_id, round_id, submission, oracle));

				// update round answer
				let (min_count, max_count) = details.submission_count_bounds;
				if details.submissions.len() >= min_count as usize {
					let new_answer = median(&mut details.submissions);
					let mut round = Self::round(feed_id, round_id).ok_or(Error::<T>::RoundNotFound)?;
					round.answer = Some(new_answer);
					let updated_at = frame_system::Module::<T>::block_number();
					round.updated_at = Some(updated_at);
					round.answered_in_round = Some(round_id);

					Self::deposit_event(RawEvent::AnswerUpdated(feed_id, round_id, new_answer, updated_at));
				}

				// pay oracle
				// uint128 payment = details[_roundId].paymentAmount;
				// Funds memory funds = recordedFunds;
				// funds.available = funds.available.sub(payment);
				// funds.allocated = funds.allocated.add(payment);
				// recordedFunds = funds;
				// oracles[msg.sender].withdrawable = oracles[msg.sender].withdrawable.add(payment);

				// emit AvailableFundsUpdated(funds.available);

				// delete the details if the maximum count has been reached
				if details.submissions.len() < max_count as usize {
					Details::<T>::insert(feed_id, round_id, details);
				}

				// TODO: answer validation

				Ok(().into())
			})
		}

		#[weight = 100]
		pub fn change_oracles(
			origin,
			feed_id: T::FeedId,
			to_disable: Vec<T::AccountId>,
			to_add: Vec<(T::AccountId, T::AccountId)>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let mut feed = FeedConfigs::<T>::get(feed_id).ok_or(Error::<T>::FeedNotFound)?;
			ensure!(feed.owner == sender, Error::<T>::NotFeedOwner);
			let mut to_disable = to_disable;
			to_disable.sort();
			to_disable.dedup();
			with_transaction_result(|| -> DispatchResultWithPostInfo {
				let disabled_count = to_disable.len() as u32;
				debug_assert!(feed.oracle_count >= disabled_count);
				// This should be fine as we assert on every oracle
				// in the loop that it exists and we deduplicate.
				feed.oracle_count = feed.oracle_count.saturating_sub(disabled_count);
				for d in to_disable {
					// disable
					let mut status = Self::oracle_status(feed_id, &d).ok_or(Error::<T>::OracleNotFound)?;
					// Is this check necessary?
					ensure!(status.ending_round.is_none(), Error::<T>::OracleDisabled);
					status.ending_round = Some(feed.reporting_round);
					OracleStati::<T>::insert(feed_id, &d, status);
					Self::deposit_event(RawEvent::OraclePermissionsUpdated(feed_id, d, false));
				}

				Self::add_oracles(&mut feed, feed_id, to_add)?;

				FeedConfigs::<T>::insert(feed_id, feed);

				Ok(().into())
			})
		}

		#[weight = 100]
		pub fn update_future_rounds(
			origin,
			feed_id: T::FeedId,
			payment_amount: T::Balance,
			submission_count_bounds: (u32, u32),
			restart_delay: T::RoundId,
			timeout: T::BlockNumber,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let (min, max) = submission_count_bounds;
			ensure!(max >= min, Error::<T>::WrongBounds);
			let mut feed = FeedConfigs::<T>::get(feed_id).ok_or(Error::<T>::FeedNotFound)?;
			ensure!(feed.owner == sender, Error::<T>::NotFeedOwner);
			// Make sure that both the min and max of submissions is
			// less or equal to the number of oracles.
			ensure!(feed.oracle_count >= max, Error::<T>::MaxExceededTotal);
			// Make sure that at least one oracle can request a new
			// round.
			ensure!(feed.oracle_count as u64 > restart_delay.into(), Error::<T>::DelayExceededTotal);
			// require(recordedFunds.available >= requiredReserve(_paymentAmount), "insufficient funds for payment");
			// if (oracleCount() > 0) {
			// 	require(_minSubmissions > 0, "min must be greater than 0");
			// }

			feed.payment_amount = payment_amount;
			feed.submission_count_bounds = submission_count_bounds;
			feed.restart_delay = restart_delay;
			feed.timeout = timeout;

			FeedConfigs::<T>::insert(feed_id, feed);

			// emit RoundDetailsUpdated(
			// 	paymentAmount,
			// 	_minSubmissions,
			// 	_maxSubmissions,
			// 	_restartDelay,
			// 	_timeout
			// );

			Ok(().into())
		}

		/// Initiate an admin transfer for the given oracle.
		#[weight = 100]
		pub fn transfer_admin(
			origin,
			oracle: T::AccountId,
			new_admin: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let old_admin = ensure_signed(origin)?;
			let mut oracle_meta = Self::oracle(&oracle).ok_or(Error::<T>::OracleNotFound)?;

			ensure!(oracle_meta.admin == old_admin, Error::<T>::NotAdmin);

			oracle_meta.pending_admin = Some(new_admin.clone());
			Oracles::<T>::insert(&oracle, oracle_meta);

			Self::deposit_event(RawEvent::OracleAdminUpdateRequested(oracle, old_admin, new_admin));

			Ok(().into())
		}

		/// Complete an admin transfer for the given oracle.
		#[weight = 100]
		pub fn accept_admin(
			origin,
			oracle: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let new_admin = ensure_signed(origin)?;
			let mut oracle_meta = Self::oracle(&oracle).ok_or(Error::<T>::OracleNotFound)?;

			ensure!(oracle_meta.pending_admin.filter(|p| p == &new_admin).is_some(), Error::<T>::NotPendingAdmin);

			oracle_meta.pending_admin = None;
			oracle_meta.admin = new_admin.clone();
			Oracles::<T>::insert(&oracle, oracle_meta);

			Self::deposit_event(RawEvent::OracleAdminUpdated(oracle, new_admin));

			Ok(().into())
		}

		#[weight = 100]
		pub fn request_new_round(
			origin,
			feed_id: T::FeedId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let mut requester = Self::requester(feed_id, &sender).ok_or(Error::<T>::NotAuthorizedRequester)?;
			let feed = Self::feed_config(feed_id).ok_or(Error::<T>::FeedNotFound)?;
			let is_first_round_or_updated = if feed.reporting_round == Zero::zero() {
				true
			} else {
				let round = Self::round(feed_id, feed.reporting_round).ok_or(Error::<T>::RoundNotFound)?;
				round.updated_at.is_some()
			};

			ensure!(is_first_round_or_updated || Self::timed_out(feed_id, feed.reporting_round), Error::<T>::RoundNotSupersedable);

			let new_round = feed.reporting_round.checked_add(&One::one()).ok_or(Error::<T>::Overflow)?;
			let last_started = requester.last_started_round.unwrap_or(Zero::zero());
			let next_allowed_round = last_started.checked_add(&requester.delay).ok_or(Error::<T>::Overflow)?;
			ensure!(requester.last_started_round.is_none() || new_round > next_allowed_round, Error::<T>::CannotRequestRoundYet);

			with_transaction_result(|| -> DispatchResultWithPostInfo {
				let started_at = Self::initialize_round(feed_id, &feed, new_round)?;

				requester.last_started_round = Some(new_round);
				Requesters::<T>::insert(feed_id, &sender, requester);

				Self::deposit_event(RawEvent::NewRound(feed_id, new_round, sender, started_at));

				Ok(().into())
			})
		}

		#[weight = 100]
		pub fn set_requester(
			origin,
			feed_id: T::FeedId,
			requester: T::AccountId,
			delay: T::RoundId,
		) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			let feed = Self::feed_config(feed_id).ok_or(Error::<T>::FeedNotFound)?;
			ensure!(feed.owner == owner, Error::<T>::NotFeedOwner);

			let mut requester_meta = Self::requester(feed_id, &requester).unwrap_or_default();
			requester_meta.delay = delay;
			Requesters::<T>::insert(feed_id, &requester, requester_meta);

			Self::deposit_event(RawEvent::RequesterPermissionsSet(feed_id, requester, true, delay));

			Ok(().into())
		}

		#[weight = 100]
		pub fn remove_requester(
			origin,
			feed_id: T::FeedId,
			requester: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			let feed = Self::feed_config(feed_id).ok_or(Error::<T>::FeedNotFound)?;
			ensure!(feed.owner == owner, Error::<T>::NotFeedOwner);

			let requester_meta = Requesters::<T>::take(feed_id, &requester).ok_or(Error::<T>::RequesterNotFound)?;

			Self::deposit_event(RawEvent::RequesterPermissionsSet(feed_id, requester, false, requester_meta.delay));

			Ok(().into())
		}

		#[weight = 100]
		pub fn transfer_ownership(
			origin,
			feed_id: T::FeedId,
			new_owner: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let old_owner = ensure_signed(origin)?;
			let mut feed = Self::feed_config(feed_id).ok_or(Error::<T>::FeedNotFound)?;

			ensure!(feed.owner == old_owner, Error::<T>::NotFeedOwner);

			feed.pending_owner = Some(new_owner.clone());
			FeedConfigs::<T>::insert(feed_id, feed);

			Self::deposit_event(RawEvent::OwnerUpdateRequested(feed_id, old_owner, new_owner));

			Ok(().into())
		}

		#[weight = 100]
		pub fn accept_ownership(
			origin,
			feed_id: T::FeedId,
		) -> DispatchResultWithPostInfo {
			let new_owner = ensure_signed(origin)?;
			let mut feed = Self::feed_config(feed_id).ok_or(Error::<T>::FeedNotFound)?;

			ensure!(feed.pending_owner.filter(|p| p == &new_owner).is_some(), Error::<T>::NotPendingOwner);

			feed.pending_owner = None;
			feed.owner = new_owner.clone();
			FeedConfigs::<T>::insert(feed_id, feed);

			Self::deposit_event(RawEvent::OwnerUpdated(feed_id, new_owner));

			Ok(().into())
		}
	}
}

impl<T: Trait> Module<T> {
	fn ensure_round_valid_for(feed: T::FeedId, acc: &T::AccountId, round_id: T::RoundId) -> DispatchResult {
		let o = Self::oracle_status(feed, acc).ok_or(Error::<T>::NotOracle)?;

		ensure!(o.starting_round <= round_id, Error::<T>::OracleNotEnabled);
		ensure!(o.ending_round.map(|e| e >= round_id).unwrap_or(true), Error::<T>::OracleDisabled);
		ensure!(o.last_reported_round.map(|l| l < round_id).unwrap_or(true), Error::<T>::ReportingOrder);
		// TODO: port solidity
		// 	if (_roundId != rrId && _roundId != rrId.add(1) && !previousAndCurrentUnanswered(_roundId, rrId)) return "invalid round to report";
		// if (_roundId != 1 && !supersedable(_roundId.sub(1))) return "previous round not supersedable";
		Ok(())
	}

	/// Initialize a new round.
	///
	/// **Warning:** Fallible function that changes storage.
	fn initialize_round(feed_id: T::FeedId, feed: &FeedConfigOf<T>, new_round_id: T::RoundId) -> Result<T::BlockNumber, DispatchError> {

		let prev_round_id = new_round_id.saturating_sub(One::one());
		if Self::timed_out(feed_id, prev_round_id) {
			Self::close_timed_out_round(feed_id, prev_round_id)?;
		}

		// reportingRoundId = _roundId;
		Details::<T>::insert(feed_id, new_round_id, RoundDetails {
			submissions: Vec::new(),
			submission_count_bounds: feed.submission_count_bounds,
			payment_amount: feed.payment_amount,
			timeout: feed.timeout,
		});
		let started_at = frame_system::Module::<T>::block_number();
		let round = Round { started_at, ..Default::default() };
		Rounds::<T>::insert(feed_id, new_round_id, round);

		Ok(started_at)
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
		let updated_at = frame_system::Module::<T>::block_number();
		timed_out_round.updated_at = Some(updated_at);

		Details::<T>::remove(feed, timed_out);

		Ok(())
	}

	/// Add the given oracles to the given feed.
	///
	/// **Warning:** Fallible function that changes storage.
	fn add_oracles(
		feed: &mut FeedConfigOf<T>,
		feed_id: T::FeedId,
		to_add: Vec<(T::AccountId, T::AccountId)>,
	) -> DispatchResult {
		let new_count = feed.oracle_count.checked_add(to_add.len() as u32).ok_or(Error::<T>::Overflow)?;
		ensure!(new_count <= T::OracleCountLimit::get(), Error::<T>::OraclesLimitExceeded);
		feed.oracle_count = new_count;
		for (oracle, admin) in to_add {
			Oracles::<T>::try_mutate(&oracle, |maybe_meta| -> DispatchResult {
				match maybe_meta {
					None => {
						*maybe_meta = Some(OracleMeta {
							withdrawable: Zero::zero(),
							admin,
							..Default::default()
						});
					},
					Some(meta) => ensure!(meta.admin == admin, Error::<T>::OwnerCannotChangeAdmin)
				}
				Ok(())
			})?;
			OracleStati::<T>::try_mutate(feed_id, &oracle, |maybe_status| -> DispatchResult {
				ensure!(maybe_status.as_ref().map(|s| s.ending_round.is_some()).unwrap_or(true), Error::<T>::AlreadyEnabled);
				*maybe_status = Some(OracleStatus {
					starting_round: feed.reporting_round,
					..Default::default()
				});
				Ok(())
			})?;
			Self::deposit_event(RawEvent::OraclePermissionsUpdated(feed_id, oracle, true));
		}

		Ok(())
	}
}
