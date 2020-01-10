use frame_support::{decl_module, decl_storage, dispatch};
use system::ensure_signed;
use chainlink::{Event, create_get_parse_request};
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

		pub fn send_request(origin) -> dispatch::Result {
            let who : <T as system::Trait>::AccountId = ensure_signed(origin)?;
			Self::deposit_event(create_get_parse_request::<T>(1, 0, who, 0, vec!["https://min-api.cryptocompare.com/data/pricemultifull?fsyms=ETH&tsyms=USD", "RAW.ETH.USD.CHANGEPCTDAY", "1000000000"]));
			Ok(())
		}
    }
}