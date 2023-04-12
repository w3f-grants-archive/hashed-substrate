#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// mod functions;
pub mod types;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::ValueQuery;
	use frame_support::pallet_prelude::*;
	use frame_support::traits::Currency;
	use frame_support::traits::UnixTime;
	use frame_system::pallet_prelude::*;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	use crate::types::*;
	use pallet_rbac::types::RoleBasedAccessControl;

	pub type BalanceOf<T> = <<T as pallet_uniques::Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_gated_marketplace::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type TimeProvider: UnixTime;
		type Rbac: RoleBasedAccessControl<Self::AccountId>;
		type RemoveOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().reads_writes(1,1))]
		pub fn initial_setup(origin: OriginFor<T>) -> DispatchResult {
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().reads_writes(1,1))]
		pub fn kill_storage(origin: OriginFor<T>) -> DispatchResult {
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().reads_writes(1,1))]
		pub fn sign_up(origin: OriginFor<T>, args: SignUpArgs) -> DispatchResult {
			let who = ensure_signed(origin)?;
			match args {
				SignUpArgs::BuyerOrSeller { first_name, last_name, email, state } => {
					let user: User<T> = User {
						first_name,
						last_name,
						email,
						lang_key: ShortString::try_from(b"en".to_vec()).unwrap(),
						created_by: Some(who.clone()),
						created_date: Some(T::TimeProvider::now().as_secs()),
						last_modified_by: Some(who.clone()),
						last_modified_date: Some(T::TimeProvider::now().as_secs()),
						phone: None,
						credits_needed: 0,
						cpa_id: ShortString::try_from(b"0".to_vec()).unwrap(),
						tax_authority_id: state,
						lock_expiration_date: None,
					};
					// <Users<T>>::insert(who, user);
				},
				SignUpArgs::CPA { first_name, last_name, email, license_number, state } => {
					let user: User<T> = User {
						first_name,
						last_name,
						email,
						lang_key: ShortString::try_from(b"en".to_vec()).unwrap(),
						created_by: Some(who.clone()),
						created_date: Some(T::TimeProvider::now().as_secs()),
						last_modified_by: Some(who.clone()),
						last_modified_date: Some(T::TimeProvider::now().as_secs()),
						phone: None,
						credits_needed: 0,
						cpa_id: license_number,
						tax_authority_id: state,
						lock_expiration_date: None,
					};
					// <Users<T>>::insert(who, user);
				},
			}

			// ! add this user to gatedMarketplace
			Ok(())
		}
	}
}
