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

	trait Store for Module<T: Trait> as TemplateModule {

		ProcessingDeposit get(fn something): Option<u32>;
		ProcessingWithdrawal get(fn something): Option<u32>;

		CompletedDeposit get(fn something): Option<u32>;
		CompletedWithdrawal get(fn something): Option<u32>;		

		TokenList get(fn something): Option<u32>;2>;
		TokenWallet get(fn something): Option<u32>;2>;

		BridgeOwner get(fn something): Option<u32>;
		PausedOperation get(fn something): Option<u32>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {

		DepositInitiated(u32, AccountId),
		DepositCancelled(u32, AccountId),
		DepositCompleted(u32, AccountId),
		WithdrawalInitiated(u32, AccountId),
		WithdrawalCancelled(u32, AccountId),		
		WithdrawalCompleted(u32, AccountId),	
		
		UpdatedTokenList(),
		UpdatedBridgeOwner(),
		PausedOperation(),
		UnpausedOperation()
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		NoneValue,
		StorageOverflow,

		AmountDepositedUnderLimit,
		AmountWithdrawdUnderLimit,
		AmountDepositedAboveLimit,
		AmountWithdrawdAboveLimit,
		
		TokenNotSupported,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		
		type Error = Error<T>;

		fn deposit_event() = default;

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn initiate_deposit(origin, something: u32) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn cancel_deposit(origin, something: u32) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}		

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn complete_deposit(origin, something: u32) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Have to mint token

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}		
		
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn initiate_withdrawal(origin, something: u32) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}	

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn cancel_withdrawal(origin, something: u32) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}			
		
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn complete_withdrawal(origin, something: u32) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Have to burn token

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}	
		
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn update_bridge_configuration(origin, something: u32) -> dispatch::DispatchResult {

			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}			

	}
}
