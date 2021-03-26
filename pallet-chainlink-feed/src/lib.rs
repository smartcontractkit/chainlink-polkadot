//! # Chainlink Price Feed Module

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod tests;

pub mod default_weights;
mod utils;

use sp_std::prelude::*;

use codec::{Decode, Encode};
use frame_support::traits::{Currency, ExistenceRequirement, Get, ReservableCurrency};
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage,
	dispatch::{DispatchError, DispatchResult, DispatchResultWithPostInfo, HasCompact},
	ensure,
	weights::Weight,
	Parameter, RuntimeDebug,
};
use frame_system::ensure_signed;
use sp_arithmetic::traits::BaseArithmetic;
use sp_runtime::{
	traits::{AccountIdConversion, CheckedAdd, CheckedSub, Member, One, Saturating, Zero},
	ModuleId,
};
use sp_std::convert::{TryFrom, TryInto};

use utils::{median, with_transaction_result};

pub type BalanceOf<T> =
	<<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

pub type RoundId = u32;

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	/// Type for feed indexing.
	type FeedId: Member + Parameter + Default + Copy + HasCompact + BaseArithmetic;

	/// Oracle feed values.
	type Value: Member + Parameter + Default + Copy + HasCompact + PartialEq + BaseArithmetic;

	/// Interface used for balance transfers.
	type Currency: ReservableCurrency<Self::AccountId>;

	/// The module id used to determine the account for storing the funds used to pay the oracles.
	type ModuleId: Get<ModuleId>;

	/// The minimum amount of funds that need to be present in the fund account.
	type MinimumReserve: Get<BalanceOf<Self>>;

	/// Maximum allowed string length.
	type StringLimit: Get<u32>;

	/// Maximum number of oracles per feed.
	type OracleCountLimit: Get<u32>;

	/// Maximum number of feeds.
	type FeedLimit: Get<Self::FeedId>;

	/// Number of rounds to keep around per feed.
	type PruningWindow: Get<RoundId>;

	/// The weight for this pallet's extrinsics.
	type WeightInfo: WeightInfo;
}

/// The configuration for an oracle feed.
#[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug)]
pub struct FeedConfig<
	AccountId: Parameter,
	Balance: Parameter,
	BlockNumber: Parameter,
	Value: Parameter,
> {
	owner: AccountId,
	pending_owner: Option<AccountId>,
	submission_value_bounds: (Value, Value),
	submission_count_bounds: (u32, u32),
	payment: Balance,
	timeout: BlockNumber,
	decimals: u8,
	description: Vec<u8>,
	restart_delay: RoundId,
	reporting_round: RoundId,
	latest_round: RoundId,
	first_valid_round: Option<RoundId>,
	oracle_count: u32,
}
pub type FeedConfigOf<T> = FeedConfig<
	<T as frame_system::Trait>::AccountId,
	BalanceOf<T>,
	<T as frame_system::Trait>::BlockNumber,
	<T as Trait>::Value,
>;

/// Round data relevant to consumers.
/// Will only be constructed once minimum amount of submissions have
/// been provided.
#[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug)]
pub struct Round<BlockNumber, Value> {
	started_at: BlockNumber,
	answer: Option<Value>,
	updated_at: Option<BlockNumber>,
	answered_in_round: Option<RoundId>,
}
pub type RoundOf<T> = Round<<T as frame_system::Trait>::BlockNumber, <T as Trait>::Value>;

impl<BlockNumber, Value> Round<BlockNumber, Value>
where
	BlockNumber: Default, // BlockNumber
	Value: Default,       // Value
{
	/// Create a new Round with the given starting block.
	fn new(started_at: BlockNumber) -> Self {
		Self {
			started_at,
			..Default::default()
		}
	}
}

/// Round data relevant to oracles.
#[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug)]
pub struct RoundDetails<Balance, BlockNumber, Value> {
	submissions: Vec<Value>,
	submission_count_bounds: (u32, u32),
	payment: Balance,
	timeout: BlockNumber,
}
pub type RoundDetailsOf<T> =
	RoundDetails<BalanceOf<T>, <T as frame_system::Trait>::BlockNumber, <T as Trait>::Value>;

/// Meta data tracking withdrawable rewards and admin for an oracle.
#[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug)]
pub struct OracleMeta<AccountId, Balance> {
	withdrawable: Balance,
	admin: AccountId,
	pending_admin: Option<AccountId>,
}
pub type OracleMetaOf<T> = OracleMeta<<T as frame_system::Trait>::AccountId, BalanceOf<T>>;

/// Meta data tracking the oracle status for a feed.
#[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug)]
pub struct OracleStatus<Value> {
	starting_round: RoundId,
	ending_round: Option<RoundId>,
	last_reported_round: Option<RoundId>,
	last_started_round: Option<RoundId>,
	latest_submission: Option<Value>,
}
pub type OracleStatusOf<T> = OracleStatus<<T as Trait>::Value>;

impl<Value> OracleStatus<Value>
where
	Value: Default,
{
	/// Create a new oracle status with the given `starting_round`.
	fn new(starting_round: RoundId) -> Self {
		Self {
			starting_round,
			..Default::default()
		}
	}
}

/// Used to store round requester permissions for accounts.
#[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug)]
pub struct Requester {
	delay: RoundId,
	last_started_round: Option<RoundId>,
}

/// Round data as served by the `FeedInterface`.
#[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug)]
pub struct RoundData<BlockNumber, Value> {
	pub started_at: BlockNumber,
	pub answer: Value,
	pub updated_at: BlockNumber,
	pub answered_in_round: RoundId,
}
pub type RoundDataOf<T> = RoundData<<T as frame_system::Trait>::BlockNumber, <T as Trait>::Value>;

/// Possible error when converting from `Round` to `RoundData`.
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug)]
pub enum RoundConversionError {
	MissingField,
}

impl<B, V> TryFrom<Round<B, V>> for RoundData<B, V> {
	type Error = RoundConversionError;

