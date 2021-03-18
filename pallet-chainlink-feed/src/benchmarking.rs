use super::*;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::{assert_ok, traits::Get};
use frame_system::RawOrigin;
use sp_runtime::traits::Bounded;

use crate::Module as ChainlinkFeed;

const SEED: u32 = 0;

benchmarks! {
	_ {}

	create_feed {
		let o in 1 .. T::OracleCountLimit::get();

		let caller: T::AccountId = whitelisted_caller();
		let pallet_admin: T::AccountId = ChainlinkFeed::<T>::pallet_admin();
		assert_ok!(ChainlinkFeed::<T>::set_feed_creator(RawOrigin::Signed(pallet_admin.clone()).into(), caller.clone()));
		let admin: T::AccountId = account("oracle_admin", 0, SEED);
		let oracles: Vec<(T::AccountId, T::AccountId)> = (0..o).map(|n| (account("oracle", n, SEED), admin.clone())).collect();
		frame_support::debug::debug!("before benchmark");
	}: _(
			RawOrigin::Signed(caller.clone()),
			50u32.into(),
			Zero::zero(),
			(1u8.into(), 100u8.into()),
			1u8.into(),
			5u8.into(),
			b"desc".to_vec(),
			Zero::zero(),
			oracles
		)
	verify {
		let feed: T::FeedId = Zero::zero();
		assert_eq!(ChainlinkFeed::<T>::feed_config(feed).expect("feed should be there").oracle_count, o);
	}

	transfer_ownership {
		let caller: T::AccountId = whitelisted_caller();
		let pallet_admin: T::AccountId = ChainlinkFeed::<T>::pallet_admin();
		assert_ok!(ChainlinkFeed::<T>::set_feed_creator(RawOrigin::Signed(pallet_admin.clone()).into(), caller.clone()));
		let oracle: T::AccountId = account("oracle", 0, SEED);
		let admin: T::AccountId = account("oracle_admin", 0, SEED);
		assert_ok!(ChainlinkFeed::<T>::create_feed(
			RawOrigin::Signed(caller.clone()).into(),
			50u32.into(),
			Zero::zero(),
			(1u8.into(), 100u8.into()),
			1u8.into(),
			5u8.into(),
			b"desc".to_vec(),
			Zero::zero(),
			vec![(oracle, admin)],
		));
		let feed = Zero::zero();
		let new_owner: T::AccountId = account("new_owner", 0, SEED);
	}: _(RawOrigin::Signed(caller.clone()), feed, new_owner.clone())
	verify {
		assert_eq!(ChainlinkFeed::<T>::feed_config(feed).expect("feed should be there").pending_owner, Some(new_owner));
	}

	accept_ownership {
		let caller: T::AccountId = whitelisted_caller();
		let pallet_admin: T::AccountId = ChainlinkFeed::<T>::pallet_admin();
		assert_ok!(ChainlinkFeed::<T>::set_feed_creator(RawOrigin::Signed(pallet_admin.clone()).into(), caller.clone()));
		let oracle: T::AccountId = account("oracle", 0, SEED);
		let admin: T::AccountId = account("oracle_admin", 0, SEED);
		assert_ok!(ChainlinkFeed::<T>::create_feed(
			RawOrigin::Signed(caller.clone()).into(),
			50u32.into(),
			Zero::zero(),
			(1u8.into(), 100u8.into()),
			1u8.into(),
			5u8.into(),
			b"desc".to_vec(),
			Zero::zero(),
			vec![(oracle, admin)],
		));
		let feed = Zero::zero();
		let new_owner: T::AccountId = account("new_owner", 0, SEED);
		assert_ok!(ChainlinkFeed::<T>::transfer_ownership(RawOrigin::Signed(caller.clone()).into(), feed, new_owner.clone()));
	}: _(RawOrigin::Signed(new_owner.clone()), feed)
	verify {
		assert_eq!(ChainlinkFeed::<T>::feed_config(feed).expect("feed should be there").owner, new_owner);
	}

	submit {
		let o in 1 .. T::OracleCountLimit::get();

		let caller: T::AccountId = whitelisted_caller();
		let pallet_admin: T::AccountId = ChainlinkFeed::<T>::pallet_admin();
		assert_ok!(ChainlinkFeed::<T>::set_feed_creator(RawOrigin::Signed(pallet_admin.clone()).into(), caller.clone()));
		let admin: T::AccountId = account("oracle_admin", 0, SEED);
		let oracles: Vec<(T::AccountId, T::AccountId)> = (0..o).map(|n| (account("oracle", n, SEED), admin.clone())).collect();
		frame_support::debug::debug!("before benchmark");
		assert_ok!(ChainlinkFeed::<T>::create_feed(
			RawOrigin::Signed(caller.clone()).into(),
			50u32.into(),
			Zero::zero(),
			(1u8.into(), 100u8.into()),
			oracles.len() as u32,
			5u8.into(),
			b"desc".to_vec(),
			Zero::zero(),
			oracles.clone(),
		));
		let feed: T::FeedId = Zero::zero();
		let round: T::RoundId = One::one();
		for (o, _a) in oracles.iter().skip(1) {
			assert_ok!(ChainlinkFeed::<T>::submit(RawOrigin::Signed(o.clone()).into(), feed, 1u8.into(), 5u8.into()));
		}
		let oracle = oracles.first().map(|(o, _a)| o.clone()).expect("first oracle should be there");
		assert_eq!(ChainlinkFeed::<T>::round(feed, round), Some(Round::new(Zero::zero())));
		// assert_eq!(ChainlinkFeed::<T>::round(feed, round), None);
	}: _(
			RawOrigin::Signed(oracle.clone()),
			feed,
			1u8.into(),
			5u8.into()
		)
	verify {
		assert_eq!(ChainlinkFeed::<T>::round(feed, round), Some(Round { started_at: One::one(), answer: Some(5u8.into()), updated_at: Some(One::one()), answered_in_round: Some(1u8.into()) }));
	}

	// force_create {
	// 	let caller: T::AccountId = whitelisted_caller();
	// 	let caller_lookup = T::Lookup::unlookup(caller.clone());
	// }: _(SystemOrigin::Root, Default::default(), caller_lookup, 1, 1u32.into())
	// verify {
	// 	assert_last_event::<T>(Event::ForceCreated(Default::default(), caller).into());
	// }

	// destroy {
	// 	let z in 0 .. 10_000;
	// 	let (caller, _) = create_default_asset::<T>(10_000);
	// 	add_zombies::<T>(caller.clone(), z);
	// }: _(SystemOrigin::Signed(caller), Default::default(), 10_000)
	// verify {
	// 	assert_last_event::<T>(Event::Destroyed(Default::default()).into());
	// }

	// force_destroy {
	// 	let z in 0 .. 10_000;
	// 	let (caller, _) = create_default_asset::<T>(10_000);
	// 	add_zombies::<T>(caller.clone(), z);
	// }: _(SystemOrigin::Root, Default::default(), 10_000)
	// verify {
	// 	assert_last_event::<T>(Event::Destroyed(Default::default()).into());
	// }

	// mint {
	// 	let (caller, caller_lookup) = create_default_asset::<T>(10);
	// 	let amount = T::Balance::from(100u32);
	// }: _(SystemOrigin::Signed(caller.clone()), Default::default(), caller_lookup, amount)
	// verify {
	// 	assert_last_event::<T>(Event::Issued(Default::default(), caller, amount).into());
	// }

	// burn {
	// 	let amount = T::Balance::from(100u32);
	// 	let (caller, caller_lookup) = create_default_minted_asset::<T>(10, amount);
	// }: _(SystemOrigin::Signed(caller.clone()), Default::default(), caller_lookup, amount)
	// verify {
	// 	assert_last_event::<T>(Event::Burned(Default::default(), caller, amount).into());
	// }

	// transfer {
	// 	let amount = T::Balance::from(100u32);
	// 	let (caller, caller_lookup) = create_default_minted_asset::<T>(10, amount);
	// 	let target: T::AccountId = account("target", 0, SEED);
	// 	let target_lookup = T::Lookup::unlookup(target.clone());
	// }: _(SystemOrigin::Signed(caller.clone()), Default::default(), target_lookup, amount)
	// verify {
	// 	assert_last_event::<T>(Event::Transferred(Default::default(), caller, target, amount).into());
	// }

	// force_transfer {
	// 	let amount = T::Balance::from(100u32);
	// 	let (caller, caller_lookup) = create_default_minted_asset::<T>(10, amount);
	// 	let target: T::AccountId = account("target", 0, SEED);
	// 	let target_lookup = T::Lookup::unlookup(target.clone());
	// }: _(SystemOrigin::Signed(caller.clone()), Default::default(), caller_lookup, target_lookup, amount)
	// verify {
	// 	assert_last_event::<T>(
	// 		Event::ForceTransferred(Default::default(), caller, target, amount).into()
	// 	);
	// }

	// freeze {
	// 	let (caller, caller_lookup) = create_default_minted_asset::<T>(10, 100u32.into());
	// }: _(SystemOrigin::Signed(caller.clone()), Default::default(), caller_lookup)
	// verify {
	// 	assert_last_event::<T>(Event::Frozen(Default::default(), caller).into());
	// }

	// thaw {
	// 	let (caller, caller_lookup) = create_default_minted_asset::<T>(10, 100u32.into());
	// 	Assets::<T>::freeze(
	// 		SystemOrigin::Signed(caller.clone()).into(),
	// 		Default::default(),
	// 		caller_lookup.clone(),
	// 	)?;
	// }: _(SystemOrigin::Signed(caller.clone()), Default::default(), caller_lookup)
	// verify {
	// 	assert_last_event::<T>(Event::Thawed(Default::default(), caller).into());
	// }

	// freeze_asset {
	// 	let (caller, caller_lookup) = create_default_minted_asset::<T>(10, 100u32.into());
	// }: _(SystemOrigin::Signed(caller.clone()), Default::default())
	// verify {
	// 	assert_last_event::<T>(Event::AssetFrozen(Default::default()).into());
	// }

	// thaw_asset {
	// 	let (caller, caller_lookup) = create_default_minted_asset::<T>(10, 100u32.into());
	// 	Assets::<T>::freeze_asset(
	// 		SystemOrigin::Signed(caller.clone()).into(),
	// 		Default::default(),
	// 	)?;
	// }: _(SystemOrigin::Signed(caller.clone()), Default::default())
	// verify {
	// 	assert_last_event::<T>(Event::AssetThawed(Default::default()).into());
	// }

	// transfer_ownership {
	// 	let (caller, _) = create_default_asset::<T>(10);
	// 	let target: T::AccountId = account("target", 0, SEED);
	// 	let target_lookup = T::Lookup::unlookup(target.clone());
	// }: _(SystemOrigin::Signed(caller), Default::default(), target_lookup)
	// verify {
	// 	assert_last_event::<T>(Event::OwnerChanged(Default::default(), target).into());
	// }

	// set_team {
	// 	let (caller, _) = create_default_asset::<T>(10);
	// 	let target0 = T::Lookup::unlookup(account("target", 0, SEED));
	// 	let target1 = T::Lookup::unlookup(account("target", 1, SEED));
	// 	let target2 = T::Lookup::unlookup(account("target", 2, SEED));
	// }: _(SystemOrigin::Signed(caller), Default::default(), target0.clone(), target1.clone(), target2.clone())
	// verify {
	// 	assert_last_event::<T>(Event::TeamChanged(
	// 		Default::default(),
	// 		account("target", 0, SEED),
	// 		account("target", 1, SEED),
	// 		account("target", 2, SEED),
	// 	).into());
	// }

	// set_max_zombies {
	// 	let (caller, _) = create_default_asset::<T>(10);
	// 	let max_zombies: u32 = 100;
	// 	T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
	// }: _(SystemOrigin::Signed(caller), Default::default(), max_zombies)
	// verify {
	// 	assert_last_event::<T>(Event::MaxZombiesChanged(Default::default(), max_zombies).into());
	// }

	// set_metadata {
	// 	let n in 0 .. T::StringLimit::get();
	// 	let s in 0 .. T::StringLimit::get();

	// 	let name = vec![0u8; n as usize];
	// 	let symbol = vec![0u8; s as usize];
	// 	let decimals = 12;

	// 	let (caller, _) = create_default_asset::<T>(10);
	// 	T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
	// }: _(SystemOrigin::Signed(caller), Default::default(), name.clone(), symbol.clone(), decimals)
	// verify {
	// 	assert_last_event::<T>(Event::MetadataSet(Default::default(), name, symbol, decimals).into());
	// }
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests::{new_test_ext, Test};

	#[test]
	fn create_feed() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_create_feed::<Test>());
		});
	}

	#[test]
	fn transfer_ownership() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_transfer_ownership::<Test>());
		});
	}

	#[test]
	fn accept_ownership() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_accept_ownership::<Test>());
		});
	}

	#[test]
	fn submit() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_submit::<Test>());
		});
	}
}
