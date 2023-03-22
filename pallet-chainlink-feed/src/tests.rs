use super::*;
use crate::{mock::*, utils::with_transaction_result, Error};
use frame_support::traits::ReservableCurrency;
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::AccountIdConversion,
	sp_runtime::traits::{One, Zero},
	traits::Currency,
};
use sp_runtime::DispatchError;
use std::convert::TryInto;

type Balances = pallet_balances::Pallet<Test>;

#[test]
fn feed_creation_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(ChainlinkFeed::create_feed(
			RuntimeOrigin::signed(1),
			20,
			10,
			(10, 1_000),
			3,
			5,
			b"desc".to_vec(),
			2,
			vec![(1, 4), (2, 4), (3, 4)],
			None,
			None,
		));
	});
}

#[test]
fn feed_creation_failure_cases() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			FeedBuilder::new().owner(123).build_and_store(),
			Error::<Test>::NotFeedCreator
		);
		assert_noop!(
			FeedBuilder::new()
				.description(b"waaaaaaaaaaaaaaaaay too long".to_vec())
				.build_and_store(),
			Error::<Test>::DescriptionTooLong
		);
		let too_many_oracles = (0..(OracleLimit::get() + 1))
			.into_iter()
			.map(|i| (i as u64, i as u64))
			.collect();
		assert_noop!(
			FeedBuilder::new()
				.oracles(too_many_oracles)
				.build_and_store(),
			Error::<Test>::OraclesLimitExceeded
		);
		assert_noop!(
			FeedBuilder::new()
				.min_submissions(3)
				.oracles(vec![(1, 2)])
				.build_and_store(),
			Error::<Test>::WrongBounds
		);
		assert_noop!(
			FeedBuilder::new()
				.min_submissions(0)
				.oracles(vec![(1, 2)])
				.build_and_store(),
			Error::<Test>::WrongBounds
		);
		assert_noop!(
			FeedBuilder::new()
				.oracles(vec![(1, 2), (2, 3), (3, 4)])
				.restart_delay(3)
				.build_and_store(),
			Error::<Test>::DelayNotBelowCount
		);

		for _feed in 0..FeedLimit::get() {
			assert_ok!(FeedBuilder::new().build_and_store());
		}
		assert_noop!(
			FeedBuilder::new().build_and_store(),
			Error::<Test>::FeedLimitReached
		);
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
			RuntimeOrigin::signed(oracle),
			feed_id,
			round_id,
			submission
		));
		let second_oracle = 3;
		assert_ok!(ChainlinkFeed::submit(
			RuntimeOrigin::signed(second_oracle),
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
fn on_answer_callback_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let payment = 20;
		let timeout = 10;
		let min_submissions = 1;
		let oracles = vec![(1, 4), (2, 4)];
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
			RuntimeOrigin::signed(oracle),
			feed_id,
			round_id,
			submission
		));

		assert_eq!(
			System::events()
				.into_iter()
				.map(|r| r.event)
				.filter_map(|e| {
					if let mock::RuntimeEvent::ChainlinkFeed(crate::Event::NewData(feed, data)) = e
					{
						Some((feed, data))
					} else {
						None
					}
				})
				.last()
				.unwrap(),
			(
				0,
				RoundData {
					started_at: 1,
					answer: 42,
					updated_at: 1,
					answered_in_round: 1
				}
			)
		);
	});
}

#[test]
fn details_are_cleared() {
	new_test_ext().execute_with(|| {
		let payment = 20;
		let timeout = 10;
		let min_submissions = 2;
		let oracle = 2;
		let snd_oracle = 3;
		let oracles = vec![(1, 4), (oracle, 4), (snd_oracle, 4)];
		assert_ok!(FeedBuilder::new()
			.payment(payment)
			.timeout(timeout)
			.min_submissions(min_submissions)
			.oracles(oracles)
			.build_and_store());

		let feed_id = 0;
		{
			// round 1
			let r = 1;
			let submission = 42;
			let answer = submission;
			assert_ok!(ChainlinkFeed::submit(
				RuntimeOrigin::signed(oracle),
				feed_id,
				r,
				submission
			));
			assert_ok!(ChainlinkFeed::submit(
				RuntimeOrigin::signed(snd_oracle),
				feed_id,
				r,
				submission
			));
			let round = ChainlinkFeed::round(feed_id, r).unwrap();
			assert_eq!(round.answer, Some(answer));
			let details = ChainlinkFeed::round_details(feed_id, r).unwrap();
			assert_eq!(details.submissions, vec![submission, submission]);
			let oracle_status = ChainlinkFeed::oracle_status(feed_id, oracle).unwrap();
			assert_eq!(oracle_status.latest_submission, Some(submission));
		}
		{
			// round 2
			let r = 2;
			let submission = 21;
			let answer = submission;
			// switch the order because `oracle` is not allowed
			// to start a new round
			assert_ok!(ChainlinkFeed::submit(
				RuntimeOrigin::signed(snd_oracle),
				feed_id,
				r,
				submission
			));
			assert_ok!(ChainlinkFeed::submit(
				RuntimeOrigin::signed(oracle),
				feed_id,
				r,
				submission
			));
			let round = ChainlinkFeed::round(feed_id, r).unwrap();
			assert_eq!(round.answer, Some(answer));
			let details = ChainlinkFeed::round_details(feed_id, r).unwrap();
			assert_eq!(details.submissions, vec![submission, submission]);
			let oracle_status = ChainlinkFeed::oracle_status(feed_id, oracle).unwrap();
			assert_eq!(oracle_status.latest_submission, Some(submission));
			// old round details should be gone
			assert_eq!(ChainlinkFeed::round_details(feed_id, 1), None);
		}
	});
}