	fn try_from(r: Round<B, V>) -> Result<Self, Self::Error> {
		if r.answered_in_round.is_none() || r.answer.is_none() || r.updated_at.is_none() {
			return Err(RoundConversionError::MissingField);
		}
		Ok(Self {
			started_at: r.started_at,
			answer: r.answer.unwrap(),
			updated_at: r.updated_at.unwrap(),
			answered_in_round: r.answered_in_round.unwrap(),
		})
	}
}

/// Trait for interacting with the feeds in the pallet.
pub trait FeedOracle<T: Trait> {
	type FeedId: Parameter + BaseArithmetic;
	type Feed: FeedInterface<T>;
	type MutableFeed: MutableFeedInterface<T>;

	/// Return the interface for the given feed if it exists.
	fn feed(id: Self::FeedId) -> Option<Self::Feed>;

	fn feed_mut(id: Self::FeedId) -> Option<Self::MutableFeed>;
}

/// Trait for read-only access to a feed.
pub trait FeedInterface<T: Trait> {
	/// Returns the id of the first round that contains non-default data.
	fn first_valid_round(&self) -> Option<RoundId>;

	/// Returns the id of the latest oracle round.
	fn latest_round(&self) -> RoundId;

	/// Returns the data for a given round.
	/// Will return `None` if there is no data for the given round.
	fn data_at(&self, round: RoundId) -> Option<RoundData<T::BlockNumber, T::Value>>;

	/// Returns the latest data for the feed.
	/// Will always return data but may contain default data if there
	/// has not been a valid round, yet.
	/// Check `first_valid_round` to determine whether there is
	/// useful data yet.
	fn latest_data(&self) -> RoundData<T::BlockNumber, T::Value>;
}

/// Trait for read-write access to a feed.
pub trait MutableFeedInterface<T: Trait>: FeedInterface<T> {
	/// Request that a new oracle round be started.
	///
	/// **Warning:** Fallible function that changes storage.
	fn request_new_round(&mut self, requester: T::AccountId) -> DispatchResult;
}

decl_storage! {
	trait Store for Module<T: Trait> as ChainlinkFeed {
		/// The account controlling the funds for this pallet.
		pub PalletAdmin get(fn pallet_admin) config(): T::AccountId;
		// possible optimization: put together with admin?
		/// The account to set as future pallet admin.
		pub PendingPalletAdmin: Option<T::AccountId>;

		/// Tracks the amount of debt accrued by the pallet towards the oracles.
		pub Debt get(fn debt): BalanceOf<T>;

		/// A running counter used internally to determine the next feed id.
		pub FeedCounter get(fn feed_counter): T::FeedId;

		/// Configuration for a feed.
		pub Feeds get(fn feed_config): map hasher(twox_64_concat) T::FeedId => Option<FeedConfigOf<T>>;

		/// Accounts allowed to create feeds.
		pub FeedCreators: map hasher(blake2_128_concat) T::AccountId => Option<()>;

		/// User-facing round data.
		pub Rounds get(fn round): double_map hasher(twox_64_concat) T::FeedId, hasher(twox_64_concat) RoundId => Option<RoundOf<T>>;

		/// Operator-facing round data.
		pub Details get(fn round_details): double_map hasher(twox_64_concat) T::FeedId, hasher(twox_64_concat) RoundId => Option<RoundDetailsOf<T>>;

		/// Global oracle meta data including admin and withdrawable funds.
		pub Oracles get(fn oracle): map hasher(blake2_128_concat) T::AccountId => Option<OracleMetaOf<T>>;

		/// Feed local oracle status data.
		pub OracleStati get(fn oracle_status): double_map hasher(twox_64_concat) T::FeedId, hasher(blake2_128_concat) T::AccountId => Option<OracleStatusOf<T>>;

		/// Per-feed permissioning for starting new rounds.
		pub Requesters get(fn requester): double_map hasher(twox_64_concat) T::FeedId, hasher(blake2_128_concat) T::AccountId => Option<Requester>;
	} add_extra_genesis {
		config(feed_creators): Vec<T::AccountId>;
		build(|config: &GenesisConfig<T>| {
			for creator in &config.feed_creators {
				FeedCreators::<T>::insert(creator, ());
			}
		})
	}
}

