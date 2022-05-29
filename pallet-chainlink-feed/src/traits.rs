//! Traits
use crate::{Config, RoundData};

/// This implementation wille be as a callback when the round answer updates
pub trait OnAnswerHandler<T: Config>
where
	<T as frame_system::Config>::AccountId: Default,
{
	fn on_answer(feed: T::FeedId, new_data: RoundData<T::BlockNumber, T::Value>);
}

impl<T: Config> OnAnswerHandler<T> for ()
where
	<T as frame_system::Config>::AccountId: Default,
{
	fn on_answer(_feed: T::FeedId, _new_data: RoundData<T::BlockNumber, T::Value>) {
		// do_nothing
	}
}
