use super::*;

use frame_support::weights::Weight;
use frame_support::{assert_noop, assert_ok, impl_outer_origin, parameter_types};
use sp_core::H256;

use frame_system as system;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};

impl_outer_origin! {
	pub enum Origin for Test {}
}

// Configure a mock runtime to test the pallet.

#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

type AccountId = u64;
type BlockNumber = u64;
impl system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}
type System = frame_system::Module<Test>;

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

type Balance = u64;
impl pallet_balances::Trait for Test {
	type MaxLocks = ();
	type Balance = Balance;
	type Event = ();
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}
type Balances = pallet_balances::Module<Test>;

const MIN_RESERVE: u64 = 100;
parameter_types! {
	pub const FeedModuleId: ModuleId = ModuleId(*b"linkfeed");
	pub const MinimumReserve: u64 = MIN_RESERVE;
	pub const StringLimit: u32 = 30;
	pub const OracleLimit: u32 = 10;
	pub const FeedLimit: u32 = 10;
	pub const PruningWindow: u32 = 3;
}

type RoundId = u32;
impl Trait for Test {
	type Event = ();
	type FeedId = u32;
	type RoundId = RoundId;
	type Value = u64;
	type Currency = Balances;
	type ModuleId = FeedModuleId;
	type MinimumReserve = MinimumReserve;
	type StringLimit = StringLimit;
	type OracleCountLimit = OracleLimit;
	type FeedLimit = FeedLimit;
	type PruningWindow = PruningWindow;
}
type ChainlinkFeed = crate::Module<Test>;

#[derive(Debug, Default)]
struct FeedBuilder {
	owner: Option<AccountId>,
	payment: Option<Balance>,
	timeout: Option<BlockNumber>,
	min_submissions: Option<u32>,
	restart_delay: Option<RoundId>,
	oracles: Option<Vec<(AccountId, AccountId)>>,
}

impl FeedBuilder {
	fn new() -> Self {
		Self::default()
	}

	fn owner(mut self, o: AccountId) -> Self {
		self.owner = Some(o);
		self
	}

	fn payment(mut self, p: Balance) -> Self {
		self.payment = Some(p);
		self
	}

	fn timeout(mut self, t: BlockNumber) -> Self {
		self.timeout = Some(t);
		self
	}

	fn min_submissions(mut self, m: u32) -> Self {
		self.min_submissions = Some(m);
		self
	}

	fn restart_delay(mut self, d: RoundId) -> Self {
		self.restart_delay = Some(d);
		self
	}

	fn oracles(mut self, o: Vec<(AccountId, AccountId)>) -> Self {
		self.oracles = Some(o);
		self
	}

	fn build_and_store(self) -> DispatchResultWithPostInfo {
		let owner = Origin::signed(self.owner.unwrap_or(1));
		let payment = self.payment.unwrap_or(20);
		let timeout = self.timeout.unwrap_or(1);
		let value_bounds = (1, 1_000);
		let min_submissions = self.min_submissions.unwrap_or(2);
		let decimals = 5;
		let description = b"desc".to_vec();
		let restart_delay = self.restart_delay.unwrap_or(1);
		let oracles = self.oracles.unwrap_or(vec![(2, 4), (3, 4), (4, 4)]);
		ChainlinkFeed::create_feed(
			owner,
			payment,
			timeout,
			value_bounds,
			min_submissions,
			decimals,
			description,
			restart_delay,
			oracles,
		)
	}
}

pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();

	let module_account: AccountId = FeedModuleId::get().into_account();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(module_account, 100 * MIN_RESERVE)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	crate::GenesisConfig::<Test> {
		pallet_admin: module_account,
	}
	.assimilate_storage(&mut t)
	.unwrap();

	t.into()
}

#[test]
fn median_works() {
	let mut values = vec![4u32, 6, 2, 7];
	assert_eq!(median(&mut values), 5);
	let mut values = vec![4u32, 6, 2, 7, 9];
	assert_eq!(median(&mut values), 6);
}

#[test]
fn feed_creation_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(ChainlinkFeed::create_feed(
			Origin::signed(1),
			20,
			10,
			(10, 1_000),
			3,
			5,
			b"desc".to_vec(),
			2,
			vec![(1, 4), (2, 4), (3, 4)],
		));
	});
}