pub type SubmissionBounds = (u32, u32);

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Trait>::AccountId,
		Balance = BalanceOf<T>,
		BlockNumber = <T as frame_system::Trait>::BlockNumber,
		FeedId = <T as Trait>::FeedId,
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
		/// The round details were updated. \[feed_id, payment, submission_count_bounds, restart_delay, timeout\]
		RoundDetailsUpdated(FeedId, Balance, SubmissionBounds, RoundId, BlockNumber),
		/// An admin change was requested for the given oracle. \[oracle, admin, pending_admin\]
		OracleAdminUpdateRequested(AccountId, AccountId, AccountId),
		/// The admin change was executed. \[oracle, new_admin\]
		OracleAdminUpdated(AccountId, AccountId),
		/// The submission permissions for the given feed and oralce have been updated. \[feed, oracle, enabled\]
		OraclePermissionsUpdated(FeedId, AccountId, bool),
		/// The requester permissions have been updated (set or removed). \[feed, requester, authorized, delays\]
		RequesterPermissionsSet(FeedId, AccountId, bool, RoundId),
		/// An owner change was requested for the given feed. \[feed, old_owner, new_owner\]
		OwnerUpdateRequested(FeedId, AccountId, AccountId),
		/// The owner change was executed. \[feed, new_owner\]
		OwnerUpdated(FeedId, AccountId),
		/// A pallet admin change was requested. \[old_pallet_admin, new_pallet_admin\]
		PalletAdminUpdateRequested(AccountId, AccountId),
		/// The pallet admin change was executed. \[new_admin\]
		PalletAdminUpdated(AccountId),
		/// The account is allowed to create feeds. \[new_creator\]
		FeedCreator(AccountId),
		/// The account is no longer allowed to create feeds. \[previously_creator\]
		FeedCreatorRemoved(AccountId),
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
		/// The specified account does not have requester permissions stored.
		RequesterNotFound,
		/// New round cannot be requested to supersede current round.
		RoundNotSupersedable,
		/// No oracle meta data found for the given account.
		OracleNotFound,
		/// Submissions are not accepted for the specified round.
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
		/// Only the pending owner of a feed can accept the transfer invitation.
		NotPendingOwner,
		/// The specified min/max pair was invalid.
		WrongBounds,
		/// The maximum number of oracles cannot exceed the amount of available oracles.
		MaxExceededTotal,
		/// The round initiation delay cannot be equal to or greater
		/// than the number of oracles.
		DelayNotBelowCount,
		/// Sender is not admin. Admin privilege can only be transfered by the admin.
		NotAdmin,
		/// Only the pending admin can accept the transfer.
		NotPendingAdmin,
		/// The requester cannot request a new round, yet.
		CannotRequestRoundYet,
		/// No requester permissions associated with the given account.
		NotAuthorizedRequester,
		/// Cannot withdraw funds.
		InsufficientFunds,
		/// Funds cannot be withdrawn as the reserve would be critically low.
		InsufficientReserve,
		/// Only the pallet admin account can call this extrinsic.
		NotPalletAdmin,
		/// Only the pending admin can accept the transfer.
		NotPendingPalletAdmin,
		/// Round zero is not allowed to be pruned.
		CannotPruneRoundZero,
		/// The given pruning bounds don't cause any pruning with the current state.
		NothingToPrune,
		/// There is no valid data, yet, so the conditions for pruning are not met.
		NoValidRoundYet,
		/// The pruning should leave the rounds story in a contiguous state (no gaps).
		PruneContiguously,
		/// The maximum number of feeds was reached.
		FeedLimitReached,
		/// The round cannot be superseded by a new round.
		NotSupersedable,
		/// The round cannot be started because it is not a valid new round.
		InvalidRound,
		/// The calling account is not allowed to create feeds.
		NotFeedCreator,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// The account used to pay oracles and manage the funds of this pallet.
		const FundAccount: T::AccountId = T::ModuleId::get().into_account();

		// TODO: weights

		// --- feed operations ---

		/// Create a new oracle feed with the given config values.
		/// Limited to feed creator accounts.
		#[weight = T::WeightInfo::create_feed(oracles.len() as u32)]
		pub fn create_feed(
			origin,
			payment: BalanceOf<T>,
			timeout: T::BlockNumber,
			submission_value_bounds: (T::Value, T::Value),
			min_submissions: u32,
			decimals: u8,
			description: Vec<u8>,
			restart_delay: RoundId,
			oracles: Vec<(T::AccountId, T::AccountId)>,
		) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			ensure!(FeedCreators::<T>::contains_key(&owner), Error::<T>::NotFeedCreator);
			ensure!(description.len() as u32 <= T::StringLimit::get(), Error::<T>::DescriptionTooLong);

			let submission_count_bounds = (min_submissions, oracles.len() as u32);

			with_transaction_result(|| -> DispatchResultWithPostInfo {
				let id: T::FeedId = FeedCounter::<T>::get();
				ensure!(id < T::FeedLimit::get(), Error::<T>::FeedLimitReached);
				let new_id = id.checked_add(&One::one()).ok_or(Error::<T>::Overflow)?;
				FeedCounter::<T>::put(new_id);

				let new_config = FeedConfig {
					owner: owner.clone(),
					pending_owner: None,
					payment,
					timeout,
					submission_value_bounds,
					submission_count_bounds,
					decimals,
					description,
					restart_delay,
					latest_round: Zero::zero(),
					reporting_round: Zero::zero(),
					first_valid_round: None,
					oracle_count: Zero::zero(),
				};
				let mut feed = Feed::<T>::new(id, new_config); // synced on drop
				let started_at = frame_system::Module::<T>::block_number();
				let updated_at = Some(started_at);
				// Store a dummy value for round 0 because we will not get useful data for
				// it, but need some seed data that future rounds can carry over.
				Rounds::<T>::insert(id, RoundId::zero(), Round {
					started_at,
					answer: Some(Zero::zero()),
					updated_at,
					answered_in_round: Some(Zero::zero())
				});
				feed.add_oracles(oracles)?;
				// validate the rounds config
				feed.update_future_rounds(payment, submission_count_bounds, restart_delay, timeout)?;
				Self::deposit_event(RawEvent::FeedCreated(id, owner));
				Ok(().into())
			})
		}

		/// Initiate the transfer of the feed to `new_owner`.
		#[weight = T::WeightInfo::transfer_ownership()]
		pub fn transfer_ownership(
			origin,
			feed_id: T::FeedId,
			new_owner: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let old_owner = ensure_signed(origin)?;
			let mut feed = Self::feed_config(feed_id).ok_or(Error::<T>::FeedNotFound)?;
			ensure!(feed.owner == old_owner, Error::<T>::NotFeedOwner);

			feed.pending_owner = Some(new_owner.clone());
			Feeds::<T>::insert(feed_id, feed);

			Self::deposit_event(RawEvent::OwnerUpdateRequested(feed_id, old_owner, new_owner));

			Ok(().into())
		}

		/// Accept the transfer of feed ownership.
		#[weight = T::WeightInfo::accept_ownership()]
		pub fn accept_ownership(
			origin,
			feed_id: T::FeedId,
		) -> DispatchResultWithPostInfo {
			let new_owner = ensure_signed(origin)?;
			let mut feed = Self::feed_config(feed_id).ok_or(Error::<T>::FeedNotFound)?;

			ensure!(feed.pending_owner.filter(|p| p == &new_owner).is_some(), Error::<T>::NotPendingOwner);

			feed.pending_owner = None;
			feed.owner = new_owner.clone();
			Feeds::<T>::insert(feed_id, feed);

			Self::deposit_event(RawEvent::OwnerUpdated(feed_id, new_owner));

			Ok(().into())
		}

		/// Submit a new value to the given feed and round.
		///
		/// - Will start a new round if there is no round for the id, yet,
		///   and a round can be started (at this time by this oracle).
		/// - Will update the round answer if minimum number of submissions
		///   has been reached.
		/// - Records the rewards incurred by the oracle.
		/// - Removes the details for the previous round if it was superseded.
		///
		/// Limited to the oracles of a feed.
		#[weight = T::WeightInfo::submit_opening_round_answers().max(
			T::WeightInfo::submit_closing_answer(T::OracleCountLimit::get())
		)]
		pub fn submit(
			origin,
			feed_id: T::FeedId,
			round_id: RoundId,
			submission: T::Value,
		) -> DispatchResultWithPostInfo {
			let oracle = ensure_signed(origin)?;

			with_transaction_result(|| -> DispatchResultWithPostInfo {
				let mut feed = Feed::<T>::load_from(feed_id).ok_or(Error::<T>::FeedNotFound)?;
				let mut oracle_status = Self::oracle_status(feed_id, &oracle).ok_or(Error::<T>::NotOracle)?;
				feed.ensure_valid_round(&oracle, round_id)?;

				let (min_val, max_val) = feed.config.submission_value_bounds;
				ensure!(submission >= min_val, Error::<T>::SubmissionBelowMinimum);
				ensure!(submission <= max_val, Error::<T>::SubmissionAboveMaximum);

				let new_round_id = feed.reporting_round_id().saturating_add(One::one());
				let next_eligible_round = oracle_status.last_started_round
					.unwrap_or(Zero::zero())
					.checked_add(feed.config.restart_delay).ok_or(Error::<T>::Overflow)?
					.checked_add(One::one()).ok_or(Error::<T>::Overflow)?;
				let eligible_to_start = round_id >= next_eligible_round
					|| oracle_status.last_started_round.is_none();

				// initialize the round if conditions are met
				if round_id == new_round_id && eligible_to_start {
					let started_at = feed.initialize_round(new_round_id)?;

					Self::deposit_event(RawEvent::NewRound(feed_id, new_round_id, oracle.clone(), started_at));

					oracle_status.last_started_round = Some(new_round_id);
				}

				// record submission
				let mut details = Details::<T>::take(feed_id, round_id).ok_or(Error::<T>::NotAcceptingSubmissions)?;
				details.submissions.push(submission);

				oracle_status.last_reported_round = Some(round_id);
				oracle_status.latest_submission = Some(submission);
				OracleStati::<T>::insert(feed_id, &oracle, oracle_status);
				Self::deposit_event(RawEvent::SubmissionReceived(feed_id, round_id, submission, oracle.clone()));

				// update round answer
				let (min_count, max_count) = details.submission_count_bounds;
				if details.submissions.len() >= min_count as usize {
					let new_answer = median(&mut details.submissions);
					let mut round = Self::round(feed_id, round_id).ok_or(Error::<T>::RoundNotFound)?;
					round.answer = Some(new_answer);
					let updated_at = frame_system::Module::<T>::block_number();
					round.updated_at = Some(updated_at);
					round.answered_in_round = Some(round_id);
					Rounds::<T>::insert(feed_id, round_id, round);

					feed.config.latest_round = round_id;
					if feed.config.first_valid_round.is_none() {
						feed.config.first_valid_round = Some(round_id);
					}
					// the previous rounds is not eligible for answers any more, so we close it
					let prev_round_id = round_id.saturating_sub(1);
					if prev_round_id > 0 {
						Details::<T>::remove(feed_id, prev_round_id);
					}

					Self::deposit_event(RawEvent::AnswerUpdated(
						feed_id, round_id, new_answer, updated_at));
				}

				// update oracle rewards and try to reserve them
				let payment = details.payment;
				T::Currency::reserve(&T::ModuleId::get().into_account(), payment).or_else(|_| -> DispatchResult {
					// track the debt in case we cannot reserve
					Debt::<T>::try_mutate(|debt| {
						*debt = debt.checked_add(&payment).ok_or(Error::<T>::Overflow)?;
						Ok(())
					})
				})?;
				let mut oracle_meta = Self::oracle(&oracle).ok_or(Error::<T>::OracleNotFound)?;
				oracle_meta.withdrawable = oracle_meta.withdrawable
					.checked_add(&payment).ok_or(Error::<T>::Overflow)?;
				Oracles::<T>::insert(&oracle, oracle_meta);

				// delete the details if the maximum count has been reached
				if details.submissions.len() < max_count as usize {
					Details::<T>::insert(feed_id, round_id, details);
				}

				// TODO: answer validation
				Ok(().into())
			})
		}

		/// Disable and add oracles for the given feed.
		/// Limited to the owner of a feed.
		#[weight = T::WeightInfo::change_oracles(to_disable.len() as u32, to_add.len() as u32)]
		pub fn change_oracles(
			origin,
			feed_id: T::FeedId,
			to_disable: Vec<T::AccountId>,
			to_add: Vec<(T::AccountId, T::AccountId)>,
		) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;

			let mut to_disable = to_disable;
			to_disable.sort();
			to_disable.dedup();
			with_transaction_result(|| -> DispatchResultWithPostInfo {
				// synced on drop
				let mut feed = Feed::<T>::load_from(feed_id).ok_or(Error::<T>::FeedNotFound)?;
				feed.ensure_owner(&owner)?;
				feed.disable_oracles(to_disable)?;
				feed.add_oracles(to_add)?;

				Ok(().into())
			})
		}

		/// Update the configuration for future oracle rounds.
		/// Limited to the owner of a feed.
		#[weight = T::WeightInfo::update_future_rounds()]
		pub fn update_future_rounds(
			origin,
			feed_id: T::FeedId,
			payment: BalanceOf<T>,
			submission_count_bounds: (u32, u32),
			restart_delay: RoundId,
			timeout: T::BlockNumber,
		) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			// synced on drop
			let mut feed = Feed::<T>::load_from(feed_id).ok_or(Error::<T>::FeedNotFound)?;
			feed.ensure_owner(&owner)?;

			feed.update_future_rounds(payment, submission_count_bounds, restart_delay, timeout)?;

			Ok(().into())
		}

		/// Prune the state of a feed to reduce storage load.
		///
		/// - Will update the `first_valid_round` to the most recent round kept.
		/// - Will only prune until hitting the pruning window (which makes sure to keep N rounds
		/// of data available).
		///
		/// Limited to the owner of a feed.
		#[weight = T::WeightInfo::prune(keep_round.saturating_sub(*first_to_prune))]
		pub fn prune(
			origin,
			feed_id: T::FeedId,
			first_to_prune: RoundId,
			keep_round: RoundId,
		) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			ensure!(first_to_prune > Zero::zero(), Error::<T>::CannotPruneRoundZero);
			ensure!(keep_round > first_to_prune, Error::<T>::NothingToPrune);
			let mut feed = Self::feed_config(feed_id).ok_or(Error::<T>::FeedNotFound)?;
			ensure!(feed.owner == owner, Error::<T>::NotFeedOwner);

			if let Some(first_valid_round) = feed.first_valid_round {
				ensure!(first_to_prune <= first_valid_round, Error::<T>::PruneContiguously);
				let pruning_window = T::PruningWindow::get();
				ensure!(feed.latest_round.saturating_sub(first_to_prune) > pruning_window, Error::<T>::NothingToPrune);
				let keep_round = feed.latest_round.saturating_sub(pruning_window).min(keep_round);
				let mut round = first_to_prune;
				while round < keep_round {
					Rounds::<T>::remove(feed_id, round);
					Details::<T>::remove(feed_id, round);
					round += RoundId::one();
				}
				feed.first_valid_round = Some(keep_round.max(first_valid_round));

				Feeds::<T>::insert(feed_id, feed);

				Ok(())
			} else {
				Err(Error::<T>::NoValidRoundYet.into())
			}
		}

		// --- feed: round requests ---

		/// Set requester permissions for `requester`.
		/// Limited to the feed owner.
		#[weight = T::WeightInfo::set_requester()]
		pub fn set_requester(
			origin,
			feed_id: T::FeedId,
			requester: T::AccountId,
			delay: RoundId,
		) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			let feed = Self::feed_config(feed_id).ok_or(Error::<T>::FeedNotFound)?;
			ensure!(feed.owner == owner, Error::<T>::NotFeedOwner);

			// Keep the `last_started_round` if the requester already existed.
			let mut requester_meta = Self::requester(feed_id, &requester).unwrap_or_default();
			requester_meta.delay = delay;
			Requesters::<T>::insert(feed_id, &requester, requester_meta);

			Self::deposit_event(RawEvent::RequesterPermissionsSet(feed_id, requester, true, delay));

			Ok(().into())
		}

		/// Remove requester permissions for `requester`.
		/// Limited to the feed owner.
		#[weight = T::WeightInfo::remove_requester()]
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

		/// Request the start of a new oracle round.
		/// Limited to accounts with "requester" permission.
		#[weight = T::WeightInfo::request_new_round()]
		pub fn request_new_round(
			origin,
			feed_id: T::FeedId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let mut requester = Self::requester(feed_id, &sender).ok_or(Error::<T>::NotAuthorizedRequester)?;

			with_transaction_result(|| -> DispatchResultWithPostInfo {
				let mut feed = Feed::<T>::load_from(feed_id).ok_or(Error::<T>::FeedNotFound)?;

				let new_round = feed.reporting_round_id().checked_add(One::one()).ok_or(Error::<T>::Overflow)?;
				let last_started = requester.last_started_round.unwrap_or(Zero::zero());
				let next_allowed_round = last_started.checked_add(requester.delay).ok_or(Error::<T>::Overflow)?;
				ensure!(requester.last_started_round.is_none() || new_round > next_allowed_round, Error::<T>::CannotRequestRoundYet);

				requester.last_started_round = Some(new_round);
				Requesters::<T>::insert(feed_id, &sender, requester);

				feed.request_new_round(sender)?;

				Ok(().into())
			})
		}

		// --- oracle operations ---

		/// Withdraw `amount` payment of the given oracle to `recipient`.
		/// Limited to the oracle admin.
		#[weight = T::WeightInfo::withdraw_payment()]
		pub fn withdraw_payment(origin,
			oracle: T::AccountId,
			recipient: T::AccountId,
			amount: BalanceOf<T>,
		) {
			let admin = ensure_signed(origin)?;
			let mut oracle_meta = Self::oracle(&oracle).ok_or(Error::<T>::OracleNotFound)?;
			ensure!(oracle_meta.admin == admin, Error::<T>::NotAdmin);

			oracle_meta.withdrawable = oracle_meta.withdrawable
				.checked_sub(&amount).ok_or(Error::<T>::InsufficientFunds)?;

			T::Currency::transfer(&T::ModuleId::get().into_account(), &recipient, amount, ExistenceRequirement::KeepAlive)?;
			Oracles::<T>::insert(&oracle, oracle_meta);
		}

		/// Initiate an admin transfer for the given oracle.
		/// Limited to the oracle admin account.
		#[weight = T::WeightInfo::transfer_admin()]
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
		/// Limited to the pending oracle admin account.
		#[weight = T::WeightInfo::accept_admin()]
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

		// --- pallet admin operations ---

		/// Withdraw `amount` funds to `recipient`.
		/// Limited to the pallet admin.
		#[weight = T::WeightInfo::withdraw_funds()]
		pub fn withdraw_funds(origin,
			recipient: T::AccountId,
			amount: BalanceOf<T>,
		) {
			let sender = ensure_signed(origin)?;
			ensure!(sender == Self::pallet_admin(), Error::<T>::NotPalletAdmin);
			let fund = T::ModuleId::get().into_account();
			let reserve = T::Currency::free_balance(&fund);
			let new_reserve = reserve.checked_sub(&amount).ok_or(Error::<T>::InsufficientFunds)?;
			ensure!(new_reserve >= T::MinimumReserve::get(), Error::<T>::InsufficientReserve);
			T::Currency::transfer(&fund, &recipient, amount, ExistenceRequirement::KeepAlive)?;
		}

		/// Reduce the amount of debt in the pallet by moving funds from
		/// the free balance to the reserved so oracles can be payed out.
		/// Limited to the pallet admin.
		#[weight = T::WeightInfo::reduce_debt()]
		pub fn reduce_debt(origin, amount: BalanceOf<T>) -> DispatchResult {
			let _sender = ensure_signed(origin)?;
			Debt::<T>::try_mutate(|debt| {
				let to_reserve = amount.min(*debt);
				T::Currency::reserve(&T::ModuleId::get().into_account(), to_reserve)?;
				// it's fine if we saturate to 0 debt
				*debt = debt.saturating_sub(amount);
				Ok(())
			})
		}

		/// Initiate an admin transfer for the pallet.
		/// Limited to the pallet admin account.
		#[weight = T::WeightInfo::transfer_pallet_admin()]
		pub fn transfer_pallet_admin(
			origin,
			new_pallet_admin: T::AccountId,
		) -> DispatchResult {
			let old_admin = ensure_signed(origin)?;

			ensure!(Self::pallet_admin() == old_admin, Error::<T>::NotPalletAdmin);

			PendingPalletAdmin::<T>::put(&new_pallet_admin);

			Self::deposit_event(RawEvent::PalletAdminUpdateRequested(old_admin, new_pallet_admin));

			Ok(())
		}

		/// Complete an admin transfer for the pallet.
		/// Limited to the pending pallet admin account.
		#[weight = T::WeightInfo::accept_pallet_admin()]
		pub fn accept_pallet_admin(origin) -> DispatchResult {
			let new_pallet_admin = ensure_signed(origin)?;

			ensure!(PendingPalletAdmin::<T>::get().filter(|p| p == &new_pallet_admin).is_some(), Error::<T>::NotPendingPalletAdmin);

			PendingPalletAdmin::<T>::take();
			PalletAdmin::<T>::put(&new_pallet_admin);

			Self::deposit_event(RawEvent::PalletAdminUpdated(new_pallet_admin));

			Ok(())
		}

		/// Allow the given account to create oracle feeds.
		/// Limited to the pallet admin account.
		#[weight = T::WeightInfo::set_feed_creator()]
		pub fn set_feed_creator(origin, new_creator: T::AccountId) {
			let admin = ensure_signed(origin)?;
			ensure!(Self::pallet_admin() == admin, Error::<T>::NotPalletAdmin);

			FeedCreators::<T>::insert(&new_creator, ());

			Self::deposit_event(RawEvent::FeedCreator(new_creator));
		}

		/// Disallow the given account to create oracle feeds.
		/// Limited to the pallet admin account.
		#[weight = T::WeightInfo::remove_feed_creator()]
		pub fn remove_feed_creator(origin, creator: T::AccountId) {
			let admin = ensure_signed(origin)?;
			ensure!(Self::pallet_admin() == admin, Error::<T>::NotPalletAdmin);

			FeedCreators::<T>::remove(&creator);

			Self::deposit_event(RawEvent::FeedCreatorRemoved(creator));
		}
	}
}

