//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.1

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

impl crate::WeightInfo for () {
	fn create_feed(o: u32) -> Weight {
		(335_863_000 as Weight)
			.saturating_add((59_108_000 as Weight).saturating_mul(o as Weight))
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().reads((2 as Weight).saturating_mul(o as Weight)))
			.saturating_add(DbWeight::get().writes(3 as Weight))
			.saturating_add(DbWeight::get().writes((2 as Weight).saturating_mul(o as Weight)))
	}
	fn transfer_ownership() -> Weight {
		(82_765_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn accept_ownership() -> Weight {
		(83_071_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn submit_opening_round_answers() -> Weight {
		(389_324_000 as Weight)
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().writes(6 as Weight))
	}
	fn submit_closing_answer(o: u32) -> Weight {
		(305_398_000 as Weight)
			.saturating_add((1_051_000 as Weight).saturating_mul(o as Weight))
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().writes(6 as Weight))
	}
	fn change_oracles(o: u32) -> Weight {
		(437_385_000 as Weight)
			.saturating_add((113_204_000 as Weight).saturating_mul(o as Weight))
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().reads((3 as Weight).saturating_mul(o as Weight)))
			.saturating_add(DbWeight::get().writes(1 as Weight))
			.saturating_add(DbWeight::get().writes((3 as Weight).saturating_mul(o as Weight)))
	}
	fn update_future_rounds() -> Weight {
		(76_909_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn prune(r: u32) -> Weight {
		(279_194_000 as Weight)
			.saturating_add((15_043_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
			.saturating_add(DbWeight::get().writes((2 as Weight).saturating_mul(r as Weight)))
	}
	fn set_requester() -> Weight {
		(100_320_000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn remove_requester() -> Weight {
		(77_638_000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn request_new_round() -> Weight {
		(176_748_000 as Weight)
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
	fn withdraw_payment() -> Weight {
		(165_609_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	fn transfer_admin() -> Weight {
		(65_743_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn accept_admin() -> Weight {
		(63_043_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn withdraw_funds() -> Weight {
		(145_198_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn reduce_debt() -> Weight {
		(88_091_000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn transfer_pallet_admin() -> Weight {
		(50_395_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn accept_pallet_admin() -> Weight {
		(57_802_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn set_feed_creator() -> Weight {
		(56_021_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn remove_feed_creator() -> Weight {
		(56_855_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
}
