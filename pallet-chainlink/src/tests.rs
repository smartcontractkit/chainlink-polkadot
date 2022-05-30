use crate::mock::*;
use codec::{Decode, Encode};
use frame_support::traits::OnFinalize;

#[test]
fn operators_can_be_registered() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert!(!<Chainlink>::operator(1));
		assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
		assert_eq!(last_event(), crate::Event::OperatorRegistered(1));
		assert!(<Chainlink>::operator(1));
		assert!(<Chainlink>::unregister_operator(Origin::signed(1)).is_ok());
		assert!(!<Chainlink>::operator(1));
		assert_eq!(last_event(), crate::Event::OperatorUnregistered(1));
	});

	new_test_ext().execute_with(|| {
		assert!(<Chainlink>::unregister_operator(Origin::signed(1)).is_err());
		assert!(!<Chainlink>::operator(1));
	});
}

#[test]
fn initiate_requests() {
	new_test_ext().execute_with(|| {
		assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
		assert!(<Chainlink>::initiate_request(
			Origin::signed(2),
			1,
			vec![],
			1,
			vec![],
			0,
			test_module::Call::<Test>::callback { result: vec![] }.into()
		)
		.is_err());
	});

	new_test_ext().execute_with(|| {
		assert!(<Chainlink>::initiate_request(
			Origin::signed(2),
			1,
			vec![],
			1,
			vec![],
			1,
			test_module::Call::<Test>::callback { result: vec![] }.into()
		)
		.is_err());
	});

	new_test_ext().execute_with(|| {
		assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
		assert!(<Chainlink>::initiate_request(
			Origin::signed(2),
			1,
			vec![],
			1,
			vec![],
			2,
			test_module::Call::<Test>::callback { result: vec![] }.into()
		)
		.is_ok());
		assert!(<Chainlink>::callback(Origin::signed(3), 0, 10u128.encode()).is_err());
	});

	new_test_ext().execute_with(|| {
		assert!(<Chainlink>::callback(Origin::signed(1), 0, 10u128.encode()).is_err());
	});

	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
		assert_eq!(last_event(), crate::Event::OperatorRegistered(1));

		let parameters = ("a", "b");
		let data = parameters.encode();
		assert!(<Chainlink>::initiate_request(
			Origin::signed(2),
			1,
			vec![],
			1,
			data.clone(),
			2,
			test_module::Call::<Test>::callback { result: vec![] }.into()
		)
		.is_ok());
		assert_eq!(
			last_event(),
			crate::Event::OracleRequest(
				1,
				vec![],
				0,
				2,
				1,
				data.clone(),
				"Chainlink.callback".into(),
				2
			)
		);

		let r = <(Vec<u8>, Vec<u8>)>::decode(&mut &data[..]).unwrap().0;
		assert_eq!("a", std::str::from_utf8(&r).unwrap());

		let result = 10u128;
		assert!(<Chainlink>::callback(Origin::signed(1), 0, result.encode()).is_ok());
		assert_eq!(test_module::Result::<Test>::get(), result);
	});
}

#[test]
pub fn on_finalize() {
	new_test_ext().execute_with(|| {
		assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
		assert!(<Chainlink>::initiate_request(
			Origin::signed(2),
			1,
			vec![],
			1,
			vec![],
			2,
			test_module::Call::<Test>::callback { result: vec![] }.into()
		)
		.is_ok());
		<Chainlink as OnFinalize<u64>>::on_finalize(20);
		// Request has been killed, too old
		assert!(<Chainlink>::callback(Origin::signed(1), 0, 10u128.encode()).is_err());
	});
}