#[test]
fn submit_failure_cases() {
	new_test_ext().execute_with(|| {
		let oracle = 2;
		let oracles = vec![(1, 4), (oracle, 4), (3, 4)];
		assert_ok!(FeedBuilder::new()
			.value_bounds(1, 100)
			.oracles(oracles)
			.build_and_store());

		let feed_id = 0;
		let no_feed = 1234;
		let round_id = 1;
		let submission = 42;
		assert_noop!(
			ChainlinkFeed::submit(RuntimeOrigin::signed(oracle), no_feed, round_id, submission),
			Error::<Test>::FeedNotFound
		);
		let not_oracle = 1337;
		assert_noop!(
			ChainlinkFeed::submit(
				RuntimeOrigin::signed(not_oracle),
				feed_id,
				round_id,
				submission
			),
			Error::<Test>::NotOracle
		);
		let invalid_round = 1337;
		assert_noop!(
			ChainlinkFeed::submit(
				RuntimeOrigin::signed(oracle),
				feed_id,
				invalid_round,
				submission
			),
			Error::<Test>::InvalidRound
		);
		let low_value = 0;
		assert_noop!(
			ChainlinkFeed::submit(RuntimeOrigin::signed(oracle), feed_id, round_id, low_value,),
			Error::<Test>::SubmissionBelowMinimum
		);
		let high_value = 13377331;
		assert_noop!(
			ChainlinkFeed::submit(RuntimeOrigin::signed(oracle), feed_id, round_id, high_value,),
			Error::<Test>::SubmissionAboveMaximum
		);
	});
}