/// Proxy used for interaction with a feed.
/// `should_sync` flag determines whether the `config` is put into
/// storage on `drop`.
pub struct Feed<T: Trait> {
	id: T::FeedId,
	config: FeedConfigOf<T>,
	should_sync: bool,
}

impl<T: Trait> Feed<T> {
	// --- constructors ---

	/// Create a new feed with the given id and config.
	/// Will store the config when dropped.
	fn new(id: T::FeedId, config: FeedConfigOf<T>) -> Self {
		Self {
			id,
			config,
			should_sync: true,
		}
	}

	/// Load the feed with the given id for reading.
	/// Will not store the config when dropped.
	/// -> Don't mutate the feed object.
	fn read_only_from(id: T::FeedId) -> Option<Self> {
		let config = Feeds::<T>::get(id)?;
		Some(Self {
			id,
			config,
			should_sync: false,
		})
	}

	/// Load the feed with the given id from storage.
	/// Will store the config when dropped.
	fn load_from(id: T::FeedId) -> Option<Self> {
		let config = Feeds::<T>::get(id)?;
		Some(Self {
			id,
			config,
			should_sync: true,
		})
	}

	// --- getters ---

	/// Return the round oracles are currently reporting data for.
	fn reporting_round_id(&self) -> RoundId {
		self.config.reporting_round
	}