#[test]
fn submit_should_work() {
	new_test_ext().execute_with(|| {
		let payment = 20;
		let timeout = 10;
		let min_submissions = 2;
		let oracles = vec![(1, 4), (2, 4), (3, 4)];
		let submission_count_bounds = (min_submissions, oracles.len() as u32);
		assert_ok!(FeedBuilder::new()
			.payment(payment)
			.timeout(timeout)
			.min_submissions(min_submissions)
			.oracles(oracles)
			.build_and_store());

		let feed_id = 0;
		let round_id = 1;
		let oracle = 2;
		let submission = 42;
		assert_ok!(ChainlinkFeed::submit(
			Origin::signed(oracle),
			feed_id,
			round_id,
			submission
		));
		let second_oracle = 3;
		assert_ok!(ChainlinkFeed::submit(
			Origin::signed(second_oracle),
			feed_id,
			round_id,
			submission
		));
		let round = ChainlinkFeed::round(feed_id, round_id).expect("first round should be present");
		assert_eq!(
			round,
			Round {
				started_at: 0,
				answer: Some(submission),
				updated_at: Some(0),
				answered_in_round: Some(1),
			}
		);
		let details = ChainlinkFeed::round_details(feed_id, round_id)
			.expect("details for first round should be present");
		assert_eq!(
			details,
			RoundDetails {
				submissions: vec![submission, submission],
				submission_count_bounds,
				payment,
				timeout,
			}
		);
		let oracle_status =
			ChainlinkFeed::oracle_status(feed_id, oracle).expect("oracle status should be present");
		assert_eq!(oracle_status.latest_submission, Some(submission));
	});
}

#[test]
fn change_oracles_should_work() {
	new_test_ext().execute_with(|| {
		let initial_oracles = vec![(1, 4), (2, 4), (3, 4)];
		assert_ok!(FeedBuilder::new()
			.oracles(initial_oracles.clone())
			.build_and_store());
		for (o, _a) in initial_oracles.iter() {
			assert!(ChainlinkFeed::oracle(o).is_some(), "oracle should be present");
		}
		let feed_id = 0;
		let feed = ChainlinkFeed::feed_config(feed_id).expect("feed should be there");
		assert_eq!(feed.oracle_count, 3);

		let to_disable: Vec<u64> = initial_oracles.into_iter().take(2).map(|(o, _a)| o).collect();
		let to_add = vec![(6, 9), (7, 9), (8, 9)];
		assert_ok!(ChainlinkFeed::change_oracles(
			Origin::signed(1),
			feed_id,
			to_disable.clone(),
			to_add.clone(),
		));
		let feed = ChainlinkFeed::feed_config(feed_id).expect("feed should be there");
		assert_eq!(feed.oracle_count, 4);
		assert_eq!(Oracles::<Test>::iter().count(), 6);
		assert_eq!(OracleStati::<Test>::iter().count(), 6);
		for o in to_disable.iter() {
			assert!(
				ChainlinkFeed::oracle_status(feed_id, o)
					.unwrap()
					.ending_round
					.is_some(),
				"oracle should be disabled"
			);
		}
		for (o, _a) in to_add.iter() {
			assert!(ChainlinkFeed::oracle(o).is_some(), "oracle should be present");
		}
	});
}

#[test]
fn oracle_deduplication() {
	new_test_ext().execute_with(|| {
		let initial_oracles = vec![(1, 4), (2, 4), (3, 4)];

		assert_ok!(FeedBuilder::new()
			.oracles(initial_oracles.clone())
			.build_and_store());
		let feed_id = 0;
		let mut to_disable = vec![1, 2, 1];
		let to_add = vec![];
		assert_ok!(ChainlinkFeed::change_oracles(
			Origin::signed(1),
			feed_id,
			to_disable.clone(),
			to_add.clone(),
		));
		to_disable.sort();
		to_disable.dedup();
		for o in to_disable.iter() {
			assert!(
				ChainlinkFeed::oracle_status(feed_id, o)
					.unwrap()
					.ending_round
					.is_some(),
				"oracle should be disabled"
			);
		}
		let feed = ChainlinkFeed::feed_config(feed_id).expect("feed should be there");
		assert_eq!(feed.oracle_count, 1);
	});
}

#[test]
fn update_future_rounds_should_work() {
	new_test_ext().execute_with(|| {
		let old_payment = 20;
		let old_timeout = 10;
		let old_min = 3;
		let oracles = vec![(1, 4), (2, 4), (3, 4)];
		assert_ok!(FeedBuilder::new()
			.payment(old_payment)
			.timeout(old_timeout)
			.min_submissions(old_min)
			.oracles(oracles)
			.build_and_store());
		let feed_id = 0;
		let feed = ChainlinkFeed::feed_config(feed_id).expect("feed should be there");
		assert_eq!(feed.payment, old_payment);

		let new_payment = 30;
		let new_min = 3;
		let new_max = 3;
		let new_delay = 1;
		let new_timeout = 5;
		assert_ok!(ChainlinkFeed::update_future_rounds(
			Origin::signed(1),
			feed_id,
			new_payment,
			(new_min, new_max),
			new_delay,
			new_timeout,
		));

		let feed_id = 0;
		let feed = ChainlinkFeed::feed_config(feed_id).expect("feed should be there");
		assert_eq!(feed.payment, new_payment);
	});
}

