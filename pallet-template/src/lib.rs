#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
#[allow(unused)]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::sp_runtime::traits::UniqueSaturatedFrom;
	use frame_system::pallet_prelude::*;
	use pallet_chainlink::{BalanceOf, CallbackWithParameter};
	use sp_std::{convert::TryInto, prelude::*};

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_chainlink::Config {
		type Callback: From<Call<Self>> + Into<<Self as pallet_chainlink::Config>::Callback>;
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		#[pallet::call_index(0)]
		pub fn send_request(
			origin: OriginFor<T>,
			operator: <T as frame_system::Config>::AccountId,
			specid: Vec<u8>,
		) -> DispatchResult {
			let parameters = (
				"get",
				"https://min-api.cryptocompare.com/data/pricemultifull?fsyms=ETH&tsyms=USD",
				"path",
				"RAW.ETH.USD.PRICE",
				"times",
				"100000000",
			);
			let call: <T as Config>::Callback = Call::callback { result: vec![] }.into();

			let fee = BalanceOf::<T>::unique_saturated_from(100u32);
			<pallet_chainlink::Pallet<T>>::initiate_request(
				origin,
				operator,
				specid,
				0,
				parameters.encode(),
				fee,
				call.into(),
			)?;

			Ok(())
		}

		#[pallet::weight(0)]
		#[pallet::call_index(1)]
		pub fn callback(origin: OriginFor<T>, result: Vec<u8>) -> DispatchResult {
			ensure_root(origin)?;
			let r: u128 = u128::decode(&mut &result[..]).map_err(|_| Error::<T>::DecodingFailed)?;
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