	/// Return the round data (including the answer, if present).
	fn round(&self, round: RoundId) -> Option<RoundOf<T>> {
		Rounds::<T>::get(self.id, round)
	}

	/// Return the round details (including submissions).
	fn details(&self, round: RoundId) -> Option<RoundDetailsOf<T>> {
		Details::<T>::get(self.id, round)
	}

	/// Return the oracle status associated with this feed.
	fn status(&self, oracle: &T::AccountId) -> Option<OracleStatusOf<T>> {
		OracleStati::<T>::get(self.id, oracle)
	}

	/// Return the number of oracles that can submit data for this feed.
	fn oracle_count(&self) -> u32 {
		self.config.oracle_count
	}

	// --- checks ---

	/// Make sure that the given account is the owner of the feed.
	fn ensure_owner(&self, owner: &T::AccountId) -> DispatchResult {
		ensure!(&self.config.owner == owner, Error::<T>::NotFeedOwner);
		Ok(())
	}

	/// Make sure that the given oracle can submit data for the given round.
	fn ensure_valid_round(&self, oracle: &T::AccountId, round_id: RoundId) -> DispatchResult {
		let o = self.status(oracle).ok_or(Error::<T>::NotOracle)?;

		ensure!(o.starting_round <= round_id, Error::<T>::OracleNotEnabled);
		ensure!(
			o.ending_round.map(|e| e >= round_id).unwrap_or(true),
			Error::<T>::OracleDisabled
		);
		ensure!(
			o.last_reported_round.map(|l| l < round_id).unwrap_or(true),
			Error::<T>::ReportingOrder
		);
		let is_current = round_id == self.reporting_round_id();
		let is_next = round_id == self.reporting_round_id().saturating_add(One::one());
		let current_unanswered = self
			.round(self.reporting_round_id())
			.map(|r| r.updated_at.is_none())
			.unwrap_or(true);
		let is_previous = round_id.saturating_add(One::one()) == self.reporting_round_id();
		ensure!(
			is_current || is_next || (is_previous && current_unanswered),
			Error::<T>::InvalidRound
		);
		ensure!(
			round_id == RoundId::one() || self.is_supersedable(round_id.saturating_sub(One::one())),
			Error::<T>::NotSupersedable
		);
		Ok(())
	}

