use frame_support::{decl_module, decl_storage, dispatch::DispatchResult};
use system::ensure_signed;
use chainlink::{Event, create_request_event_from_parameters};
use sp_std::prelude::*;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as ExampleStorage {
        // Declare storage and getter functions here
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		pub fn send_request(origin) -> DispatchResult {
            let who : <T as system::Trait>::AccountId = ensure_signed(origin)?;
			Self::deposit_event(create_request_event_from_parameters::<T, (&[u8], &[u8], &[u8], &[u8], &[u8], &[u8])>(1, 0, who, 0, (b"get", b"https://min-api.cryptocompare.com/data/pricemultifull?fsyms=ETH&tsyms=USD", b"path", b"RAW.ETH.USD.CHANGEPCTDAY", b"times", b"2")));
			Ok(())
		}
    }
}