#[test]
fn admin_transfer_should_work() {
	new_test_ext().execute_with(|| {
		let oracle = 1;
		let old_admin = 2;
		assert_ok!(FeedBuilder::new()
			.min_submissions(1)
			.restart_delay(0)
			.oracles(vec![(oracle, old_admin)])
			.build_and_store());

		let new_admin = 42;
		assert_ok!(ChainlinkFeed::transfer_admin(
			Origin::signed(old_admin),
			oracle,
			new_admin
		));
		let oracle_meta = ChainlinkFeed::oracle(oracle).expect("oracle should be present");
		assert_eq!(oracle_meta.pending_admin, Some(new_admin));
		assert_ok!(ChainlinkFeed::accept_admin(Origin::signed(new_admin), oracle));
		let oracle_meta = ChainlinkFeed::oracle(oracle).expect("oracle should be present");
		assert_eq!(oracle_meta.pending_admin, None);
		assert_eq!(oracle_meta.admin, new_admin);
	});
}

#[test]
fn request_new_round_should_work() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		let payment = 20;
		let timeout = 10;
		let min_submissions = 2;
		let oracles = vec![(1, 4), (2, 4), (3, 4)];
		let submission_count_bounds = (min_submissions, oracles.len() as u32);
		assert_ok!(FeedBuilder::new()
			.owner(owner)
			.payment(payment)
			.timeout(timeout)
			.min_submissions(min_submissions)
			.oracles(oracles)
			.build_and_store());

		let feed_id = 0;
		let requester = 22;
		let delay = 4;
		assert_ok!(ChainlinkFeed::set_requester(
			Origin::signed(owner),
			feed_id,
			requester,
			delay
		));
		let requester_meta =
			ChainlinkFeed::requester(feed_id, requester).expect("requester should be present");
		assert_eq!(
			requester_meta,
			Requester {
				delay,
				last_started_round: None
			}
		);
		assert_ok!(ChainlinkFeed::request_new_round(
			Origin::signed(requester),
			feed_id
		));
		let round_id = 1;
		let round = ChainlinkFeed::round(feed_id, round_id).expect("first round should be present");
		assert_eq!(
			round,
			Round {
				started_at: 0,
				..Default::default()
			}
		);
		let details = ChainlinkFeed::round_details(feed_id, round_id)
			.expect("details for first round should be present");
		assert_eq!(
			details,
			RoundDetails {
				submissions: Vec::new(),
				submission_count_bounds,
				payment,
				timeout,
			}
		);
	});
}

#[test]
fn transfer_ownership_should_work() {
	new_test_ext().execute_with(|| {
		let old_owner = 1;
		assert_ok!(FeedBuilder::new().owner(old_owner).build_and_store());

		let feed_id = 0;
		let new_owner = 42;
		assert_ok!(ChainlinkFeed::transfer_ownership(
			Origin::signed(old_owner),
			feed_id,
			new_owner
		));
		let feed = ChainlinkFeed::feed_config(feed_id).expect("feed should be there");
		assert_eq!(feed.pending_owner, Some(new_owner));
		assert_ok!(ChainlinkFeed::accept_ownership(
			Origin::signed(new_owner),
			feed_id
		));
		let feed = ChainlinkFeed::feed_config(feed_id).expect("feed should be there");
		assert_eq!(feed.pending_owner, None);
		assert_eq!(feed.owner, new_owner);
	});
}

#[test]
fn feed_oracle_trait_should_work() {
	new_test_ext().execute_with(|| {
		let oracle = 2;
		let second_oracle = 3;
		assert_ok!(FeedBuilder::new()
			.oracles(vec![(oracle, 4), (second_oracle, 4)])
			.build_and_store());

		let feed_id = 0;
		{
			let feed = ChainlinkFeed::feed(feed_id).expect("feed should be there");
			assert_eq!(feed.first_valid_round(), None);
			assert_eq!(feed.latest_round(), 0);
			assert_eq!(feed.latest_data(), RoundDataOf::<Test>::default());
		}
		let round_id = 1;
		let submission = 42;
		assert_ok!(ChainlinkFeed::submit(
			Origin::signed(oracle),
			feed_id,
			round_id,
			submission
		));
		assert_ok!(ChainlinkFeed::submit(
			Origin::signed(second_oracle),
			feed_id,
			round_id,
			submission
		));
		{
			let feed = ChainlinkFeed::feed(feed_id).expect("feed should be there");
			assert_eq!(feed.first_valid_round(), Some(1));
			assert_eq!(feed.latest_round(), 1);
			assert_eq!(
				feed.latest_data(),
				RoundData {
					answer: 42,
					started_at: 0,
					updated_at: 0,
					answered_in_round: 1,
				}
			);

			assert_ok!(feed.request_new_round(AccountId::default()));
		}
		let round_id = 2;
		let round = ChainlinkFeed::round(feed_id, round_id).expect("second round should be present");
		assert_eq!(
			round,
			Round {
				started_at: 0,
				..Default::default()
			}
		);
	});
}

