use chainlink::{CallbackWithParameter, Event, Trait as ChainlinkTrait};
use codec::{Decode, Encode};
use frame_support::{decl_module, decl_storage, dispatch::DispatchResult};
use sp_std::prelude::*;
use system::ensure_root;

pub trait Trait: chainlink::Trait + ChainlinkTrait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type Callback: From<Call<Self>> + Into<<Self as ChainlinkTrait>::Callback>;
}

decl_storage! {
    trait Store for Module<T: Trait> as ExampleStorage {
        pub Result: u128;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        pub fn send_request(origin, operator: T::AccountId) -> DispatchResult {
            let parameters = ("get", "https://min-api.cryptocompare.com/data/pricemultifull?fsyms=ETH&tsyms=USD", "path", "RAW.ETH.USD.PRICE", "times", "100000000");
            let call: <T as Trait>::Callback = Call::callback(vec![]).into();
            
            <chainlink::Module<T>>::initiate_request(origin, operator, 1, 0, parameters.encode(), 100, call.into())?;

            Ok(())
        }

        pub fn callback(origin, result: Vec<u8>) -> DispatchResult {
            ensure_root(origin)?;

            // The result is expected to be a SCALE encoded `u128`
            let r : u128 = u128::decode(&mut &result[..]).map_err(|err| err.what())?;
            <Result>::put(r);

            Ok(())
        }
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