#[test]
fn change_oracles_should_work() {
	new_test_ext().execute_with(|| {
		let oracle = 2;
		let admin = 4;
		let initial_oracles = vec![(1, admin), (oracle, admin), (3, admin)];
		assert_ok!(FeedBuilder::new()
			.oracles(initial_oracles.clone())
			.min_submissions(1)
			.build_and_store());
		for (o, _a) in initial_oracles.iter() {
			assert!(
				ChainlinkFeed::oracle(o).is_some(),
				"oracle should be present"
			);
		}
		let feed_id = 0;
		let owner = 1;
		let feed = ChainlinkFeed::feed_config(feed_id).expect("feed should be there");
		assert_eq!(feed.oracle_count, 3);

		let round = 1;
		let submission = 42;
		assert_ok!(ChainlinkFeed::submit(
			RuntimeOrigin::signed(oracle),
			feed_id,
			round,
			submission
		));

		let to_disable: Vec<u64> = initial_oracles
			.iter()
			.cloned()
			.take(2)
			.map(|(o, _a)| o)
			.collect();
		let to_add = vec![(6, 9), (7, 9), (8, 9)];
		// failing cases
		assert_noop!(
			ChainlinkFeed::change_oracles(
				RuntimeOrigin::signed(owner),
				123,
				to_disable.clone(),
				to_add.clone(),
			),
			Error::<Test>::FeedNotFound
		);
		assert_noop!(
			ChainlinkFeed::change_oracles(
				RuntimeOrigin::signed(123),
				feed_id,
				to_disable.clone(),
				to_add.clone(),
			),
			Error::<Test>::NotFeedOwner
		);
		// we cannot disable the oracles before adding them
		let cannot_disable = to_add.iter().cloned().take(2).map(|(o, _a)| o).collect();
		assert_noop!(
			ChainlinkFeed::change_oracles(
				RuntimeOrigin::signed(owner),
				feed_id,
				cannot_disable,
				to_add.clone(),
			),
			Error::<Test>::OracleNotFound
		);
		let too_many_oracles = (0..(OracleLimit::get() + 1))
			.into_iter()
			.map(|i| (i as u64, i as u64))
			.collect();
		assert_noop!(
			ChainlinkFeed::change_oracles(
				RuntimeOrigin::signed(owner),
				feed_id,
				to_disable.clone(),
				too_many_oracles,
			),
			Error::<Test>::OraclesLimitExceeded
		);
		let changed_admin = initial_oracles
			.iter()
			.cloned()
			.map(|(o, _a)| (o, 33))
			.collect();
		assert_noop!(
			ChainlinkFeed::change_oracles(
				RuntimeOrigin::signed(owner),
				feed_id,
				to_disable.clone(),
				changed_admin,
			),
			Error::<Test>::OwnerCannotChangeAdmin
		);
		let many_duplicates: Vec<AccountId> = initial_oracles
			.iter()
			.cloned()
			.chain(initial_oracles.iter().cloned())
			.map(|(o, _a)| o)
			.collect();
		assert_noop!(
			ChainlinkFeed::change_oracles(
				RuntimeOrigin::signed(owner),
				feed_id,
				many_duplicates,
				to_add.clone(),
			),
			Error::<Test>::NotEnoughOracles
		);
		let duplicates = vec![1, 1, 1];
		assert_noop!(
			ChainlinkFeed::change_oracles(
				RuntimeOrigin::signed(owner),
				feed_id,
				duplicates,
				to_add.clone(),
			),
			Error::<Test>::OracleDisabled
		);

		{
			tx_assert_ok!(ChainlinkFeed::feed_mut(feed_id)
				.unwrap()
				.request_new_round(AccountId::default()));
		}
		// successfully change oracles
		assert_ok!(ChainlinkFeed::change_oracles(
			RuntimeOrigin::signed(owner),
			feed_id,
			to_disable.clone(),
			to_add.clone(),
		));

		// we cannot disable the same oracles a second time
		assert_noop!(
			ChainlinkFeed::change_oracles(
				RuntimeOrigin::signed(owner),
				feed_id,
				to_disable.clone(),
				to_add.clone(),
			),
			Error::<Test>::OracleDisabled
		);
		assert_noop!(
			ChainlinkFeed::change_oracles(
				RuntimeOrigin::signed(owner),
				feed_id,
				vec![],
				to_add.clone(),
			),
			Error::<Test>::AlreadyEnabled
		);

		let feed = ChainlinkFeed::feed_config(feed_id).expect("feed should be there");
		assert_eq!(feed.oracle_count, 4);
		assert_eq!(Oracles::<Test>::iter().count(), 6);
		assert_eq!(OracleStatuses::<Test>::iter().count(), 6);
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
			assert!(
				ChainlinkFeed::oracle(o).is_some(),
				"oracle should be present"
			);
		}
		assert_ok!(ChainlinkFeed::change_oracles(
			RuntimeOrigin::signed(owner),
			feed_id,
			vec![],
			vec![(oracle, admin)],
		));
		let expected_status = OracleStatus {
			starting_round: 2,
			ending_round: None,
			last_reported_round: Some(1),
			last_started_round: Some(1),
			latest_submission: Some(submission),
		};
		assert_eq!(
			ChainlinkFeed::oracle_status(feed_id, oracle),
			Some(expected_status)
		);
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
			.oracles(oracles.clone())
			.build_and_store());
		let feed_id = 0;
		let feed = ChainlinkFeed::feed_config(feed_id).expect("feed should be there");
		assert_eq!(feed.payment, old_payment);

		let owner = 1;
		let new_payment = 30;
		let new_min = 3;
		let new_max = 3;
		let new_delay = 1;
		let new_timeout = 5;
		// failure cases
		assert_noop!(
			ChainlinkFeed::update_future_rounds(
				RuntimeOrigin::signed(owner),
				5,
				new_payment,
				(new_min, new_max),
				new_delay,
				new_timeout,
			),
			Error::<Test>::FeedNotFound
		);
		assert_noop!(
			ChainlinkFeed::update_future_rounds(
				RuntimeOrigin::signed(123),
				feed_id,
				new_payment,
				(new_min, new_max),
				new_delay,
				new_timeout,
			),
			Error::<Test>::NotFeedOwner
		);
		assert_noop!(
			ChainlinkFeed::update_future_rounds(
				RuntimeOrigin::signed(owner),
				feed_id,
				new_payment,
				(new_max + 1, new_max),
				new_delay,
				new_timeout,
			),
			Error::<Test>::WrongBounds
		);
		assert_noop!(
			ChainlinkFeed::update_future_rounds(
				RuntimeOrigin::signed(owner),
				feed_id,
				new_payment,
				(new_min, oracles.len() as u32 + 1),
				new_delay,
				new_timeout,
			),
			Error::<Test>::MaxExceededTotal
		);
		assert_noop!(
			ChainlinkFeed::update_future_rounds(
				RuntimeOrigin::signed(owner),
				feed_id,
				new_payment,
				(new_min, new_max),
				oracles.len() as RoundId,
				new_timeout,
			),
			Error::<Test>::DelayNotBelowCount
		);
		assert_noop!(
			ChainlinkFeed::update_future_rounds(
				RuntimeOrigin::signed(owner),
				feed_id,
				new_payment,
				(0, new_max),
				new_delay,
				new_timeout,
			),
			Error::<Test>::WrongBounds
		);

		// successful update
		tx_assert_ok!(ChainlinkFeed::update_future_rounds(
			RuntimeOrigin::signed(owner),
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
fn force_admin_transfer_should_work() {
	new_test_ext().execute_with(|| {
		let current_admin = ChainlinkFeed::pallet_admin();
		let new_admin = 42;

		assert_noop!(
			ChainlinkFeed::force_set_pallet_admin(RuntimeOrigin::signed(current_admin), new_admin),
			DispatchError::BadOrigin
		);
		assert_ok!(ChainlinkFeed::force_set_pallet_admin(
			RuntimeOrigin::root(),
			new_admin
		));
		assert_eq!(ChainlinkFeed::pallet_admin(), new_admin);
	});
}

#[test]
fn admin_transfer_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let oracle = 1;
		let old_admin = 2;
		assert_ok!(FeedBuilder::new()
			.min_submissions(1)
			.restart_delay(0)
			.oracles(vec![(oracle, old_admin)])
			.build_and_store());

		let new_admin = 42;
		assert_noop!(
			ChainlinkFeed::transfer_admin(RuntimeOrigin::signed(old_admin), 123, new_admin),
			Error::<Test>::OracleNotFound
		);
		assert_noop!(
			ChainlinkFeed::transfer_admin(RuntimeOrigin::signed(123), oracle, new_admin),
			Error::<Test>::NotAdmin
		);
		assert_noop!(
			ChainlinkFeed::cancel_admin_transfer(RuntimeOrigin::signed(old_admin), oracle,),
			Error::<Test>::NoPendingOwnershipTransfer
		);

		// transfer to self yields nothing
		assert_ok!(ChainlinkFeed::transfer_admin(
			RuntimeOrigin::signed(old_admin),
			oracle,
			old_admin
		));
		assert!(System::events().into_iter().all(|r| {
			!matches!(
				r.event,
				mock::RuntimeEvent::ChainlinkFeed(crate::Event::OracleAdminUpdateRequested(
					_,
					_,
					_
				))
			)
		}));

		assert_ok!(ChainlinkFeed::transfer_admin(
			RuntimeOrigin::signed(old_admin),
			oracle,
			new_admin
		));
		let oracle_meta = ChainlinkFeed::oracle(oracle).expect("oracle should be present");
		assert_eq!(oracle_meta.pending_admin, Some(new_admin));
		assert_noop!(
			ChainlinkFeed::accept_admin(RuntimeOrigin::signed(new_admin), 123,),
			Error::<Test>::OracleNotFound
		);
		assert_noop!(
			ChainlinkFeed::accept_admin(RuntimeOrigin::signed(123), oracle),
			Error::<Test>::NotPendingAdmin
		);
		assert_ok!(ChainlinkFeed::cancel_admin_transfer(
			RuntimeOrigin::signed(old_admin),
			oracle,
		),);
		assert_noop!(
			ChainlinkFeed::accept_admin(RuntimeOrigin::signed(new_admin), oracle),
			Error::<Test>::NotPendingAdmin
		);
		assert_ok!(ChainlinkFeed::transfer_admin(
			RuntimeOrigin::signed(old_admin),
			oracle,
			new_admin
		));

		assert_ok!(ChainlinkFeed::accept_admin(
			RuntimeOrigin::signed(new_admin),
			oracle
		));
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
		let snd_requester = 33;
		let delay = 4;
		assert_ok!(ChainlinkFeed::set_requester(
			RuntimeOrigin::signed(owner),
			feed_id,
			requester,
			delay
		));
		assert_ok!(ChainlinkFeed::set_requester(
			RuntimeOrigin::signed(owner),
			feed_id,
			snd_requester,
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
		// failure cases
		assert_noop!(
			ChainlinkFeed::request_new_round(RuntimeOrigin::signed(123), feed_id),
			Error::<Test>::NotAuthorizedRequester
		);
		// non existent feed is not present but also not authorized
		assert_noop!(
			ChainlinkFeed::request_new_round(RuntimeOrigin::signed(requester), 123),
			Error::<Test>::NotAuthorizedRequester
		);

		// actually request new round
		assert_ok!(ChainlinkFeed::request_new_round(
			RuntimeOrigin::signed(requester),
			feed_id
		));
		// need to wait `delay` rounds before requesting again
		assert_noop!(
			ChainlinkFeed::request_new_round(RuntimeOrigin::signed(requester), feed_id),
			Error::<Test>::CannotRequestRoundYet
		);
		// round does not have data and is not timed out
		// --> not supersedable
		assert_noop!(
			ChainlinkFeed::request_new_round(RuntimeOrigin::signed(snd_requester), feed_id),
			Error::<Test>::RoundNotSupersedable
		);

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
		let requester_meta =
			ChainlinkFeed::requester(feed_id, requester).expect("requester should be present");
		assert_eq!(
			requester_meta,
			Requester {
				delay,
				last_started_round: Some(1)
			}
		);
	});
}

#[test]
fn requester_permissions() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		assert_ok!(FeedBuilder::new().owner(owner).build_and_store());

		let feed_id = 0;
		let requester = 22;
		let delay = 4;
		// failure cases
		assert_noop!(
			ChainlinkFeed::set_requester(RuntimeOrigin::signed(owner), 123, requester, delay),
			Error::<Test>::FeedNotFound
		);
		assert_noop!(
			ChainlinkFeed::set_requester(RuntimeOrigin::signed(123), feed_id, requester, delay),
			Error::<Test>::NotFeedOwner
		);
		// actually set the requester
		assert_ok!(ChainlinkFeed::set_requester(
			RuntimeOrigin::signed(owner),
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
		// failure cases
		assert_noop!(
			ChainlinkFeed::remove_requester(RuntimeOrigin::signed(owner), 123, requester),
			Error::<Test>::FeedNotFound
		);
		assert_noop!(
			ChainlinkFeed::remove_requester(RuntimeOrigin::signed(123), feed_id, requester),
			Error::<Test>::NotFeedOwner
		);
		assert_noop!(
			ChainlinkFeed::remove_requester(RuntimeOrigin::signed(owner), feed_id, 123),
			Error::<Test>::RequesterNotFound
		);
		// actually remove the requester
		assert_ok!(ChainlinkFeed::remove_requester(
			RuntimeOrigin::signed(owner),
			feed_id,
			requester
		));
		assert_eq!(ChainlinkFeed::requester(feed_id, requester), None);
	});
}

#[test]
fn transfer_ownership_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let old_owner = 1;
		assert_ok!(FeedBuilder::new().owner(old_owner).build_and_store());

		let feed_id = 0;
		let new_owner = 42;
		assert_noop!(
			ChainlinkFeed::transfer_ownership(RuntimeOrigin::signed(old_owner), 5, new_owner),
			Error::<Test>::FeedNotFound
		);
		assert_noop!(
			ChainlinkFeed::transfer_ownership(RuntimeOrigin::signed(23), feed_id, new_owner),
			Error::<Test>::NotFeedOwner
		);
		assert_noop!(
			ChainlinkFeed::cancel_ownership_transfer(RuntimeOrigin::signed(old_owner), feed_id,),
			Error::<Test>::NoPendingOwnershipTransfer
		);
		// transfer to self yields nothing
		assert_ok!(ChainlinkFeed::transfer_ownership(
			RuntimeOrigin::signed(old_owner),
			feed_id,
			old_owner
		));
		assert!(System::events().into_iter().all(|r| {
			!matches!(
				r.event,
				mock::RuntimeEvent::ChainlinkFeed(crate::Event::OwnerUpdateRequested(_, _, _))
			)
		}));
		assert_ok!(ChainlinkFeed::transfer_ownership(
			RuntimeOrigin::signed(old_owner),
			feed_id,
			new_owner
		));

		let feed = ChainlinkFeed::feed_config(feed_id).expect("feed should be there");
		assert_eq!(feed.pending_owner, Some(new_owner));
		assert_noop!(
			ChainlinkFeed::accept_ownership(RuntimeOrigin::signed(new_owner), 123),
			Error::<Test>::FeedNotFound
		);

		// cancel transfer
		assert_ok!(ChainlinkFeed::cancel_ownership_transfer(
			RuntimeOrigin::signed(old_owner),
			feed_id,
		));
		assert_noop!(
			ChainlinkFeed::accept_ownership(RuntimeOrigin::signed(new_owner), feed_id),
			Error::<Test>::NotPendingOwner
		);

		// initiate again
		assert_ok!(ChainlinkFeed::transfer_ownership(
			RuntimeOrigin::signed(old_owner),
			feed_id,
			new_owner
		));
		assert_noop!(
			ChainlinkFeed::accept_ownership(RuntimeOrigin::signed(old_owner), feed_id),
			Error::<Test>::NotPendingOwner
		);
		assert_ok!(ChainlinkFeed::accept_ownership(
			RuntimeOrigin::signed(new_owner),
			feed_id
		));
		assert_noop!(
			ChainlinkFeed::cancel_ownership_transfer(RuntimeOrigin::signed(old_owner), feed_id,),
			Error::<Test>::NotFeedOwner
		);

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
			RuntimeOrigin::signed(oracle),
			feed_id,
			round_id,
			submission
		));
		assert_ok!(ChainlinkFeed::submit(
			RuntimeOrigin::signed(second_oracle),
			feed_id,
			round_id,
			submission
		));
		{
			let mut feed = ChainlinkFeed::feed(feed_id).expect("feed should be there");
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

			tx_assert_ok!(feed.request_new_round(AccountId::default()));
		}
		let round_id = 2;
		let round =
			ChainlinkFeed::round(feed_id, round_id).expect("second round should be present");
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
		let amount = ExistentialDeposit::get();
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
			ChainlinkFeed::withdraw_payment(RuntimeOrigin::signed(admin), 123, recipient, amount),
			Error::<Test>::OracleNotFound
		);
		assert_noop!(
			ChainlinkFeed::withdraw_payment(RuntimeOrigin::signed(123), oracle, recipient, amount),
			Error::<Test>::NotAdmin
		);
		assert_noop!(
			ChainlinkFeed::withdraw_payment(
				RuntimeOrigin::signed(admin),
				oracle,
				recipient,
				2 * amount
			),
			Error::<Test>::InsufficientFunds
		);
		let fund = FeedPalletId::get().into_account_truncating();
		let fund_balance = Balances::free_balance(&fund);
		Balances::make_free_balance_be(&fund, ExistentialDeposit::get());
		assert!(ChainlinkFeed::withdraw_payment(
			RuntimeOrigin::signed(admin),
			oracle,
			recipient,
			amount
		)
		.is_err());
		Balances::make_free_balance_be(&fund, fund_balance);

		// nothing reserved for the oracle yet
		assert_noop!(
			ChainlinkFeed::withdraw_payment(
				RuntimeOrigin::signed(admin),
				oracle,
				recipient,
				amount
			),
			Error::<Test>::InsufficientReserve
		);

		assert_ok!(Balances::reserve(&fund, fund_balance / 2));

		// nothing reserved for the oracle yet
		assert_ok!(ChainlinkFeed::withdraw_payment(
			RuntimeOrigin::signed(admin),
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
		let fund = FeedPalletId::get().into_account_truncating();
		assert_noop!(
			ChainlinkFeed::withdraw_funds(RuntimeOrigin::signed(123), recipient, amount),
			Error::<Test>::NotPalletAdmin
		);
		assert_noop!(
			ChainlinkFeed::withdraw_funds(
				RuntimeOrigin::signed(fund),
				recipient,
				101 * MIN_RESERVE
			),
			Error::<Test>::InsufficientFunds
		);
		assert_noop!(
			ChainlinkFeed::withdraw_funds(
				RuntimeOrigin::signed(fund),
				recipient,
				100 * MIN_RESERVE
			),
			Error::<Test>::InsufficientReserve
		);
		assert_ok!(ChainlinkFeed::withdraw_funds(
			RuntimeOrigin::signed(fund),
			recipient,
			amount
		));
	});
}

