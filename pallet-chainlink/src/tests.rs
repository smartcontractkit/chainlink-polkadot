#![cfg(test)]

use super::*;
use frame_support::{
	impl_outer_origin, impl_outer_event, parameter_types, weights::Weight,
	traits::{OnFinalize}
};
use sp_core::H256;
use sp_runtime::{
	Perbill,
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use codec::{Decode, Encode};
use crate::sp_api_hidden_includes_decl_storage::hidden_include::StorageValue;
use frame_system as system;

impl_outer_origin! {
	pub enum Origin for Test where system = frame_system {}
}

pub mod chainlink {
	// Re-export needed for `impl_outer_event!`.
	pub use super::super::*;
}

impl_outer_event! {
	pub enum TestEvent for Test {
		system<T>,
		pallet_balances<T>,
		chainlink<T>,
	}
}


#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}
impl system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Call = ();
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u128; // u64 is not enough to hold bytes used to generate bounty account
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type AvailableBlockRatio = AvailableBlockRatio;
	type MaximumBlockLength = MaximumBlockLength;
	type Version = ();
	type PalletInfo = ();
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}
parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}
impl pallet_balances::Trait for Test {
	type MaxLocks = ();
	type Balance = u64;
	type Event = TestEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}
impl chainlink::Trait for Test {
	type Event = TestEvent;
	type Currency = pallet_balances::Module<Test>;
	type Callback = module2::Call<Test>;
	type ValidityPeriod = ValidityPeriod;
}
impl module2::Trait for Test {
}
parameter_types! {
	pub const ValidityPeriod: u64 = 10;
}

type System = frame_system::Module<Test>;
type Chainlink = chainlink::Module<Test>;

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test>{
		// Total issuance will be 200 with treasury account initialized at ED.
		balances: vec![(0, 100000), (1, 100000), (2, 100000)],
	}.assimilate_storage(&mut t).unwrap();
	t.into()
}

pub fn last_event() -> RawEvent<u128, u64> {
	System::events().into_iter().map(|r| r.event)
		.filter_map(|e| {
			if let TestEvent::chainlink(inner) = e { Some(inner) } else { None }
		})
		.last()
		.unwrap()
}


pub mod module2 {
	use super::*;

	pub trait Trait: frame_system::Trait {}

	frame_support::decl_module! {
		pub struct Module<T: Trait> for enum Call
			where origin: <T as frame_system::Trait>::Origin
		{
			#[weight = 0]
			pub fn callback(_origin, result: Vec<u8>) -> frame_support::dispatch::DispatchResult {
				let r : u128 = u128::decode(&mut &result[..]).map_err(|err| err.what())?;
				<Result>::put(r);
				Ok(())
			}
		}
	}

	frame_support::decl_storage! {
		trait Store for Module<T: Trait> as TestStorage {
			pub Result: u128;
		}
	}

	impl <T: Trait> CallbackWithParameter for Call<T> {
		fn with_result(&self, result: Vec<u8>) -> Option<Self> {
			match *self {
				Call::callback(_) => Some(Call::callback(result)),
				_ => None
			}
		}
	}

}

#[test]
fn operators_can_be_registered() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert!(!<Chainlink>::operator(1));
		assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
		assert_eq!(last_event(),RawEvent::OperatorRegistered(1));
		assert!(<Chainlink>::operator(1));
		assert!(<Chainlink>::unregister_operator(Origin::signed(1)).is_ok());
		assert!(!<Chainlink>::operator(1));
		assert_eq!(last_event(),RawEvent::OperatorUnregistered(1));
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
		assert!(<Chainlink>::initiate_request(Origin::signed(2), 1, vec![], 1, vec![], 0, module2::Call::<Test>::callback(vec![]).into()).is_err());
	});

	new_test_ext().execute_with(|| {
		assert!(<Chainlink>::initiate_request(Origin::signed(2), 1, vec![], 1, vec![], 1, module2::Call::<Test>::callback(vec![]).into()).is_err());
	});

	new_test_ext().execute_with(|| {
		assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
		assert!(<Chainlink>::initiate_request(Origin::signed(2), 1, vec![], 1, vec![], 2, module2::Call::<Test>::callback(vec![]).into()).is_ok());
		assert!(<Chainlink>::callback(Origin::signed(3), 0, 10.encode()).is_err());
	});

	new_test_ext().execute_with(|| {
		assert!(<Chainlink>::callback(Origin::signed(1), 0, 10.encode()).is_err());
	});

	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
		assert_eq!(last_event(),RawEvent::OperatorRegistered(1));

		let parameters = ("a", "b");
		let data = parameters.encode();
		assert!(<Chainlink>::initiate_request(Origin::signed(2), 1, vec![], 1, data.clone(), 2, module2::Call::<Test>::callback(vec![]).into()).is_ok());
		assert_eq!(last_event(),RawEvent::OracleRequest(1, vec![], 0, 2, 1, data.clone(), "Chainlink.callback".into(), 2));

		let r = <(Vec<u8>, Vec<u8>)>::decode(&mut &data[..]).unwrap().0;
		assert_eq!("a", std::str::from_utf8(&r).unwrap());

		let result = 10;
		assert!(<Chainlink>::callback(Origin::signed(1), 0, result.encode()).is_ok());
		assert_eq!(module2::Result::get(), result);
	});

}

#[test]
pub fn on_finalize() {

	new_test_ext().execute_with(|| {
		assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
		assert!(<Chainlink>::initiate_request(Origin::signed(2), 1, vec![], 1, vec![], 2, module2::Call::<Test>::callback(vec![]).into()).is_ok());
		<Chainlink as OnFinalize<u64>>::on_finalize(20);
		// Request has been killed, too old
		assert!(<Chainlink>::callback(Origin::signed(1), 0, 10.encode()).is_err());
	});

}

