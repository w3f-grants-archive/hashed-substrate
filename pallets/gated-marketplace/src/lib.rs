#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod functions;
mod types;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::{*, OptionQuery}};
	use frame_system::pallet_prelude::*;
	use crate::types::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		#[pallet::constant]
		type MaxMarketsPerAuth: Get<u32>;
		#[pallet::constant]
		type MaxApplicants: Get<u32>;
		#[pallet::constant]
		type LabelMaxLen: Get<u32>;
		#[pallet::constant]
		type NotesMaxLen: Get<u32>;
		#[pallet::constant]
		type NameMaxLen: Get<u32>;
		#[pallet::constant]
		type MaxFiles: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/*--- Onchain storage section ---*/

	#[pallet::storage]
	#[pallet::getter(fn marketplaces)]
	pub(super) type Marketplaces<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8; 32], 
		Marketplace<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn marketplaces_by_authority)]
	pub(super) type MarketplacesByAuthority<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, 
		T::AccountId, 
		Blake2_128Concat, 
		MarketplaceAuthority, 
		BoundedVec<[u8;32],T::MaxMarketsPerAuth>, 
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn authorities_by_marketplace)]
	pub(super) type AuthoritiesByMarketplace<T: Config> = StorageDoubleMap<
		_, 
		Identity, 
		[u8;32], 
		Blake2_128Concat, 
		MarketplaceAuthority, 
		BoundedVec<T::AccountId,T::MaxMarketsPerAuth>, 
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn applications)]
	pub(super) type Applications<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8;32], 
		Application<T>, 
		OptionQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn applications_by_account)]
	pub(super) type ApplicationsByAccount<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, 
		T::AccountId, 
		Blake2_128Concat, 
		[u8;32], //marketplace_id 
		[u8;32], //application_id
		OptionQuery
	>;


	#[pallet::storage]
	#[pallet::getter(fn applicants_by_marketplace)]
	pub(super) type ApplicantsByMarketplace<T: Config> = StorageDoubleMap<
		_, 
		Identity, 
		[u8;32], 
		Blake2_128Concat, 
		ApplicationStatus, 
		BoundedVec<T::AccountId,T::MaxApplicants>, 
		ValueQuery
	>;



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
		/// Work In Progress
		NotYetImplemented,
		/// Error names should be descriptive.
		NoneValue,
		/// The account supervises too many marketplaces
		ExceedMaxMarketsPerAuth,
		/// Too many applicants for this market! try again later
		ExceedMaxApplicants,
		ApplicationNotFound,
		ApplicantNotFound,

	}
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_marketplace(origin: OriginFor<T>, admin: T::AccountId,label: BoundedVec<u8,T::LabelMaxLen>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let m = Marketplace{
				owner: who,
				admin,
				label,
				appraiser: None,
			};
			Self::do_create_marketplace(m)
		}
		
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn apply(
			origin: OriginFor<T>, 
			marketplace_id: [u8;32], 
			notes: BoundedVec<u8,T::NotesMaxLen>, 
			files : BoundedVec<ApplicationFile<T::NameMaxLen>, T::MaxFiles>, 
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let application = Application::<T>{
				applicant: who,
				marketplace_id, 
				status: ApplicationStatus::default(),
				notes,
				files,
			};
			Self::do_apply(application)
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn enroll(origin: OriginFor<T>, marketplace_id: [u8;32], account_or_application: AccountOrApplication<T>, approved: bool ) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			//TODO: ensure the enroller is owner or admin
			// ensure that the market exists
			Self::do_enroll(marketplace_id, account_or_application, approved)
		}

	}
}