#[test]
fn transfer_pallet_admin_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let new_admin = 23;
		let fund = FeedPalletId::get().into_account_truncating();
		assert_noop!(
			ChainlinkFeed::transfer_pallet_admin(RuntimeOrigin::signed(123), new_admin),
			Error::<Test>::NotPalletAdmin
		);
		assert_noop!(
			ChainlinkFeed::cancel_pallet_admin_transfer(RuntimeOrigin::signed(fund)),
			Error::<Test>::NoPendingOwnershipTransfer
		);
		// transfer to self yields nothing
		assert_ok!(ChainlinkFeed::transfer_pallet_admin(
			RuntimeOrigin::signed(fund),
			fund
		));
		assert!(System::events().into_iter().all(|r| {
			!matches!(
				r.event,
				mock::RuntimeEvent::ChainlinkFeed(crate::Event::PalletAdminUpdateRequested(_, _))
			)
		}));
		assert_ok!(ChainlinkFeed::transfer_pallet_admin(
			RuntimeOrigin::signed(fund),
			new_admin
		));
		assert_eq!(PendingPalletAdmin::<Test>::get(), Some(new_admin));
		assert_noop!(
			ChainlinkFeed::accept_pallet_admin(RuntimeOrigin::signed(123)),
			Error::<Test>::NotPendingPalletAdmin
		);

		assert_ok!(ChainlinkFeed::cancel_pallet_admin_transfer(
			RuntimeOrigin::signed(fund)
		),);

		assert_noop!(
			ChainlinkFeed::accept_pallet_admin(RuntimeOrigin::signed(new_admin)),
			Error::<Test>::NotPendingPalletAdmin
		);

		assert_ok!(ChainlinkFeed::transfer_pallet_admin(
			RuntimeOrigin::signed(fund),
			new_admin
		));

		assert_ok!(ChainlinkFeed::accept_pallet_admin(RuntimeOrigin::signed(
			new_admin
		)));
		assert_eq!(PalletAdmin::<Test>::get(), new_admin);
		assert_eq!(PendingPalletAdmin::<Test>::get(), None);
	});
}

