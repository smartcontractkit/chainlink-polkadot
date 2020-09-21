#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_error, dispatch, traits::Get};
use frame_system::{ensure_signed, ensure_root, Trait as SystemTrait, Origin};
use pallet_chainlink::{CallbackWithParameter, Event, Trait as ChainlinkTrait, BalanceOf};
use codec::{Decode, Encode};
use sp_std::prelude::*;
use sp_runtime::traits::{Dispatchable, DispatchInfoOf as Info};
use sp_runtime::DispatchResultWithInfo;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: ChainlinkTrait {
    type Event: From<Event<Self>> + Into<<Self as SystemTrait>::Event>;
	type Callback: From<Call<Self>> + Into<<Self as ChainlinkTrait>::Callback>;

}

decl_storage! {
	trait Store for Module<T: Trait> as ChainLinkExampleStorage {
		// the result of the oracle call
		pub Result get(fn get_result): i128;
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn send_request(origin, operator: T::AccountId, specid: Vec<u8>) -> dispatch::DispatchResult {
			ensure_signed(origin.clone())?;

			let parameters = ("get", "https://min-api.cryptocompare.com/data/pricemultifull?fsyms=ETH&tsyms=USD", "path", "RAW.ETH.USD.PRICE", "times", "100000000");
			// Update storage.
			let call: <T as Trait>::Callback = Call::callback(vec![]).into();
            <pallet_chainlink::Module<T>>::initiate_request(origin, operator, specid, 0, parameters.encode(), BalanceOf::<T>::from(100), call.into())?;

			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn callback(origin, result: Vec<u8>) -> dispatch::DispatchResult {
			ensure_root(origin)?;

            let r : i128 = i128::decode(&mut &result[..]).map_err(|err| err.what())?;
            <Result>::put(r);

            Ok(())
		}
	}
}

impl <T:Trait> Dispatchable for Call<T> {
    type Origin = T::Origin;
    type Trait = ();
    type Info = ();
    type PostInfo = ();

    fn dispatch(self, _origin: Self::Origin) -> DispatchResultWithInfo<Self::PostInfo> {
        Ok(())
        // unimplemented!()
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