	/// Check whether a round is timed out.
	/// Returns `false` for rounds not present in storage.
	fn is_timed_out(&self, round: RoundId) -> bool {
		// Assumption: returning false for non-existent rounds is fine.
		let started_at = self
			.round(round)
			.map(|r| r.started_at)
			.unwrap_or(Zero::zero());
		let timeout = self
			.details(round)
			.map(|d| d.timeout)
			.unwrap_or(Zero::zero());
		let block_num = frame_system::Module::<T>::block_number();

		started_at > Zero::zero()
			&& timeout > Zero::zero()
			&& started_at
				.checked_add(&timeout)
				.expect("started_at and timeout should have sane values -> no overflow; qed")
				< block_num
	}

	/// Check whether a round has been updated.
	/// Returns `false` for rounds not present in storage.
	fn was_updated(&self, round: RoundId) -> bool {
		self.round(round)
			.map(|r| r.updated_at.is_some())
			.unwrap_or(false)
	}

	/// Check whether the round can be superseded by the next one.
	/// Returns `false` for rounds not present in storage.
	fn is_supersedable(&self, round: RoundId) -> bool {
		round == RoundId::zero() || self.was_updated(round) || self.is_timed_out(round)
	}

	// --- mutators ---

	/// Add the given oracles to the feed.
	///
	/// **Warning:** Fallible function that changes storage.
	// TODO: use [require_transactional](https://github.com/paritytech/substrate/issues/7004)
	// after migrating to Substrate v3 for this and others.
	fn add_oracles(&mut self, to_add: Vec<(T::AccountId, T::AccountId)>) -> DispatchResult {
		let new_count = self
			.oracle_count()
			// saturating is fine because we inforce a limit below
			.saturating_add(to_add.len() as u32);
		ensure!(
			new_count <= T::OracleCountLimit::get(),
			Error::<T>::OraclesLimitExceeded
		);
		self.config.oracle_count = new_count;
		for (oracle, admin) in to_add {
			// Initialize the oracle if it is not tracked, yet.
			Oracles::<T>::try_mutate(&oracle, |maybe_meta| -> DispatchResult {
				match maybe_meta {
					None => {
						*maybe_meta = Some(OracleMeta {
							withdrawable: Zero::zero(),
							admin,
							..Default::default()
						});
					}
					Some(meta) => ensure!(meta.admin == admin, Error::<T>::OwnerCannotChangeAdmin),
				}
				Ok(())
			})?;
			OracleStati::<T>::try_mutate(self.id, &oracle, |maybe_status| -> DispatchResult {
				// Only allow enabling oracles once in order to keep
				// the count accurate.
				ensure!(
					maybe_status
						.as_ref()
						.map(|s| s.ending_round.is_some())
						.unwrap_or(true),
					Error::<T>::AlreadyEnabled
				);
				*maybe_status = Some(OracleStatus::new(self.reporting_round_id()));
				Ok(())
			})?;
			Module::<T>::deposit_event(RawEvent::OraclePermissionsUpdated(self.id, oracle, true));
		}

		Ok(())
	}

