use codec::{Decode, Encode};
use frame_support::sp_runtime::traits::UniqueSaturatedFrom;
use frame_support::traits::Currency;
use frame_support::{decl_module, decl_storage, dispatch::DispatchResult};
use frame_system::ensure_root;
use pallet_chainlink::{CallbackWithParameter, Config as ChainlinkTrait, Event};
use sp_std::prelude::*;

type BalanceOf<T> = <<T as pallet_chainlink::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

pub trait Config: pallet_chainlink::Config + ChainlinkTrait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type Callback: From<Call<Self>> + Into<<Self as ChainlinkTrait>::Callback>;
}

decl_storage! {
	trait Store for Module<T: Config> as ExampleStorage {
		pub Result: i128;
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		#[weight = 0]
		pub fn send_request(origin, operator: T::AccountId, specid: Vec<u8>) -> DispatchResult {
			let parameters = ("get", "https://min-api.cryptocompare.com/data/pricemultifull?fsyms=ETH&tsyms=USD", "path", "RAW.ETH.USD.PRICE", "times", "100000000");
			let call: <T as Config>::Callback = Call::callback(vec![]).into();

			let fee = BalanceOf::<T>::unique_saturated_from(100u32);
			<pallet_chainlink::Pallet<T>>::initiate_request(origin, operator, specid, 0, parameters.encode(), fee, call.into())?;

			Ok(())
		}

		#[weight = 0]
		pub fn callback(origin, result: Vec<u8>) -> DispatchResult {
			ensure_root(origin)?;

			// The result is expected to be a SCALE encoded `i128`
			let r : i128 = i128::decode(&mut &result[..]).map_err(|_| Error::<T>::DecodingFailed)?;
			<Result>::put(r);

			Ok(())
		}
	}
}

frame_support::decl_error! {
	pub enum Error for Module<T: Config> {
		DecodingFailed
	}
}

impl<T: Config> CallbackWithParameter for Call<T> {
	fn with_result(&self, result: Vec<u8>) -> Option<Self> {
		match *self {
			Call::callback(_) => Some(Call::callback(result)),
			_ => None,
		}
	}
}