#[test]
fn auto_prune_should_work() {
	new_test_ext().execute_with(|| {
		let feed_id = 0;
		let oracle_a = 2;
		let oracle_b = 3;
		let oracle_admin = 4;
		let submission = 42;
		let owner = 1;

		assert_noop!(
			ChainlinkFeed::create_feed(
				RuntimeOrigin::signed(1),
				20,
				10,
				(10, 1_000),
				3,
				5,
				b"desc".to_vec(),
				2,
				vec![(1, 4), (2, 4), (3, 4)],
				Some(0),
				None,
			),
			Error::<Test>::CannotPruneRoundZero
		);

		let submit_a = |r| {
			assert_ok!(ChainlinkFeed::submit(
				RuntimeOrigin::signed(oracle_a),
				feed_id,
				r,
				submission
			));
		};
		let submit_a_and_b = |r| {
			submit_a(r);
			assert_ok!(ChainlinkFeed::submit(
				RuntimeOrigin::signed(oracle_b),
				feed_id,
				r,
				submission
			));
		};

		assert_ok!(FeedBuilder::new()
			.owner(owner)
			.timeout(1)
			.min_submissions(2)
			.restart_delay(0)
			.oracles(vec![(oracle_a, oracle_admin), (oracle_b, oracle_admin)])
			.pruning_window(2)
			.build_and_store());

		System::set_block_number(1);
		submit_a_and_b(1);
		System::set_block_number(2);
		submit_a_and_b(2);

		assert!(ChainlinkFeed::round(feed_id, 0).is_some());
		assert!(ChainlinkFeed::round(feed_id, 1).is_some());
		assert!(ChainlinkFeed::round(feed_id, 2).is_some());

		let feed = ChainlinkFeed::feed(feed_id).expect("feed should be there");
		assert_eq!(feed.first_valid_round(), Some(1));

		System::set_block_number(3);
		submit_a_and_b(3);
		assert!(ChainlinkFeed::round(feed_id, 0).is_some());
		// only oldest round is pruned
		assert!(ChainlinkFeed::round(feed_id, 1).is_none());
		assert!(ChainlinkFeed::round(feed_id, 2).is_some());
		assert!(ChainlinkFeed::round(feed_id, 3).is_some());

		let feed = ChainlinkFeed::feed(feed_id).expect("feed should be there");
		assert_eq!(feed.first_valid_round(), Some(2));

		System::set_block_number(4);
		submit_a_and_b(4);
		assert!(ChainlinkFeed::round(feed_id, 2).is_none());
		let feed = ChainlinkFeed::feed(feed_id).expect("feed should be there");
		assert_eq!(feed.first_valid_round(), Some(3));

		// shrink pruning window
		assert_ok!(ChainlinkFeed::set_pruning_window(
			RuntimeOrigin::signed(owner),
			feed_id,
			1
		));
		let feed = ChainlinkFeed::feed(feed_id).expect("feed should be there");
		assert_eq!(feed.first_valid_round(), Some(4));
		assert!(ChainlinkFeed::round(feed_id, 3).is_none());
	});
}

