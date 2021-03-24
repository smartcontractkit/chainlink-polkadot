//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.1

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{Weight, constants::RocksDbWeight as DbWeight};

pub struct WeightInfo;
impl pallet_chainlink_feed::WeightInfo for WeightInfo {
	fn create_feed(o: u32, ) -> Weight {
		(263_994_000 as Weight)
			.saturating_add((70_097_000 as Weight).saturating_mul(o as Weight))
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().reads((2 as Weight).saturating_mul(o as Weight)))
			.saturating_add(DbWeight::get().writes(3 as Weight))
			.saturating_add(DbWeight::get().writes((2 as Weight).saturating_mul(o as Weight)))
	}
	fn transfer_ownership() -> Weight {
		(79_481_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn accept_ownership() -> Weight {
		(76_528_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn submit_opening_round_answers() -> Weight {
		(376_956_000 as Weight)
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().writes(6 as Weight))
	}
	fn submit_closing_answer(o: u32, ) -> Weight {
		(304_767_000 as Weight)
			.saturating_add((1_555_000 as Weight).saturating_mul(o as Weight))
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().writes(6 as Weight))
	}
	fn change_oracles(d: u32, n: u32, ) -> Weight {
		(0 as Weight)
			.saturating_add((153_090_000 as Weight).saturating_mul(d as Weight))
			.saturating_add((67_931_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().reads((1 as Weight).saturating_mul(d as Weight)))
			.saturating_add(DbWeight::get().reads((2 as Weight).saturating_mul(n as Weight)))
			.saturating_add(DbWeight::get().writes(1 as Weight))
			.saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(d as Weight)))
			.saturating_add(DbWeight::get().writes((2 as Weight).saturating_mul(n as Weight)))
	}
	fn update_future_rounds() -> Weight {
		(113_287_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn prune(r: u32, ) -> Weight {
		(0 as Weight)
			.saturating_add((21_667_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
			.saturating_add(DbWeight::get().writes((2 as Weight).saturating_mul(r as Weight)))
	}
	fn set_requester() -> Weight {
		(90_332_000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn remove_requester() -> Weight {
		(78_258_000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn request_new_round() -> Weight {
		(190_407_000 as Weight)
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
	fn withdraw_payment() -> Weight {
		(180_234_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	fn transfer_admin() -> Weight {
		(70_214_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn accept_admin() -> Weight {
		(68_352_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn withdraw_funds() -> Weight {
		(161_843_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn reduce_debt() -> Weight {
		(97_857_000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn transfer_pallet_admin() -> Weight {
		(54_232_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn accept_pallet_admin() -> Weight {
		(62_670_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn set_feed_creator() -> Weight {
		(60_644_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn remove_feed_creator() -> Weight {
		(60_381_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
}
