#![cfg_attr(not(feature = "std"), no_std)]
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use sp_core::sr25519::{Public, Signature};
use sp_runtime::app_crypto::{app_crypto, RuntimePublic};
use sp_std::{prelude::*, str};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
	trait Store for Module<T: Config> as CryptoModule {
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		// Something get(fn something): Option<u32>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		VerifySr25519(AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		NoneValue,
		VerifyErr,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn verify_sr25519_ext(origin, account: [u8; 32], msg: Vec<u8>, sign: [u8; 64]) ->  Result<(), Error<T>> {

			// verify
			let public = Public::from_raw(account);
			let succ = public.verify(&msg, &Signature::from_raw(sign));
			if !succ {
				return Err(Error::VerifyErr)
			}

			// Return a successful DispatchResult
			Ok(())
		}
	}
}

impl<T: Config> Module<T> {
	pub fn verify_sr25519(account: [u8; 32], msg: [u8; 32], sign: [u8; 64]) ->  Result<(), Error<T>> {
		// verify
		let public = Public::from_raw(account);
		let succ = public.verify(&msg, &Signature::from_raw(sign));
		if !succ {
			return Err(Error::VerifyErr)
		}

		// Return a successful DispatchResult
		Ok(())
	}
	pub fn verify_sr25519_bytes(account: [u8; 32], msg: [u8; 47], sign: [u8; 64]) ->  Result<(), Error<T>> {
		// verify
		let public = Public::from_raw(account);
		let succ = public.verify(&msg, &Signature::from_raw(sign));
		if !succ {
			return Err(Error::VerifyErr)
		}

		// Return a successful DispatchResult
		Ok(())
	}
}