#[test]
fn payment_withdrawal_should_work() {
	new_test_ext().execute_with(|| {
		let amount = 50;
		let oracle = 3;
		let admin = 4;
		let recipient = 5;
		Oracles::<Test>::insert(
			oracle,
			OracleMeta {
				withdrawable: amount,
				admin,
				..Default::default()
			},
		);
		assert_noop!(
			ChainlinkFeed::withdraw_payment(Origin::signed(admin), oracle, recipient, 2 * amount),
			Error::<Test>::InsufficientFunds
		);
		assert_ok!(ChainlinkFeed::withdraw_payment(
			Origin::signed(admin),
			oracle,
			recipient,
			amount
		));
	});
}

#[test]
fn funds_withdrawal_should_work() {
	new_test_ext().execute_with(|| {
		let amount = 50;
		let recipient = 5;
		let fund = FeedModuleId::get().into_account();
		assert_noop!(
			ChainlinkFeed::withdraw_funds(Origin::signed(fund), recipient, 100 * MIN_RESERVE),
			Error::<Test>::InsufficientReserve
		);
		assert_ok!(ChainlinkFeed::withdraw_funds(
			Origin::signed(fund),
			recipient,
			amount
		));
	});
}

#[test]
fn transfer_pallet_admin_should_work() {
	new_test_ext().execute_with(|| {
		let new_admin = 23;
		let fund = FeedModuleId::get().into_account();
		assert_ok!(ChainlinkFeed::transfer_pallet_admin(
			Origin::signed(fund),
			new_admin
		));
		assert_eq!(PendingPalletAdmin::<Test>::get(), Some(new_admin));
		assert_ok!(ChainlinkFeed::accept_pallet_admin(Origin::signed(new_admin)));
		assert_eq!(PalletAdmin::<Test>::get(), new_admin);
		assert_eq!(PendingPalletAdmin::<Test>::get(), None);
	});
}

#[test]
fn prune_should_work() {
	// ## Pruning Testing Scenario
	//
	// |- round zero
	// v             v- latest round
	// 0 1 2 3 4 5 6 7 8 <- reporting round
	//       ^- first valid round
	new_test_ext().execute_with(|| {
		let feed_id = 0;
		let oracle_a = 2;
		let oracle_b = 3;
		let oracle_admin = 4;
		let submission = 42;
		let submit_a = |r| {
			assert_ok!(ChainlinkFeed::submit(
				Origin::signed(oracle_a),
				feed_id,
				r,
				submission
			));
		};
		let submit_a_and_b = |r| {
			submit_a(r);
			assert_ok!(ChainlinkFeed::submit(
				Origin::signed(oracle_b),
				feed_id,
				r,
				submission
			));
		};

		let owner = 1;
		// we require min 2 oracles so that we can time out the first few
		// so first_valid_round will be > 1
		assert_ok!(FeedBuilder::new()
			.owner(owner)
			.timeout(1)
			.min_submissions(2)
			.restart_delay(0)
			.oracles(vec![(oracle_a, oracle_admin), (oracle_b, oracle_admin)])
			.build_and_store());

		System::set_block_number(1);
		// submit 2 rounds that will be timed out
		submit_a(1);
		System::set_block_number(3);
		submit_a(2);
		System::set_block_number(5);
		// submit the valid rounds
		submit_a_and_b(3);
		submit_a_and_b(4);
		submit_a_and_b(5);
		submit_a_and_b(6);
		submit_a_and_b(7);
		// simulate an unfinished round so reporting_round != latest_round
		submit_a(8);

		assert_ok!(ChainlinkFeed::prune(Origin::signed(owner), feed_id, 1, 5));
		// we try to prune until 5, but limits are set up in a way that we can
		// only prune until 4
		assert_eq!(ChainlinkFeed::round(feed_id, 3), None);
		let round = ChainlinkFeed::round(feed_id, 4).expect("fourth round should be present");
		assert_eq!(
			round,
			Round {
				started_at: 5,
				answer: Some(submission),
				updated_at: Some(5),
				answered_in_round: Some(4),
			}
		);
	});
}
