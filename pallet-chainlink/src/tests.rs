use codec::{Decode, Encode};
use frame_support::{
	parameter_types,
	traits::{ConstU16, ConstU32, ConstU64, OnFinalize},
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use sp_std::convert::{TryFrom, TryInto};

use crate as pallet_chainlink;

use super::*;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Chainlink: pallet_chainlink::{Pallet, Call, Storage, Event<T>},
	  Module2: module2::{Pallet, Call, Storage},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

pub(crate) type TestId = [u8; 8];

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type Balance = u64;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ConstU32<2>;
	type ReserveIdentifier = TestId;
	type WeightInfo = ();
	type HoldIdentifier = TestId;
	type FreezeIdentifier = TestId;
	type MaxFreezes = ConstU32<2>;
	type MaxHolds = ConstU32<2>;
}

impl pallet_chainlink::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = pallet_balances::Pallet<Test>;
	type Callback = module2::Call<Test>;
	type ValidityPeriod = ValidityPeriod;
}

impl module2::Config for Test {}

parameter_types! {
	pub const ValidityPeriod: u64 = 10;
}

// type Chainlink = pallet_chainlink::Pallet<Test>;

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();
	pallet_balances::GenesisConfig::<Test> {
		// Total issuance will be 200 with treasury account initialized at ED.
		balances: vec![(0, 100000), (1, 100000), (2, 100000)],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
}

pub fn last_event() -> Event<Test> {
	System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(|e| {
			if let RuntimeEvent::Chainlink(inner) = e {
				Some(inner)
			} else {
				None
			}
		})
		.last()
		.unwrap()
}

pub mod module2 {
	pub use pallet::*;

	#[frame_support::pallet]
	#[allow(unused)]
	pub mod pallet {
		use crate::CallbackWithParameter;
		use frame_support::pallet_prelude::*;
		use frame_system::pallet_prelude::*;
		use sp_std::{convert::TryInto, prelude::*};

		#[pallet::pallet]
		#[pallet::without_storage_info]
		pub struct Pallet<T>(_);

		#[pallet::config]
		pub trait Config: frame_system::Config {}

		#[pallet::call]
		impl<T: Config> Pallet<T> {
			#[pallet::weight(0)]
			#[pallet::call_index(0)]
			pub fn callback(_: OriginFor<T>, result: Vec<u8>) -> DispatchResult {
				let r: u128 =
					u128::decode(&mut &result[..]).map_err(|_| Error::<T>::DecodingFailed)?;
				<Result<T>>::put(r);
				Ok(())
			}
		}

		#[pallet::storage]
		#[pallet::getter(fn result)]
		pub type Result<T> = StorageValue<_, u128, ValueQuery>;

		#[pallet::error]
		pub enum Error<T> {
			DecodingFailed,
		}

		impl<T: Config> CallbackWithParameter for Call<T> {
			fn with_result(&self, result: Vec<u8>) -> Option<Self> {
				match *self {
					Call::callback { .. } => Some(Call::callback { result }),
					_ => None,
				}
			}
		}
	}
}

type Origin = RuntimeOrigin;

#[test]
fn operators_can_be_registered() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert!(<Chainlink>::operator(1).is_none());
		assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
		assert_eq!(last_event(), Event::OperatorRegistered(1));
		assert!(<Chainlink>::operator(1).is_some());
		assert!(<Chainlink>::unregister_operator(Origin::signed(1)).is_ok());
		assert!(<Chainlink>::operator(1).is_none());
		assert_eq!(last_event(), Event::OperatorUnregistered(1));
	});

	new_test_ext().execute_with(|| {
		assert!(<Chainlink>::unregister_operator(Origin::signed(1)).is_err());
		assert!(<Chainlink>::operator(1).is_none());
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
			module2::Call::<Test>::callback { result: vec![] }.into()
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
			module2::Call::<Test>::callback { result: vec![] }.into()
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
			module2::Call::<Test>::callback { result: vec![] }.into()
		)
		.is_ok());
		assert!(<Chainlink>::callback(Origin::signed(3), 0, 10.encode()).is_err());
	});

	new_test_ext().execute_with(|| {
		assert!(<Chainlink>::callback(Origin::signed(1), 0, 10.encode()).is_err());
	});

	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
		assert_eq!(last_event(), Event::OperatorRegistered(1));

		let parameters = ("a", "b");
		let data = parameters.encode();
		assert!(<Chainlink>::initiate_request(
			Origin::signed(2),
			1,
			vec![],
			1,
			data.clone(),
			2,
			module2::Call::<Test>::callback { result: vec![] }.into()
		)
		.is_ok());
		assert_eq!(
			last_event(),
			Event::OracleRequest(
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

		let result = 10;
		assert!(<Chainlink>::callback(Origin::signed(1), 0, result.encode()).is_ok());
		assert_eq!(module2::Result::<Test>::get(), result);
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
			module2::Call::<Test>::callback { result: vec![] }.into()
		)
		.is_ok());
		<Chainlink as OnFinalize<u64>>::on_finalize(20);
		// Request has been killed, too old
		assert!(<Chainlink>::callback(Origin::signed(1), 0, 10.encode()).is_err());
	});
}
