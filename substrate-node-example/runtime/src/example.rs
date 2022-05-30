pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use codec::{Decode, Encode};
    use frame_support::{
        sp_runtime::traits::UniqueSaturatedFrom,
        traits::Currency,
        dispatch::DispatchResult,
        pallet_prelude::{StorageValue, ValueQuery},
    };
    use frame_system::ensure_root;
    use frame_system::pallet_prelude::OriginFor;
    use pallet_chainlink::{CallbackWithParameter, Config as ChainlinkTrait};
    use sp_std::{prelude::*, convert::{TryInto}};

    type BalanceOf<T> = <<T as pallet_chainlink::Config>::Currency as Currency<
	          <T as frame_system::Config>::AccountId,
        >>::Balance;

    #[pallet::pallet]
		#[pallet::generate_store(pub (super) trait Store)]
		#[pallet::without_storage_info]
		pub struct Pallet<T>(_);

    #[pallet::config]
		pub trait Config: frame_system::Config + pallet_chainlink::Config {
        // type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	      type Callback: From<Call<Self>> + Into<<Self as ChainlinkTrait>::Callback>;
    }

    #[pallet::storage]
		#[pallet::getter(fn operator)]
		pub type Result<T: Config> = StorageValue<_, i128, ValueQuery>;


    #[pallet::call]
		impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
		    pub fn send_request(origin: OriginFor<T>, operator: T::AccountId, specid: Vec<u8>) -> DispatchResult {
			      let parameters = ("get", "https://min-api.cryptocompare.com/data/pricemultifull?fsyms=ETH&tsyms=USD", "path", "RAW.ETH.USD.PRICE", "times", "100000000");
			      let call: <T as Config>::Callback = Call::callback {result: vec![]}.into();

			      let fee = BalanceOf::<T>::unique_saturated_from(100u32);
			      <pallet_chainlink::Pallet<T>>::initiate_request(origin, operator, specid, 0, parameters.encode(), fee, call.into())?;

			      Ok(())
		    }


        #[pallet::weight(0)]
		    pub fn callback(origin: OriginFor<T>, result: Vec<u8>) -> DispatchResult {
			      ensure_root(origin)?;

			      // The result is expected to be a SCALE encoded `i128`
			      let r : i128 = i128::decode(&mut &result[..]).map_err(|_| Error::<T>::DecodingFailed)?;
			      Result::<T>::put(r);

			      Ok(())
		    }
    }

    #[pallet::error]
		pub enum Error<T> {
			  DecodingFailed,
		}


    impl<T: Config> CallbackWithParameter for Call<T> {
	      fn with_result(&self, result: Vec<u8>) -> Option<Self> {
		        match *self {
			          Call::callback{..} => Some(Call::callback{result}),
			          _ => None,
		        }
	      }
    }
}
