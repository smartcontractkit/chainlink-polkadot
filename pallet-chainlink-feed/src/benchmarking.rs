use super::*;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::traits::Get;
use frame_system::RawOrigin;
use sp_std::fmt::Debug;

use crate::Module as ChainlinkFeed;

const SEED: u32 = 0;

fn assert_is_ok<T: Debug, E: Debug>(r: Result<T, E>) {
	#[cfg(feature = "std")]
	frame_support::assert_ok!(r);
	#[cfg(not(feature = "std"))]
	assert!(r.is_ok());
}

benchmarks! {
	_ {}

	create_feed {
		let o in 1 .. T::OracleCountLimit::get();

		let caller: T::AccountId = whitelisted_caller();
		let pallet_admin: T::AccountId = ChainlinkFeed::<T>::pallet_admin();
		assert_is_ok(ChainlinkFeed::<T>::set_feed_creator(RawOrigin::Signed(pallet_admin.clone()).into(), caller.clone()));
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
		assert_is_ok(ChainlinkFeed::<T>::set_feed_creator(RawOrigin::Signed(pallet_admin.clone()).into(), caller.clone()));
		let oracle: T::AccountId = account("oracle", 0, SEED);
		let admin: T::AccountId = account("oracle_admin", 0, SEED);
		assert_is_ok(ChainlinkFeed::<T>::create_feed(
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
		assert_is_ok(ChainlinkFeed::<T>::set_feed_creator(RawOrigin::Signed(pallet_admin.clone()).into(), caller.clone()));
		let oracle: T::AccountId = account("oracle", 0, SEED);
		let admin: T::AccountId = account("oracle_admin", 0, SEED);
		assert_is_ok(ChainlinkFeed::<T>::create_feed(
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
		assert_is_ok(ChainlinkFeed::<T>::transfer_ownership(RawOrigin::Signed(caller.clone()).into(), feed, new_owner.clone()));
	}: _(RawOrigin::Signed(new_owner.clone()), feed)
	verify {
		assert_eq!(ChainlinkFeed::<T>::feed_config(feed).expect("feed should be there").owner, new_owner);
	}

	submit_opening_round_answers {
		let o = 3;
		let caller: T::AccountId = whitelisted_caller();
		let pallet_admin: T::AccountId = ChainlinkFeed::<T>::pallet_admin();
		assert_is_ok(ChainlinkFeed::<T>::set_feed_creator(RawOrigin::Signed(pallet_admin.clone()).into(), caller.clone()));
		let admin: T::AccountId = account("oracle_admin", 0, SEED);
		let oracles: Vec<(T::AccountId, T::AccountId)> = (0..o).map(|n| (account("oracle", n, SEED), admin.clone())).collect();
		frame_support::debug::debug!("before benchmark");
		assert_is_ok(ChainlinkFeed::<T>::create_feed(
			RawOrigin::Signed(caller.clone()).into(),
			50u32.into(),
			Zero::zero(),
			(1u8.into(), 100u8.into()),
			1,
			5u8.into(),
			b"desc".to_vec(),
			Zero::zero(),
			oracles.clone(),
		));
		let feed: T::FeedId = Zero::zero();
		let round: T::RoundId = One::one();
		let answer: T::Value = 5u8.into();
		let oracle = oracles.first().map(|(o, _a)| o.clone()).expect("first oracle should be there");
		assert_eq!(ChainlinkFeed::<T>::round(feed, round), None);
	}: submit(
			RawOrigin::Signed(oracle.clone()),
			feed,
			round,
			answer
		)
	verify {
		let expected_round = Round {
			started_at: One::one(),
			answer: Some(answer),
			updated_at: Some(One::one()),
			answered_in_round: Some(1u8.into())
		};
		assert_eq!(ChainlinkFeed::<T>::round(feed, round), Some(expected_round));
	}

	submit_closing_answer {
		let o in 2 .. T::OracleCountLimit::get();

		let caller: T::AccountId = whitelisted_caller();
		let pallet_admin: T::AccountId = ChainlinkFeed::<T>::pallet_admin();
		assert_is_ok(ChainlinkFeed::<T>::set_feed_creator(RawOrigin::Signed(pallet_admin.clone()).into(), caller.clone()));
		let admin: T::AccountId = account("oracle_admin", 0, SEED);
		let oracles: Vec<(T::AccountId, T::AccountId)> = (0..o).map(|n| (account("oracle", n, SEED), admin.clone())).collect();
		frame_support::debug::debug!("before benchmark");
		assert_is_ok(ChainlinkFeed::<T>::create_feed(
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
		let answer: T::Value = 42u8.into();
		for (o, _a) in oracles.iter().skip(1) {
			assert_is_ok(ChainlinkFeed::<T>::submit(RawOrigin::Signed(o.clone()).into(), feed, 1u8.into(), answer));
		}
		let oracle = oracles.first().map(|(o, _a)| o.clone()).expect("first oracle should be there");
		assert_eq!(ChainlinkFeed::<T>::round(feed, round), Some(Round::new(Zero::zero())));
	}: submit(
			RawOrigin::Signed(oracle.clone()),
			feed,
			round,
			answer
		)
	verify {
		let expected_round = Round {
			started_at: Zero::zero(),
			answer: Some(answer),
			updated_at: Some(One::one()),
			answered_in_round: Some(1u8.into())
		};
		assert_eq!(ChainlinkFeed::<T>::round(feed, round), Some(expected_round));
	}

	change_oracles {
		let o in 1 .. T::OracleCountLimit::get();

		let caller: T::AccountId = whitelisted_caller();
		let pallet_admin: T::AccountId = ChainlinkFeed::<T>::pallet_admin();
		assert_is_ok(ChainlinkFeed::<T>::set_feed_creator(RawOrigin::Signed(pallet_admin.clone()).into(), caller.clone()));
		let admin: T::AccountId = account("oracle_admin", 0, SEED);
		let oracles: Vec<(T::AccountId, T::AccountId)> = (0..o).map(|n| (account("oracle", n, SEED), admin.clone())).collect();
		let oracles_after: Vec<(T::AccountId, T::AccountId)> = (0..o).map(|n| (account("new_oracle", n, SEED), admin.clone())).collect();
		frame_support::debug::debug!("before benchmark");
		assert_is_ok(ChainlinkFeed::<T>::create_feed(
			RawOrigin::Signed(caller.clone()).into(),
			50u32.into(),
			Zero::zero(),
			(1u8.into(), 100u8.into()),
			1u8.into(),
			5u8.into(),
			b"desc".to_vec(),
			Zero::zero(),
			oracles.clone(),
		));
		let oracles_before = oracles.into_iter().map(|(o, _a)| o).collect();
		let feed: T::FeedId = Zero::zero();
	}: _(
			RawOrigin::Signed(caller.clone()),
			feed,
			oracles_before,
			oracles_after
		)
	verify {
		assert_eq!(ChainlinkFeed::<T>::feed_config(feed).expect("feed should be there").oracle_count, o);
	}

	update_future_rounds {
		let o = 2;
		let caller: T::AccountId = whitelisted_caller();
		let pallet_admin: T::AccountId = ChainlinkFeed::<T>::pallet_admin();
		assert_is_ok(ChainlinkFeed::<T>::set_feed_creator(RawOrigin::Signed(pallet_admin.clone()).into(), caller.clone()));
		let admin: T::AccountId = account("oracle_admin", 0, SEED);
		let oracles: Vec<(T::AccountId, T::AccountId)> = (0..o).map(|n| (account("oracle", n, SEED), admin.clone())).collect();
		frame_support::debug::debug!("before benchmark");
		assert_is_ok(ChainlinkFeed::<T>::create_feed(
			RawOrigin::Signed(caller.clone()).into(),
			50u32.into(),
			Zero::zero(),
			(1u8.into(), 100u8.into()),
			1u8.into(),
			5u8.into(),
			b"desc".to_vec(),
			Zero::zero(),
			oracles.clone(),
		));
		let payment: BalanceOf<T> = 42u32.into();
		let timeout: T::BlockNumber = 3u8.into();
		let feed: T::FeedId = Zero::zero();
	}: _(
			RawOrigin::Signed(caller.clone()),
			feed,
			payment,
			(1, oracles.len() as u32),
			1u8.into(),
			timeout
		)
	verify {
		let config = ChainlinkFeed::<T>::feed_config(feed).expect("feed should be there");
		assert_eq!(config.payment, payment);
		assert_eq!(config.timeout, timeout);
	}

	prune {
		let r in 1u32 .. 1_000u32;

		let caller: T::AccountId = whitelisted_caller();
		let pallet_admin: T::AccountId = ChainlinkFeed::<T>::pallet_admin();
		assert_is_ok(ChainlinkFeed::<T>::set_feed_creator(RawOrigin::Signed(pallet_admin.clone()).into(), caller.clone()));
		let oracle: T::AccountId = account("oracle", 0, SEED);
		let admin: T::AccountId = account("oracle_admin", 0, SEED);
		assert_is_ok(ChainlinkFeed::<T>::create_feed(
			RawOrigin::Signed(caller.clone()).into(),
			50u32.into(),
			Zero::zero(),
			(1u8.into(), 100u8.into()),
			1u8.into(),
			5u8.into(),
			b"desc".to_vec(),
			Zero::zero(),
			vec![(oracle.clone(), admin)],
		));
		let feed = Zero::zero();
		let answer: T::Value = 42u8.into();
		let pruning_window: u32 = T::PruningWindow::get().try_into().map_err(|_|()).expect("should be able to convert into u32");
		for round in 1..(pruning_window + r + 2) {
			let round = T::RoundId::from(round);
			assert_is_ok(ChainlinkFeed::<T>::submit(RawOrigin::Signed(oracle.clone()).into(), feed, round, answer));
		}
		let r = T::RoundId::from(r);
	}: _(
		RawOrigin::Signed(caller.clone()),
		feed,
		1u8.into(),
		r + One::one()
	)
	verify {
		// rounds until `r` should be pruned
		assert_eq!(ChainlinkFeed::<T>::round(feed, T::RoundId::one()), None);
		assert_eq!(ChainlinkFeed::<T>::round(feed, r), None);
		let expected_round = Round {
			started_at: Zero::zero(),
			answer: Some(answer),
			updated_at: Some(Zero::zero()),
			answered_in_round: Some(r + One::one())
		};
		// round `r+1` should be kept
		assert_eq!(ChainlinkFeed::<T>::round(feed, r + T::RoundId::one()), Some(expected_round));
	}

	set_requester {
		let caller: T::AccountId = whitelisted_caller();
		let pallet_admin: T::AccountId = ChainlinkFeed::<T>::pallet_admin();
		assert_is_ok(ChainlinkFeed::<T>::set_feed_creator(RawOrigin::Signed(pallet_admin.clone()).into(), caller.clone()));
		let oracle: T::AccountId = account("oracle", 0, SEED);
		let admin: T::AccountId = account("oracle_admin", 0, SEED);
		assert_is_ok(ChainlinkFeed::<T>::create_feed(
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
		let requester: T::AccountId = account("requester", 0, SEED);
		let delay: T::RoundId = 3u8.into();
	}: _(RawOrigin::Signed(caller.clone()), feed, requester.clone(), delay)
	verify {
		assert_eq!(ChainlinkFeed::<T>::requester(feed, requester).expect("feed should be there").delay, delay);
	}

	remove_requester {
		let caller: T::AccountId = whitelisted_caller();
		let pallet_admin: T::AccountId = ChainlinkFeed::<T>::pallet_admin();
		assert_is_ok(ChainlinkFeed::<T>::set_feed_creator(RawOrigin::Signed(pallet_admin.clone()).into(), caller.clone()));
		let oracle: T::AccountId = account("oracle", 0, SEED);
		let admin: T::AccountId = account("oracle_admin", 0, SEED);
		assert_is_ok(ChainlinkFeed::<T>::create_feed(
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
		let requester: T::AccountId = account("requester", 0, SEED);
		let delay: T::RoundId = 3u8.into();
		assert_is_ok(ChainlinkFeed::<T>::set_requester(RawOrigin::Signed(caller.clone()).into(), feed, requester.clone(), delay));
	}: _(RawOrigin::Signed(caller.clone()), feed, requester.clone())
	verify {
		assert_eq!(ChainlinkFeed::<T>::requester(feed, requester), None);
	}

	request_new_round {
		let o = 3;
		let caller: T::AccountId = whitelisted_caller();
		let pallet_admin: T::AccountId = ChainlinkFeed::<T>::pallet_admin();
		assert_is_ok(ChainlinkFeed::<T>::set_feed_creator(RawOrigin::Signed(pallet_admin.clone()).into(), caller.clone()));
		let admin: T::AccountId = account("oracle_admin", 0, SEED);
		let oracles: Vec<(T::AccountId, T::AccountId)> = (0..o).map(|n| (account("oracle", n, SEED), admin.clone())).collect();
		frame_support::debug::debug!("before benchmark");
		assert_is_ok(ChainlinkFeed::<T>::create_feed(
			RawOrigin::Signed(caller.clone()).into(),
			50u32.into(),
			Zero::zero(),
			(1u8.into(), 100u8.into()),
			1,
			5u8.into(),
			b"desc".to_vec(),
			Zero::zero(),
			oracles.clone(),
		));
		let feed: T::FeedId = Zero::zero();
		let round: T::RoundId = One::one();
		let answer: T::Value = 5u8.into();
		let oracle = oracles.first().map(|(o, _a)| o.clone()).expect("first oracle should be there");
		assert_is_ok(ChainlinkFeed::<T>::submit(
			RawOrigin::Signed(oracle.clone()).into(),
			feed,
			round,
			answer
		));
		let config = ChainlinkFeed::<T>::feed_config(feed).expect("config should be there");
		assert_eq!(config.reporting_round, round);
		let requester: T::AccountId = account("requester", 0, SEED);
		let delay: T::RoundId = 3u8.into();
		assert_is_ok(ChainlinkFeed::<T>::set_requester(RawOrigin::Signed(caller.clone()).into(), feed, requester.clone(), delay));
	}: _(
			RawOrigin::Signed(requester.clone()),
			feed
		)
	verify {
		let config = ChainlinkFeed::<T>::feed_config(feed).expect("config should be there");
		assert_eq!(config.reporting_round, 2u8.into());
	}

	withdraw_payment {
		let o = 3;
		let caller: T::AccountId = whitelisted_caller();
		let pallet_admin: T::AccountId = ChainlinkFeed::<T>::pallet_admin();
		assert_is_ok(ChainlinkFeed::<T>::set_feed_creator(RawOrigin::Signed(pallet_admin.clone()).into(), caller.clone()));
		let admin: T::AccountId = account("oracle_admin", 0, SEED);
		let oracles: Vec<(T::AccountId, T::AccountId)> = (0..o).map(|n| (account("oracle", n, SEED), admin.clone())).collect();
		frame_support::debug::debug!("before benchmark");
		let payment: BalanceOf<T> = 50u32.into();
		assert_is_ok(ChainlinkFeed::<T>::create_feed(
			RawOrigin::Signed(caller.clone()).into(),
			payment,
			Zero::zero(),
			(1u8.into(), 100u8.into()),
			1,
			5u8.into(),
			b"desc".to_vec(),
			Zero::zero(),
			oracles.clone(),
		));
		let feed: T::FeedId = Zero::zero();
		let round: T::RoundId = One::one();
		let answer: T::Value = 5u8.into();
		let oracle = oracles.first().map(|(o, _a)| o.clone()).expect("first oracle should be there");
		assert_is_ok(ChainlinkFeed::<T>::submit(
			RawOrigin::Signed(oracle.clone()).into(),
			feed,
			round,
			answer
		));
		let recipient: T::AccountId = account("recipient", 0, SEED);
	}: _(
			RawOrigin::Signed(admin.clone()),
			oracle.clone(),
			recipient.clone(),
			payment
		)
	verify {
		assert_eq!(T::Currency::free_balance(&recipient), payment);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests::{new_test_ext, Test};
	use frame_support::assert_ok;

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
	fn submit_opening_round_answers() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_submit_opening_round_answers::<Test>());
		});
	}

	#[test]
	fn submit_closing_answer() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_submit_closing_answer::<Test>());
		});
	}

	#[test]
	fn change_oracles() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_change_oracles::<Test>());
		});
	}

	#[test]
	fn update_future_rounds() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_update_future_rounds::<Test>());
		});
	}

	#[test]
	fn prune() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_prune::<Test>());
		});
	}

	#[test]
	fn set_requester() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_set_requester::<Test>());
		});
	}

	#[test]
	fn remove_requester() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_remove_requester::<Test>());
		});
	}

	#[test]
	fn request_new_round() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_request_new_round::<Test>());
		});
	}

	#[test]
	fn withdraw_payment() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_withdraw_payment::<Test>());
		});
	}
}