#[test]
fn feed_creation_permissioning() {
	new_test_ext().execute_with(|| {
		let admin = FeedPalletId::get().into_account_truncating();
		let new_creator = 15;
		assert_noop!(
			FeedBuilder::new().owner(new_creator).build_and_store(),
			Error::<Test>::NotFeedCreator
		);
		assert_noop!(
			ChainlinkFeed::set_feed_creator(RuntimeOrigin::signed(123), new_creator),
			Error::<Test>::NotPalletAdmin
		);
		assert_ok!(ChainlinkFeed::set_feed_creator(
			RuntimeOrigin::signed(admin),
			new_creator
		));
		assert_ok!(FeedBuilder::new().owner(new_creator).build_and_store());
		assert_noop!(
			ChainlinkFeed::remove_feed_creator(RuntimeOrigin::signed(123), new_creator),
			Error::<Test>::NotPalletAdmin
		);
		assert_ok!(ChainlinkFeed::remove_feed_creator(
			RuntimeOrigin::signed(admin),
			new_creator
		));
		assert_noop!(
			FeedBuilder::new().owner(new_creator).build_and_store(),
			Error::<Test>::NotFeedCreator
		);
	});
}

#[test]
fn can_go_into_debt_and_repay() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let admin: AccountId = FeedPalletId::get().into_account_truncating();
		let owner = 1;
		let oracle = 2;
		let payment = 33;
		assert_ok!(FeedBuilder::new()
			.payment(payment)
			.owner(owner)
			.oracles(vec![(oracle, 3), (3, 3)])
			.max_debt(42)
			.build_and_store());
		assert_eq!(ChainlinkFeed::debt(0).unwrap(), 0);
		// ensure the fund is out of tokens
		Balances::make_free_balance_be(&admin, ExistentialDeposit::get());
		assert_ok!(ChainlinkFeed::submit(
			RuntimeOrigin::signed(oracle),
			0,
			1,
			42
		));
		assert_eq!(ChainlinkFeed::debt(0).unwrap(), payment);
		let new_funds = 2 * payment;
		Balances::make_free_balance_be(&admin, new_funds);

		// reducing debt should fail for non admin
		assert_noop!(
			ChainlinkFeed::reduce_debt(RuntimeOrigin::signed(1), 0, 10),
			Error::<Test>::NotPalletAdmin
		);

		// should be possible to reduce debt partially
		assert_ok!(ChainlinkFeed::reduce_debt(
			RuntimeOrigin::signed(admin),
			0,
			10
		));
		assert_eq!(Balances::free_balance(admin), new_funds - 10);
		assert_eq!(ChainlinkFeed::debt(0).unwrap(), payment - 10);
		// should be possible to overshoot in passing the amount correcting debt...
		assert_ok!(ChainlinkFeed::reduce_debt(
			RuntimeOrigin::signed(admin),
			0,
			payment
		));
		// ... but will only correct the debt
		assert_eq!(Balances::free_balance(admin), new_funds - payment);
		assert_eq!(ChainlinkFeed::debt(0).unwrap(), 0);
	});
}

