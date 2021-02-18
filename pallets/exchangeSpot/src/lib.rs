#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {

	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {

	trait SpotExchangeStore for Module<T: Trait> as SpotExchangeModule {

		Asset get(fn something): Option<u32>;
		Pair get(fn something): Option<u32>;

		OrderBook get(fn something): Option<u32>;
		OrderTrade get(fn something): Option<u32>;

		MarginDeposit get(fn something): Option<u32>;
		MarginLent get(fn something): Option<u32>;

		// Configuration set by the Exchange
		SpotMinimumVolume get(fn something): Option<u32>;
		SpotMaximumVolume get(fn something): Option<u32>;
		MarginAllowableMultiplier get(fn something): Option<u32>;
		MarginInterestMultiplier get(fn something): Option<u32>;
		SpotExchangeOwner get(fn something): Option<u32>;
		SpotPausedOperation get(fn something): Option<u32>;		
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		
		AssetAdded(u32),
		AssetRemoved(u32),
		PairAdded(u32),
		PairRemoved(u32)

		BuyOrderCreated(u32),
		BuyOrderCancelled(u32),
		BuyOrderModified(u32),

		SellOrderCreated(u32),
		SellOrderCancelled(u32),
		SellOrderModified(u32),
	
		OrderFullyMatched(u32),
		OrderPartiallyMatched(u32),

		MarginDeposited(u32),
		MarginWithdrew(u32),
		MarginCallWarning(u32), // at every threshold above 80% margin, it will emit this
		MarginCalled(u32), // When it reaches 90% of margin deposit, it will close

		SpotOperationPause(u32),
		SpotOperationUnpause(u32),

	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {

		AssetIsNotListed,
		PairIsNotListed,

		OrderVolumeBelowLimit,
		OrderVolumeAboveLimit,

		DepositBelowLimit,
		DepositAboveLimit,

		MarginAmountBelowLimit,
		MarginAmountAboveLimit,		
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		fn create_order_event() = default;

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn create_order(origin, something: u32) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update orderbook

			// Emit an event.
			Self::create_order_event(RawEvent::SomethingStored(something, who));

			Ok(())			
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn cancel_order(origin, something: u32) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update orderbook

			// Emit an event.
			Self::cancel_order_event(RawEvent::SomethingStored(something, who));

			Ok(())			
		}	

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn create_margin_order(origin, something: u32) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update orderbook

			// Emit an event.
			Self::create_order_event(RawEvent::SomethingStored(something, who));

			Ok(())			
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn cancel_margin_order(origin, something: u32) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update orderbook

			// Emit an event.
			Self::cancel_order_event(RawEvent::SomethingStored(something, who));

			Ok(())			
		}		
		
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn match_order(origin) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update orderbook

			// Emit an event.
			Self::cancel_order_event(RawEvent::SomethingStored(something, who));

			Ok(())			
		}	
		
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn calculate_margin(origin) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update orderbook

			// Emit an event.
			Self::cancel_order_event(RawEvent::SomethingStored(something, who));

			Ok(())			
		}	
		
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn call_margin(origin) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update orderbook

			// Emit an event.
			Self::cancel_order_event(RawEvent::SomethingStored(something, who));

			Ok(())			
		}		

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn deposit_margin(origin) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update orderbook

			// Emit an event.
			Self::cancel_order_event(RawEvent::SomethingStored(something, who));

			Ok(())			
		}			
		
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn withdraw_margin(origin) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update orderbook

			// Emit an event.
			Self::cancel_order_event(RawEvent::SomethingStored(something, who));

			Ok(())			
		}	
		
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn add_margin_interest(origin) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update orderbook

			// Emit an event.
			Self::cancel_order_event(RawEvent::SomethingStored(something, who));

			Ok(())			
		}				
		
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn add_asset(origin) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update orderbook

			// Emit an event.
			Self::cancel_order_event(RawEvent::SomethingStored(something, who));

			Ok(())			
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn add_pair(origin) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update orderbook

			// Emit an event.
			Self::cancel_order_event(RawEvent::SomethingStored(something, who));

			Ok(())			
		}
		
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn remove_asset(origin) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update orderbook

			// Emit an event.
			Self::cancel_order_event(RawEvent::SomethingStored(something, who));

			Ok(())			
		}			
		
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn remove_pair(origin) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update orderbook

			// Emit an event.
			Self::cancel_order_event(RawEvent::SomethingStored(something, who));

			Ok(())			
		}			
				
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn update_exchange_configuration(origin) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update orderbook

			// Emit an event.
			Self::cancel_order_event(RawEvent::SomethingStored(something, who));

			Ok(())			
		}			

		/// An example dispatchable that may throw a custom error.
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match Something::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					Something::put(new);
					Ok(())
				},
			}
		}
	}
}
