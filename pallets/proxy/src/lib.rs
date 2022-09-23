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
	use frame_support::{pallet_prelude::{*, ValueQuery}, BoundedVec};
	use frame_system::pallet_prelude::*;
	use frame_support::transactional;
	use sp_runtime::traits::Scale;
	use frame_support::traits::{Time};

	use crate::types::*;
	use pallet_rbac::types::RoleBasedAccessControl;


	#[pallet::config]
	pub trait Config: frame_system::Config {
		//TODO: change all accounts names for users
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		
		type Moment: Parameter
		+ Default
		+ Scale<Self::BlockNumber, Output = Self::Moment>
		+ Copy
		+ MaxEncodedLen
		+ scale_info::StaticTypeInfo
		+ Into<u64>;

		type Timestamp: Time<Moment = Self::Moment>;

		type Rbac : RoleBasedAccessControl<Self::AccountId>;

		type RemoveOrigin: EnsureOrigin<Self::Origin>;		

		#[pallet::constant]
		type ProjectNameMaxLen: Get<u32>;

		#[pallet::constant]
		type ProjectDescMaxLen: Get<u32>;

		#[pallet::constant]
		type MaxChildrens: Get<u32>;

		#[pallet::constant]
		type MaxDocuments: Get<u32>;

		#[pallet::constant]
		type MaxAccountsPerTransaction: Get<u32>;

		#[pallet::constant]
		type MaxProjectsPerUser: Get<u32>;

		#[pallet::constant]
		type MaxUserPerProject: Get<u32>;

		#[pallet::constant]
		type CIDMaxLen: Get<u32>;

		#[pallet::constant]
		type MaxDevelopersPerProject: Get<u32>;

		#[pallet::constant]
		type MaxInvestorsPerProject: Get<u32>;

		#[pallet::constant]
		type MaxIssuersPerProject: Get<u32>;

		#[pallet::constant]
		type MaxRegionalCenterPerProject: Get<u32>;

		#[pallet::constant]
		type MaxBoundedVecs: Get<u32>;

		#[pallet::constant]
		type MaxExpendituresPerProject: Get<u32>;


	
		
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/*--- Onchain storage section ---*/

	#[pallet::storage]
	#[pallet::getter(fn global_scope)]
	pub(super) type GlobalScope<T> = StorageValue<_, [u8;32], ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn users_info)]
	pub(super) type UsersInfo<T: Config> = StorageMap<
		_, 
		Identity, 
		T::AccountId, // Key
		UserData<T>,  // Value
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn projects)]
	pub(super) type ProjectsInfo<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8;32], // Key project_id
		ProjectData<T>,  // Value
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn users_by_project)]
	pub(super) type UsersByProject<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8;32], // Key project_id
		BoundedVec<T::AccountId, T::MaxUserPerProject>,  // Value users
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn projects_by_user)]
	pub(super) type ProjectsByUser<T: Config> = StorageMap<
		_, 
		Identity, 
		T::AccountId, // Key account_id
		BoundedVec<[u8;32], T::MaxProjectsPerUser>,  // Value projects
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn expenditures)]
	pub(super) type ExpendituresInfo<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8;32], // Key expenditure_id
		ExpenditureData<T>,  // Value expenditure
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn expenditures_by_project)]
	pub(super) type ExpendituresByProject<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8;32], // Key project_id
		BoundedVec<[u8;32], T::MaxExpendituresPerProject>,  // Value expenditures
		ValueQuery,
	>;





	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Project was created
		ProjectCreated(T::AccountId, [u8;32]),
		/// Proxy setup completed
		ProxySetupCompleted,
		/// User registered successfully
		UserAdded(T::AccountId),
		/// Project was edited
		ProjectEdited([u8;32]),
		/// Project was deleted
		ProjectDeleted([u8;32]),
		/// Administator added
		AdministratorAssigned(T::AccountId),
		/// Administator removed
		AdministratorRemoved(T::AccountId),
		/// User assigned to project
		UserAssignedToProject(T::AccountId, [u8;32]),
		/// User removed from project
		UserUnassignedFromProject(T::AccountId, [u8;32]),
		/// User info updated
		UserUpdated(T::AccountId),
		/// User removed
		UserDeleted(T::AccountId),
		/// Expenditure was created successfully
		ExpenditureCreated([u8;32]),

	}

	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		/// TODO: map each constant type used by bounded vecs to a descriptive error
		NoneValue,
		/// Project ID is already in use
		ProjectIdAlreadyInUse,
		/// Timestamp error
		TimestampError,
		/// Completition date must be later than creation date
		CompletitionDateMustBeLater,
		/// User is already registered
		UserAlreadyRegistered,
		/// Project is not found
		ProjectNotFound,
		///Date can not be in the past
		DateCanNotBeInThePast,
		/// Can not modify project
		CannotEditCompletedProject,
		/// Creation date must be in the past
		CreationDateMustBeInThePast,
		/// Can not delete a completed project
		CannotDeleteCompletedProject,
		/// Global scope is not set
		GlobalScopeNotSet,
		/// User is not registered
		UserNotRegistered,
		/// User has been already added to the project
		UserAlreadyAssignedToProject,
		/// Max number of users per project reached
		MaxUsersPerProjectReached,
		/// Max number of projects per user reached
		MaxProjectsPerUserReached,
		/// User already has the role
		UserAlreadyHasRole,
		/// User is not assigned to the project
		UserNotAssignedToProject,
		/// Can not register administator role 
		CannotRegisterAdminRole,
		/// Max number of developers per project reached
		MaxDevelopersPerProjectReached,
		/// Max number of investors per project reached
		MaxInvestorsPerProjectReached,
		/// Max number of issuers per project reached
		MaxIssuersPerProjectReached,
		/// Max number of regional centers per project reached
		MaxRegionalCenterPerProjectReached,
		/// Can not remove administator role
		CannotRemoveAdminRole,
		/// Can not delete an user with active projects
		CannotDeleteUserWithAssignedProjects,
		/// Can not add admin role at user project assignment
		CannotAddAdminRole,
		/// User can not have more than one role at the same time
		UserCannotHaveMoreThanOneRole,
		/// Cannot create parent expenditure
		CannotCreateParentExpenditure, 
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// I N I T I A L 
		// --------------------------------------------------------------------------------------------
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(10))]
		pub fn initial_setup(origin: OriginFor<T>) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			Self::do_initial_setup()?;
			Ok(())
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(10))]
		pub fn sudo_add_administrator(origin: OriginFor<T>, admin: T::AccountId) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			Self::do_sudo_add_administrator(admin)?;
			Ok(())
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(10))]
		pub fn sudo_remove_administrator(origin: OriginFor<T>, admin: T::AccountId) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			Self::do_sudo_remove_administrator(admin)?;
			Ok(())
		}




		// U S E R S
		// --------------------------------------------------------------------------------------------
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn users_register_user(
			origin: OriginFor<T>, 
			user: T::AccountId, 
			name: FieldName,
			image: CID,
			email: FieldName,
			documents: Option<Documents<T>> 
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_register_user(who, user, name, image, email, documents)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn users_update_user(
			origin: OriginFor<T>, 
			user: T::AccountId, 
			name: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>,
			image: Option<BoundedVec<CID, T::MaxBoundedVecs>>,
			email: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>,
			documents: Option<Documents<T>> 
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_update_user(who, user, name, image, email, documents)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn users_delete_user(
			origin: OriginFor<T>, 
			user: T::AccountId, 
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_delete_user(who, user)
		}
		
		// P R O J E C T S
		// --------------------------------------------------------------------------------------------
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn projects_create_project(
			origin: OriginFor<T>, 
			tittle: FieldName, 
			description: FieldDescription, 
			image: CID, 
			adress: FieldName,
			completition_date: u64, 
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_create_project(who, tittle, description, image, adress, completition_date)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn projects_edit_project(
			origin: OriginFor<T>, 
			project_id: [u8;32], 
			tittle: Option<FieldName>,
			description: Option<FieldDescription>, 
			image: Option<CID>, 
			adress: Option<FieldName>,
			creation_date: Option<u64>, 
			completition_date: Option<u64>,  
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_edit_project(who, project_id, tittle, description, image, adress, creation_date, completition_date)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn projects_delete_project(
			origin: OriginFor<T>, 
			project_id: [u8;32],  
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_delete_project(who, project_id)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn projects_assign_user(
			origin: OriginFor<T>, 
			user: T::AccountId,
			project_id: [u8;32],  
			role: ProxyRole,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_assign_user(who, user, project_id, role)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn projects_unassign_user(
			origin: OriginFor<T>, 
			user: T::AccountId,
			project_id: [u8;32],  
			role: ProxyRole,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_unassign_user(who, user, project_id, role)
		}
		//TOOD: Move this logic to edit after friday demo
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn extrinsic_testing_edit_project(
			origin: OriginFor<T>, 
			project_id: [u8;32], 
			tittle: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>,	
			description: Option<BoundedVec<FieldDescription, T::MaxBoundedVecs>>,
			image: Option<BoundedVec<CID, T::MaxBoundedVecs>>,
			adress: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>,
			creation_date: Option<u64>, 
			completition_date: Option<u64>, 
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_extrinisc_testing(who, project_id, tittle, description, image, adress, creation_date, completition_date)
		}

		// B U D G E T  E X P E N D I T U R E 
		// --------------------------------------------------------------------------------------------
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn expenditures_create_expenditure(
			origin: OriginFor<T>, 
			project_id: [u8;32], 
			name: FieldName,
			expenditure_type: ExpenditureType,
			expenditure_subtype: ExpenditureSubType,
			budget_amount: Option<u64>,
			naics_code: Option<u32>,
			jobs_multiplier: Option<u32>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_create_expenditure(who, project_id, name, expenditure_type, expenditure_subtype, budget_amount, naics_code, jobs_multiplier)
		}

	}
}