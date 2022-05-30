use frame_support::{
	pallet_prelude::ConstU32,
	parameter_types,
	sp_runtime::{
		testing::Header,
		traits::{BlakeTwo256, IdentityLookup},
	},
	sp_std::convert::{TryFrom, TryInto},
};
use frame_system as system;
use sp_core::H256;

use crate as pallet_chainlink;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	  pub enum Test where
			  Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	  {
			  System: frame_system::{Pallet, Call, Config, Storage, Event<T>} = 0,
			  Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 1,
			  Chainlink: pallet_chainlink::{Pallet, Call, Storage, Event<T>} = 2,
			  TestModule: test_module,
	  }
);

parameter_types! {
	  pub const BlockHashCount: u64 = 250;
	  pub const SS58Prefix: u8 = 42;
}

pub(crate) type AccountId = u128;
pub(crate) type BlockNumber = u64;

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

parameter_types! {
	  pub const ExistentialDeposit: u64 = 1;
}

type Balance = u64;

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

impl pallet_chainlink::Config for Test {
	type Event = Event;
	type Currency = pallet_balances::Pallet<Test>;
	type Callback = test_module::Call<Test>;
	type ValidityPeriod = ValidityPeriod;
}

impl test_module::Config for Test {
	// type Event = Event;
}

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

pub fn last_event() -> crate::Event<Test> {
	System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(|e| {
			if let Event::Chainlink(inner) = e {
				Some(inner)
			} else {
				None
			}
		})
		.last()
		.unwrap()
}

// test pallets
pub mod test_module {
	use crate::CallbackWithParameter;
	use codec::Decode;
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::{StorageValue, ValueQuery},
		sp_std::convert::TryInto,
	};
	use frame_system::pallet_prelude::OriginFor;

	pub use pallet::*;

	#[frame_support::pallet]
	pub mod pallet {
		use super::*;

		#[pallet::config]
		pub trait Config: frame_system::Config {}

		#[pallet::pallet]
		#[pallet::generate_store(pub (super) trait Store)]
		#[pallet::without_storage_info]
		pub struct Pallet<T>(_);

		#[pallet::call]
		impl<T: Config> Pallet<T> {
			#[pallet::weight(0)]
			pub fn callback(_origin: OriginFor<T>, result: Vec<u8>) -> DispatchResult {
				let r: u128 =
					u128::decode(&mut &result[..]).map_err(|_| Error::<T>::DecodingFailed)?;
				Result::<T>::put(r);
				Ok(())
			}
		}

		#[pallet::storage]
		#[pallet::getter(fn operator)]
		pub type Result<T: Config> = StorageValue<_, u128, ValueQuery>;

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
