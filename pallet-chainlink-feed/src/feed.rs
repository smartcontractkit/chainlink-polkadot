//! utils for creating feeds

use crate::{BalanceOf, Config, FeedConfig, RoundId};
use frame_support::{
	sp_runtime::traits::{One, Zero},
	BoundedVec, Parameter,
};
use sp_std::prelude::*;

use frame_support::sp_std::convert::TryInto;
use frame_support::traits::Get;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Builder with all the parameters to call the `create_feed`
#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct FeedBuilder<AccountId, Balance, BlockNumber, Value> {
	#[cfg_attr(
		feature = "std",
		serde(bound(
			serialize = "Option<AccountId>: Serialize",
			deserialize = "Option<AccountId>: Deserialize<'de>"
		))
	)]
	pub owner: Option<AccountId>,
	#[cfg_attr(
		feature = "std",
		serde(bound(
			serialize = "Option<Balance>: Serialize",
			deserialize = "Option<Balance>: Deserialize<'de>"
		))
	)]
	pub payment: Option<Balance>,
	#[cfg_attr(
		feature = "std",
		serde(bound(
			serialize = "Option<BlockNumber>: Serialize",
			deserialize = "Option<BlockNumber>: Deserialize<'de>"
		))
	)]
	pub timeout: Option<BlockNumber>,
	#[cfg_attr(
		feature = "std",
		serde(bound(
			serialize = "Option<(Value, Value)>: Serialize",
			deserialize = "Option<(Value, Value)>: Deserialize<'de>"
		))
	)]
	pub value_bounds: Option<(Value, Value)>,
	pub min_submissions: Option<u32>,
	pub description: Option<Vec<u8>>,
	pub decimals: Option<u8>,
	pub restart_delay: Option<u32>,
	#[cfg_attr(
		feature = "std",
		serde(bound(
			serialize = "Option<Vec<(AccountId, AccountId)>>: Serialize",
			deserialize = "Option<Vec<(AccountId, AccountId)>>: Deserialize<'de>"
		))
	)]
	pub oracles: Option<Vec<(AccountId, AccountId)>>,
	pub pruning_window: Option<u32>,
	#[cfg_attr(
		feature = "std",
		serde(bound(
			serialize = "Option<Balance>: Serialize",
			deserialize = "Option<Balance>: Deserialize<'de>"
		))
	)]
	pub max_debt: Option<Balance>,
}

impl<AccountId, Balance, BlockNumber, Value> FeedBuilder<AccountId, Balance, BlockNumber, Value>
where
	AccountId: Parameter,
	Balance: Parameter + Zero,
	BlockNumber: Parameter,
	Value: Parameter,
{
	pub fn new() -> Self {
		Self::default()
	}

	pub fn owner(mut self, o: AccountId) -> Self {
		self.owner = Some(o);
		self
	}

	pub fn payment(mut self, p: Balance) -> Self {
		self.payment = Some(p);
		self
	}

	pub fn timeout(mut self, t: BlockNumber) -> Self {
		self.timeout = Some(t);
		self
	}

	pub fn value_bounds(mut self, min: Value, max: Value) -> Self {
		self.value_bounds = Some((min, max));
		self
	}

	pub fn min_submissions(mut self, m: u32) -> Self {
		self.min_submissions = Some(m);
		self
	}

	pub fn description(mut self, d: Vec<u8>) -> Self {
		self.description = Some(d);
		self
	}

	pub fn restart_delay(mut self, d: u32) -> Self {
		self.restart_delay = Some(d);
		self
	}

	pub fn oracles(mut self, o: Vec<(AccountId, AccountId)>) -> Self {
		self.oracles = Some(o);
		self
	}

	pub fn decimals(mut self, decimals: u8) -> Self {
		self.decimals = Some(decimals);
		self
	}

	pub fn pruning_window(mut self, w: u32) -> Self {
		self.pruning_window = Some(w);
		self
	}

	pub fn max_debt(mut self, v: Balance) -> Self {
		self.max_debt = Some(v);
		self
	}

	/// turn the builder into a storage `FeedConfig`
	pub fn build<'a, StringLimit: Get<u32>>(
		self,
	) -> Result<
		FeedConfig<AccountId, Balance, BlockNumber, Value, BoundedVec<u8, StringLimit>>,
		&'static str,
	> {
		let oracles = self.oracles.ok_or("Feed requires oracles.")?;
		let submission_count_bounds = (
			self.min_submissions
				.ok_or("Feed requires min_submissions.")?,
			oracles.len() as u32,
		);

		let description = if let Some(desc) = self.description {
			desc.try_into().map_err(|_| "Feed description too long.")?
		} else {
			Default::default()
		};

		let config = FeedConfig {
			owner: self.owner.ok_or("Feed requires owner.")?,
			pending_owner: None,
			submission_value_bounds: self.value_bounds.ok_or("Feed requires value_bounds.")?,
			submission_count_bounds,
			payment: self.payment.ok_or("Feed requires payment.")?,
			timeout: self.timeout.ok_or("Feed requires timeout.")?,
			decimals: self.decimals.ok_or("Feed requires decimals.")?,
			description,
			restart_delay: self.restart_delay.ok_or("Feed requires restart_delay.")?,
			reporting_round: Zero::zero(),
			latest_round: Zero::zero(),
			first_valid_round: None,
			oracle_count: Zero::zero(),
			pruning_window: self.pruning_window.unwrap_or(RoundId::MAX),
			next_round_to_prune: RoundId::one(),
			debt: Zero::zero(),
			max_debt: self.max_debt,
		};
		Ok(config)
	}
}

impl<AccountId, Balance, BlockNumber, Value> Default
	for FeedBuilder<AccountId, Balance, BlockNumber, Value>
{
	fn default() -> Self {
		Self {
			owner: None,
			payment: None,
			timeout: None,
			value_bounds: None,
			min_submissions: None,
			description: None,
			decimals: None,
			restart_delay: None,
			oracles: None,
			pruning_window: None,
			max_debt: None,
		}
	}
}

pub type FeedBuilderOf<T> = FeedBuilder<
	<T as frame_system::Config>::AccountId,
	BalanceOf<T>,
	<T as frame_system::Config>::BlockNumber,
	<T as Config>::Value,
>;