	/// Disable the given oracles.
	///
	/// **Warning:** Fallible function that changes storage.
	fn disable_oracles(&mut self, to_disable: Vec<T::AccountId>) -> DispatchResult {
		let disabled_count = to_disable.len() as u32;
		debug_assert!(self.config.oracle_count >= disabled_count);
		// This should be fine as we assert on every oracle
		// in the loop that it exists and we deduplicate.
		self.config.oracle_count = self.config.oracle_count.saturating_sub(disabled_count);
		for d in to_disable {
			let mut status = self.status(&d).ok_or(Error::<T>::OracleNotFound)?;
			ensure!(status.ending_round.is_none(), Error::<T>::OracleDisabled);
			status.ending_round = Some(self.reporting_round_id());
			OracleStati::<T>::insert(self.id, &d, status);
			Module::<T>::deposit_event(RawEvent::OraclePermissionsUpdated(self.id, d, false));
		}
		Ok(())
	}

	/// Update the configuration for future oracle rounds.
	/// (Past and present rounds are unaffected.)
	///
	/// **Warning:** Fallible function that changes storage.
	fn update_future_rounds(
		&mut self,
		payment: BalanceOf<T>,
		submission_count_bounds: (u32, u32),
		restart_delay: RoundId,
		timeout: T::BlockNumber,
	) -> DispatchResult {
		let (min, max) = submission_count_bounds;
		ensure!(max >= min, Error::<T>::WrongBounds);
		// Make sure that both the min and max of submissions is
		// less or equal to the number of oracles.
		ensure!(self.oracle_count() >= max, Error::<T>::MaxExceededTotal);
		// Make sure that at least one oracle can request a new
		// round.
		ensure!(
			self.oracle_count() > restart_delay,
			Error::<T>::DelayNotBelowCount
		);
		if self.oracle_count() > 0 {
			ensure!(min > 0, Error::<T>::WrongBounds);
		}

		self.config.payment = payment;
		self.config.submission_count_bounds = submission_count_bounds;
		self.config.restart_delay = restart_delay;
		self.config.timeout = timeout;

		Module::<T>::deposit_event(RawEvent::RoundDetailsUpdated(
			self.id,
			payment,
			submission_count_bounds,
			restart_delay,
			timeout,
		));
		Ok(())
	}