#[test]
fn feed_life_cylce() {
	new_test_ext().execute_with(|| {
		let id = 0;
		let owner = 1;
		let payment = 33;
		let timeout = 10;
		let submission_value_bounds = (1, 1_000);
		let submission_count_bounds = (1, 3);
		let decimals = 5;
		let description = b"desc".to_vec().try_into().unwrap();
		let restart_delay = 1;
		let new_config = FeedConfig {
			owner,
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
			next_round_to_prune: One::one(),
			pruning_window: RoundId::MAX,
			debt: Zero::zero(),
			max_debt: None,
		};
		let oracles = vec![(2, 2), (3, 3), (4, 4)];
		{
			let mut feed = Feed::<Test>::new(id, new_config.clone());
			tx_assert_ok!(feed.add_oracles(oracles.clone()));
			tx_assert_ok!(feed.update_future_rounds(
				payment,
				submission_count_bounds,
				restart_delay,
				timeout
			));
		}
		let new_config = FeedConfig {
			oracle_count: oracles.len() as u32,
			..new_config
		};
		// config should be stored on drop
		assert_eq!(ChainlinkFeed::feed_config(id), Some(new_config.clone()));
		let new_timeout = 5;
		{
			let mut feed = Feed::<Test>::load_from(id).expect("feed should be there");
			feed.config.timeout = new_timeout;
		}
		let modified_config = FeedConfig {
			timeout: new_timeout,
			..new_config
		};
		// modified config should be stored on drop
		assert_eq!(
			ChainlinkFeed::feed_config(id),
			Some(modified_config.clone())
		);
		let ignored_timeout = 23;
		{
			let mut feed = Feed::<Test>::read_only_from(id).expect("feed should be there");
			feed.config.timeout = ignored_timeout;
		}
		// read only access should not store changes
		assert_eq!(ChainlinkFeed::feed_config(id), Some(modified_config));
		{
			let mut feed = ChainlinkFeed::feed_mut(id).expect("feed should be there");
			tx_assert_ok!(feed.request_new_round(AccountId::default()));
		}
		assert_eq!(ChainlinkFeed::feed_config(id).unwrap().reporting_round, 1);
	});
}