	/// Initialize a new round.
	/// Will close the previous one if it is timed out.
	///
	/// **Warning:** Fallible function that changes storage.
	fn initialize_round(&mut self, new_round_id: RoundId) -> Result<T::BlockNumber, DispatchError> {
		self.config.reporting_round = new_round_id;

		let prev_round_id = new_round_id.saturating_sub(One::one());
		if self.is_timed_out(prev_round_id) {
			self.close_timed_out_round(prev_round_id)?;
		}

		Details::<T>::insert(
			self.id,
			new_round_id,
			RoundDetails {
				submissions: Vec::new(),
				submission_count_bounds: self.config.submission_count_bounds,
				payment: self.config.payment,
				timeout: self.config.timeout,
			},
		);
		let started_at = frame_system::Module::<T>::block_number();
		Rounds::<T>::insert(self.id, new_round_id, Round::new(started_at));

		Ok(started_at)
	}

	/// Close a timed out round and remove its details.
	///
	/// **Warning:** Fallible function that changes storage.
	fn close_timed_out_round(&self, timed_out_id: RoundId) -> DispatchResult {
		let prev_id = timed_out_id.saturating_sub(One::one());
		let prev_round = self.round(prev_id).ok_or(Error::<T>::RoundNotFound)?;
		let mut timed_out_round = self.round(timed_out_id).ok_or(Error::<T>::RoundNotFound)?;
		timed_out_round.answer = prev_round.answer;
		timed_out_round.answered_in_round = prev_round.answered_in_round;
		let updated_at = frame_system::Module::<T>::block_number();
		timed_out_round.updated_at = Some(updated_at);

		Rounds::<T>::insert(self.id, timed_out_id, timed_out_round);
		Details::<T>::remove(self.id, timed_out_id);

		Ok(())
	}

	/// Store the feed config in storage.
	fn sync_to_storage(&mut self) {
		Feeds::<T>::insert(self.id, sp_std::mem::take(&mut self.config));
	}
}

// We want the feed to sync automatically when going out of scope.
impl<T: Trait> Drop for Feed<T> {
	fn drop(&mut self) {
		if self.should_sync {
			self.sync_to_storage();
		}
	}
}

impl<T: Trait> FeedOracle<T> for Module<T> {
	type FeedId = T::FeedId;
	type Feed = Feed<T>;
	type MutableFeed = Feed<T>;

	/// Return a transient feed proxy object for interacting with the feed given by the id.
	/// Provides read-only access.
	fn feed(id: Self::FeedId) -> Option<Self::Feed> {
		Feed::read_only_from(id)
	}

	/// Return a transient feed proxy object for interacting with the feed given by the id.
	/// Provides read-write access.
	fn feed_mut(id: Self::FeedId) -> Option<Self::MutableFeed> {
		Feed::load_from(id)
	}
}

impl<T: Trait> FeedInterface<T> for Feed<T> {
	/// Returns the id of the first round that contains non-default data.
	fn first_valid_round(&self) -> Option<RoundId> {
		self.config.first_valid_round
	}

	/// Returns the id of the latest oracle round.
	fn latest_round(&self) -> RoundId {
		self.config.latest_round
	}

	/// Returns the data for a given round.
	fn data_at(&self, round: RoundId) -> Option<RoundData<T::BlockNumber, T::Value>> {
		self.round(round)?.try_into().ok()
	}

	/// Returns the latest data for the feed.
	fn latest_data(&self) -> RoundData<T::BlockNumber, T::Value> {
		let latest_round = self.latest_round();
		self.data_at(latest_round).unwrap_or_else(|| {
			debug_assert!(false, "The latest round data should always be available.");
			frame_support::debug::error!(
				"Latest round is data missing which should never happen. (Latest round id: {:?})",
				latest_round
			);
			RoundData::default()
		})
	}
}

impl<T: Trait> MutableFeedInterface<T> for Feed<T> {
	/// Requests that a new round be started for the feed.
	/// Returns `Ok` on success and `Err` in case the round could not be started.
	///
	/// **Warning:** Fallible function that changes storage.
	fn request_new_round(&mut self, requester: T::AccountId) -> DispatchResult {
		let new_round = self
			.reporting_round_id()
			.checked_add(One::one())
			.ok_or(Error::<T>::Overflow)?;
		ensure!(
			self.is_supersedable(self.reporting_round_id()),
			Error::<T>::RoundNotSupersedable
		);
		let started_at = self.initialize_round(new_round)?;

		Module::<T>::deposit_event(RawEvent::NewRound(
			self.id, new_round, requester, started_at,
		));

		Ok(())
	}
}

/// Trait for the chainlink pallet extrinsic weights.
pub trait WeightInfo {
	fn create_feed(o: u32) -> Weight;
	fn transfer_ownership() -> Weight;
	fn accept_ownership() -> Weight;
	fn submit_opening_round_answers() -> Weight;
	fn submit_closing_answer(o: u32) -> Weight;
	fn change_oracles(d: u32, n: u32) -> Weight;
	fn update_future_rounds() -> Weight;
	fn prune(r: u32) -> Weight;
	fn set_requester() -> Weight;
	fn remove_requester() -> Weight;
	fn request_new_round() -> Weight;
	fn withdraw_payment() -> Weight;
	fn transfer_admin() -> Weight;
	fn accept_admin() -> Weight;
	fn withdraw_funds() -> Weight;
	fn reduce_debt() -> Weight;
	fn transfer_pallet_admin() -> Weight;
	fn accept_pallet_admin() -> Weight;
	fn set_feed_creator() -> Weight;
	fn remove_feed_creator() -> Weight;
}