#[test]
fn allows_submissions_until_max_debt() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let admin: AccountId = FeedPalletId::get().into_account_truncating();
		let owner = 1;
		let payment = 10;
		let oracle_admin = 4;
		// this allows max 3 payments
		let max_debt = 3 * payment;
		let oracles = vec![
			(1, oracle_admin),
			(2, oracle_admin),
			(3, oracle_admin),
			(5, oracle_admin),
		];
		// create two feeds
		assert_ok!(FeedBuilder::new()
			.payment(payment)
			.owner(owner)
			.oracles(oracles.clone())
			.max_debt(max_debt)
			.build_and_store());
		assert_ok!(FeedBuilder::new()
			.payment(payment)
			.owner(owner)
			.oracles(oracles)
			.max_debt(max_debt)
			.build_and_store());
		assert_eq!(ChainlinkFeed::debt(0).unwrap(), 0);
		assert_eq!(ChainlinkFeed::debt(1).unwrap(), 0);
		// ensure the fund is out of tokens
		Balances::make_free_balance_be(&admin, ExistentialDeposit::get());
		assert_ok!(ChainlinkFeed::submit(RuntimeOrigin::signed(1), 0, 1, 42));
		assert_ok!(ChainlinkFeed::submit(RuntimeOrigin::signed(2), 0, 1, 42));
		assert_ok!(ChainlinkFeed::submit(RuntimeOrigin::signed(3), 0, 1, 42));

		assert_ok!(ChainlinkFeed::submit(RuntimeOrigin::signed(1), 1, 1, 42));
		assert_ok!(ChainlinkFeed::submit(RuntimeOrigin::signed(2), 1, 1, 42));
		assert_ok!(ChainlinkFeed::submit(RuntimeOrigin::signed(3), 1, 1, 42));

		assert_noop!(
			ChainlinkFeed::submit(RuntimeOrigin::signed(5), 0, 1, 42),
			Error::<Test>::MaxDebtReached
		);
		assert_noop!(
			ChainlinkFeed::submit(RuntimeOrigin::signed(5), 1, 1, 42),
			Error::<Test>::MaxDebtReached
		);
		assert_eq!(ChainlinkFeed::debt(0).unwrap(), max_debt);
		assert_eq!(ChainlinkFeed::debt(1).unwrap(), max_debt);

		// reduce debt withdraw and submit again
		Balances::make_free_balance_be(&admin, max_debt + ExistentialDeposit::get());
		assert_ok!(ChainlinkFeed::reduce_debt(
			RuntimeOrigin::signed(admin),
			0,
			max_debt
		));
		// now max_debt amount is reserved and can be withdrawn
		assert_eq!(Balances::reserved_balance(&admin), max_debt);

		// try to withdraw more than the oracle earned so far
		assert_noop!(
			ChainlinkFeed::withdraw_payment(RuntimeOrigin::signed(oracle_admin), 1, 1, payment * 3),
			Error::<Test>::InsufficientFunds
		);

		// withdraw the payment
		assert_ok!(ChainlinkFeed::withdraw_payment(
			RuntimeOrigin::signed(oracle_admin),
			1,
			1,
			payment
		));
		// oracle has payment in their account
		assert_eq!(Balances::total_balance(&1), payment);
		assert_eq!(Balances::reserved_balance(&admin), max_debt - payment);

		// can accumulate debt again, with different submission orders
		assert_ok!(ChainlinkFeed::submit(RuntimeOrigin::signed(3), 0, 2, 42));
		assert_ok!(ChainlinkFeed::submit(RuntimeOrigin::signed(1), 0, 2, 42));
		assert_ok!(ChainlinkFeed::submit(RuntimeOrigin::signed(2), 0, 2, 42));

		assert_noop!(
			ChainlinkFeed::submit(RuntimeOrigin::signed(5), 0, 2, 42),
			Error::<Test>::MaxDebtReached
		);
		assert_eq!(ChainlinkFeed::debt(0).unwrap(), max_debt);

		// can withdraw two more payments until fund is out of funds because current
		// debt was not paid yet
		assert_ok!(ChainlinkFeed::withdraw_payment(
			RuntimeOrigin::signed(oracle_admin),
			2,
			2,
			payment
		));
		assert_ok!(ChainlinkFeed::withdraw_payment(
			RuntimeOrigin::signed(oracle_admin),
			3,
			3,
			payment
		));
		assert_noop!(
			ChainlinkFeed::withdraw_payment(RuntimeOrigin::signed(oracle_admin), 1, 1, payment),
			Error::<Test>::InsufficientReserve
		);
	});
}

#[test]
fn can_create_feed_in_genesis() {
	let oracles = vec![(1, 4), (2, 4), (3, 4)];
	let builder = FeedBuilder::new()
		.owner(1)
		.decimals(8)
		.payment(20)
		.restart_delay(2)
		.timeout(10)
		.description(b"feed".to_vec())
		.value_bounds(1, 1_000)
		.min_submissions(2)
		.oracles(oracles.clone());

	new_test_ext_with_feeds(vec![builder.clone()]).execute_with(|| {
		let mut config = builder.build().unwrap();
		// need to adjust the oracle count as this gets set in genesis
		config.oracle_count = oracles.len() as u32;
		let feed = ChainlinkFeed::feed_config(&0).unwrap();
		assert_eq!(feed, config);
	});
}
