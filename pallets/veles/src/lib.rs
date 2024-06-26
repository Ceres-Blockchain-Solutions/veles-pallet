#![cfg_attr(not(feature = "std"), no_std)]

pub use codec::{Decode, Encode, MaxEncodedLen};
pub use common::BoundedString;
pub use frame_support::pallet_prelude::Get;
pub use frame_support::traits::Currency;
pub use pallet::*;
pub use sp_core::{blake2_256, H256};
pub use sp_std::collections::btree_set::BTreeSet;

// This module contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
#[cfg(test)]
mod tests;

/// Global data structures
// Project Validator / Project Owner data structure
#[derive(Encode, Decode, Default, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
#[scale_info(skip_type_params(IPFSLength))]
pub struct PVoPOInfo<IPFSLength: Get<u32>, BlockNumber> {
	// IPFS link to PV/PO documentation
	documentation_ipfs: BoundedString<IPFSLength>,
	// Penalty level
	penalty_level: u8,
	// Penalty timeout
	penalty_timeout: BlockNumber,
}

// Carbon Footprint account data structure
#[derive(Encode, Decode, Default, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
#[scale_info(skip_type_params(IPFSLength))]
pub struct CFAccountInfo<MomentOf, IPFSLength: Get<u32>> {
	// IPFS link to CFA documentation
	documentation_ipfs: BoundedString<IPFSLength>,
	// Carbon credit balance
	carbon_credit_balance: i128,
	// Creation date
	creation_date: MomentOf,
}

// Carbon Footprint report data structure
#[derive(Encode, Decode, Default, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct CFReportInfo<AccountIdOf, MomentOf> {
	// Carbon footprint account
	cf_account: AccountIdOf,
	// Creation date
	creation_date: MomentOf,
	// Carbon deficit (aka Carbon footprint)
	carbon_deficit: i128,
	// Votes for
	votes_for: BTreeSet<AccountIdOf>,
	// Votes against
	votes_against: BTreeSet<AccountIdOf>,
	// Voting status
	voting_active: bool,
}

// Project Proposal info structure
#[derive(Encode, Decode, Clone, Default, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ProjectProposalInfo<AccountIdOf, MomentOf> {
	// Project owner
	project_owner: AccountIdOf,
	// Creation date
	creation_date: MomentOf,
	// Project hash
	project_hash: H256,
	// Votes for
	votes_for: BTreeSet<AccountIdOf>,
	// Votes against
	votes_against: BTreeSet<AccountIdOf>,
	// Voting status
	voting_active: bool,
}

// Carbon Credit Batch Proposal info structure
#[derive(Encode, Decode, Clone, Default, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct CCBProposalInfo<MomentOf, BalanceOf, AccountIdOf> {
	// Project hash
	project_hash: H256,
	// Carbon credit batch hash
	batch_hash: H256,
	// Creation date
	creation_date: MomentOf,
	// Carbon credit amount
	credit_amount: i128,
	// Initial carbon credit price
	initial_credit_price: BalanceOf,
	// Votes for
	votes_for: BTreeSet<AccountIdOf>,
	// Votes against
	votes_against: BTreeSet<AccountIdOf>,
	// Voting status
	voting_active: bool,
}

// Projects info structure
#[derive(Encode, Decode, Clone, Default, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
#[scale_info(skip_type_params(IPFSLength))]
pub struct ProjectInfo<IPFSLength: Get<u32>, MomentOf, BlockNumber> {
	// IPFS link to project documentation
	documentation_ipfs: BoundedString<IPFSLength>,
	// Creation date
	creation_date: MomentOf,
	// Penalty level
	penalty_level: u8,
	// Penalty timeout
	penalty_timeout: BlockNumber,
}

// Carbon credit batch info structure
#[derive(Encode, Decode, Clone, Default, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
#[scale_info(skip_type_params(IPFSLength))]
pub struct CCBInfo<IPFSLength: Get<u32>, MomentOf, BalanceOf, VoteType> {
	// IPFS link to CFA documentation
	documentation_ipfs: BoundedString<IPFSLength>,
	// Creation date
	creation_date: MomentOf,
	// Carbon credit amount
	credit_amount: i128,
	// Initial carbon credit price
	initial_credit_price: BalanceOf,
	// Batch status
	status: VoteType,
}

// Penalty level structure for carbon footprint
#[derive(Encode, Decode, Default, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct PenaltyLevelConfig {
	pub level: u8, // Penalty level
	pub base: i32, // Balance
}

// Vote type enum
#[derive(Encode, Decode, PartialEq, Eq, scale_info::TypeInfo, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum VoteType {
	CFReportVote,
	PProposalVote,
	CCBatchVote,
}

// Carbon credit batch status
#[derive(Encode, Decode, PartialEq, Eq, scale_info::TypeInfo, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum CCBStatus {
	Active,   // Tokens can be traded and retired
	Frozen,   // Tokens can't be traded or retired
	Redacted, // Tokens have been removed from circulation
}

// Timeout type
#[derive(Encode, Decode, PartialEq, Eq, scale_info::TypeInfo, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum TimeoutType {
	Penalty, // Penalty timeout type
	Voting,  // Voting timeout type
	Sales,   // Sales timeout type
}

// Fee type
#[derive(Encode, Decode, PartialEq, Eq, scale_info::TypeInfo, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum FeeType {
	TraderAccountFee,           // Trader acccount registration fee
	ProjectValidatorAccountFee, // Project validator account registration fee
	ProjectOwnerAccountFee,     // Project owner account registration fee
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_support::traits::ReservableCurrency;
	use frame_support::traits::Time;
	use frame_system::offchain::{SendTransactionTypes, SubmitTransaction};
	use frame_system::pallet_prelude::*;
	use sp_std::collections::btree_set::BTreeSet;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Pallet configuration
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type IPFSLength: Get<u32>;
		type CarboCreditDecimal: Get<u8>;
		type Time: Time;
		type Currency: ReservableCurrency<Self::AccountId>;

		#[pallet::constant]
		type PenaltyLevelsConfiguration: Get<[PenaltyLevelConfig; 5]>;
		type UnsignedPriority: Get<TransactionPriority>;
		type UnsignedLongevity: Get<u64>;
	}

	/// Pallet types and constants
	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	type MomentOf<T> = <<T as Config>::Time as Time>::Moment;
	type BlockNumber<T> = BlockNumberFor<T>;
	type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

	/// Helper functions
	// Default authority accounts
	#[pallet::type_value]
	pub fn DefaultForAuthorityAccounts<T: Config>() -> BTreeSet<AccountIdOf<T>> {
		let set: BTreeSet<AccountIdOf<T>> = BTreeSet::<AccountIdOf<T>>::new();

		set
	}

	// Default trade accounts
	#[pallet::type_value]
	pub fn DefaultForTraderAccounts<T: Config>() -> BTreeSet<AccountIdOf<T>> {
		let set: BTreeSet<AccountIdOf<T>> = BTreeSet::<AccountIdOf<T>>::new();

		set
	}

	// Default trader account fee
	#[pallet::type_value]
	pub fn DefaultForTraderAccountFee<T: Config>() -> BalanceOf<T> {
		let fee = BalanceOf::<T>::from(100u32);

		fee
	}

	// Default project validator account fee
	#[pallet::type_value]
	pub fn DefaultForProjectValidatorAccountFee<T: Config>() -> BalanceOf<T> {
		let fee = BalanceOf::<T>::from(100u32);

		fee
	}

	// Default project owner account fee
	#[pallet::type_value]
	pub fn DefaultForProjectOwnerAccountFee<T: Config>() -> BalanceOf<T> {
		let fee = BalanceOf::<T>::from(100u32);

		fee
	}

	// Default pallet base time (in blocks) of pallet 
	// Note: Used to see if proposals can be submitted
	#[pallet::type_value]
	pub fn DefaultForPalletBaseTime<T: Config>() -> BlockNumber<T> {
		let current_block = frame_system::Pallet::<T>::block_number();	// Current block number

		current_block
	}

	// Default penalty timeout time [in blocks]
	#[pallet::type_value]
	pub fn DefaultForPenaltyTimeoutTime<T: Config>() -> BlockNumber<T> {
		let timeout_time: u32 = 31 * 24 * 60 * 60; // A MONTH in seconds
		let block_finalization_time: u32 = 8; // Predefined block finalization time

		BlockNumber::<T>::from(timeout_time / block_finalization_time)
	}

	// Default voting timeout time [in blocks]
	#[pallet::type_value]
	pub fn DefaultForVotingTimeoutTime<T: Config>() -> BlockNumber<T> {
		let timeout_time: u32 = 7 * 24 * 60 * 60; // A WEEK in seconds
		let block_finalization_time: u32 = 8; // Predefined block finalization time

		BlockNumber::<T>::from(timeout_time / block_finalization_time)
	}

	// Default sales timeout time [in blocks]
	#[pallet::type_value]
	pub fn DefaultForSalesTimeoutTime<T: Config>() -> BlockNumber<T> {
		let timeout_time: u32 = 7 * 24 * 60 * 60; // A WEEK in seconds
		let block_finalization_time: u32 = 8; // Predefined block finalization time

		BlockNumber::<T>::from(timeout_time / block_finalization_time)
	}

	/// Pallet storages
	// Trader account fee
	#[pallet::storage]
	#[pallet::getter(fn trader_account_fee)]
	pub type TraderAccountFee<T: Config> =
		StorageValue<_, BalanceOf<T>, ValueQuery, DefaultForTraderAccountFee<T>>;

	// Project validator account fee
	#[pallet::storage]
	#[pallet::getter(fn project_validator_account_fee)]
	pub type ProjectValidatorAccountFee<T: Config> =
		StorageValue<_, BalanceOf<T>, ValueQuery, DefaultForProjectValidatorAccountFee<T>>;

	// Project owner account fee
	#[pallet::storage]
	#[pallet::getter(fn project_owner_account_fee)]
	pub type ProjectOwnerAccountFee<T: Config> =
		StorageValue<_, BalanceOf<T>, ValueQuery, DefaultForProjectOwnerAccountFee<T>>;

	// Pallet base time
	#[pallet::storage]
	#[pallet::getter(fn pallet_base_time)]
	pub type BasePalletTime<T: Config> =
		StorageValue<_, BlockNumber<T>, ValueQuery, DefaultForPalletBaseTime<T>>;

	// Penalty timeout time
	#[pallet::storage]
	#[pallet::getter(fn penalty_timeout_time)]
	pub type PenaltyTimeoutTime<T: Config> =
		StorageValue<_, BlockNumber<T>, ValueQuery, DefaultForPenaltyTimeoutTime<T>>;

	// Voting timeout time
	#[pallet::storage]
	#[pallet::getter(fn voting_timeout_time)]
	pub type VotingTimeoutTime<T: Config> =
		StorageValue<_, BlockNumber<T>, ValueQuery, DefaultForVotingTimeoutTime<T>>;

	// Sales timeout time
	#[pallet::storage]
	#[pallet::getter(fn sales_timeout_time)]
	pub type SalesTimeoutTime<T: Config> =
		StorageValue<_, BlockNumber<T>, ValueQuery, DefaultForSalesTimeoutTime<T>>;

	// Authority accounts
	#[pallet::storage]
	#[pallet::getter(fn authority_accounts)]
	pub type AuthorityAccounts<T: Config> =
		StorageValue<_, BTreeSet<AccountIdOf<T>>, ValueQuery, DefaultForAuthorityAccounts<T>>;

	// Carbon Footprint accounts
	#[pallet::storage]
	#[pallet::getter(fn carbon_footprint_accounts)]
	pub(super) type CarbonFootprintAccounts<T: Config> = StorageMap<
		_,
		Identity,
		AccountIdOf<T>,
		CFAccountInfo<MomentOf<T>, T::IPFSLength>,
		OptionQuery,
	>;

	// Trader accounts
	#[pallet::storage]
	#[pallet::getter(fn trader_accounts)]
	pub type TraderAccounts<T: Config> =
		StorageValue<_, BTreeSet<AccountIdOf<T>>, ValueQuery, DefaultForTraderAccounts<T>>;

	// Project Validator accounts
	#[pallet::storage]
	#[pallet::getter(fn project_validators)]
	pub(super) type ProjectValidators<T: Config> = StorageMap<
		_,
		Identity,
		AccountIdOf<T>,
		PVoPOInfo<T::IPFSLength, BlockNumber<T>>,
		OptionQuery,
	>;

	// Project Owner accounts
	#[pallet::storage]
	#[pallet::getter(fn project_owners)]
	pub(super) type ProjectOwners<T: Config> = StorageMap<
		_,
		Identity,
		AccountIdOf<T>,
		PVoPOInfo<T::IPFSLength, BlockNumber<T>>,
		OptionQuery,
	>;

	// Projects
	#[pallet::storage]
	#[pallet::getter(fn projects)]
	pub(super) type Projects<T: Config> = StorageMap<
		_,
		Identity,
		H256,
		ProjectInfo<T::IPFSLength, MomentOf<T>, BlockNumber<T>>,
		OptionQuery,
	>;

	// Penalty timeouts (for AccountID's)
	#[pallet::storage]
	#[pallet::getter(fn penalty_timeouts_accounts)]
	pub(super) type PenaltyTimeoutsAccounts<T: Config> =
		StorageMap<_, Identity, BlockNumber<T>, BTreeSet<AccountIdOf<T>>, OptionQuery>;

	// Penalty timeouts (for hashes)
	#[pallet::storage]
	#[pallet::getter(fn penalty_timeouts_hashes)]
	pub(super) type PenaltyTimeoutsHashes<T: Config> =
		StorageMap<_, Identity, BlockNumber<T>, BTreeSet<H256>, OptionQuery>;

	// Voting timeouts
	#[pallet::storage]
	#[pallet::getter(fn voting_timeouts)]
	pub(super) type VotingTimeouts<T: Config> = StorageMap<
		_,
		Identity,
		BlockNumber<T>,
		BTreeSet<BoundedString<T::IPFSLength>>,
		OptionQuery,
	>;

	// Sales timeouts
	#[pallet::storage]
	#[pallet::getter(fn sales_timeouts)]
	pub(super) type SalesTimeouts<T: Config> =
		StorageMap<_, Identity, BlockNumber<T>, BTreeSet<H256>, OptionQuery>;

	// Carbon footprint reports
	#[pallet::storage]
	#[pallet::getter(fn carbon_footprint_reports)]
	pub(super) type CFReports<T: Config> = StorageMap<
		_,
		Identity,
		BoundedString<T::IPFSLength>,
		CFReportInfo<AccountIdOf<T>, MomentOf<T>>,
		OptionQuery,
	>;

	// Projects proposals
	#[pallet::storage]
	#[pallet::getter(fn project_proposals)]
	pub(super) type ProjectProposals<T: Config> = StorageMap<
		_,
		Identity,
		BoundedString<T::IPFSLength>,
		ProjectProposalInfo<AccountIdOf<T>, MomentOf<T>>,
		OptionQuery,
	>;

	// Carbon Credit Batch proposals
	#[pallet::storage]
	#[pallet::getter(fn carbon_credit_batch_proposals)]
	pub(super) type CCBProposals<T: Config> = StorageMap<
		_,
		Identity,
		BoundedString<T::IPFSLength>,
		CCBProposalInfo<MomentOf<T>, BalanceOf<T>, AccountIdOf<T>>,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Timeout time updated
		TimeoutTimeUpdated(TimeoutType, BlockNumber<T>),
		/// Fee amount updated
		FeeAmountUpdated(FeeType, BalanceOf<T>),
		/// Trader Account Registered
		TraderAccountRegistered(AccountIdOf<T>),
		/// Project Validator Account Registered
		ProjectValidatorAccountRegistered(AccountIdOf<T>, BoundedString<T::IPFSLength>),
		/// Project Owner Account Registered
		ProjectOwnerAccountRegistered(AccountIdOf<T>, BoundedString<T::IPFSLength>),
		/// Successful Vote Cast
		SuccessfulVote(AccountIdOf<T>, BoundedString<T::IPFSLength>, VoteType, bool),
		/// Successful Project Proposal Created
		ProjectProposalCreated(AccountIdOf<T>, BoundedString<T::IPFSLength>),
		/// Carbon Credit Batch Proposal Created
		CarbonCreditBatchProposalCreated(AccountIdOf<T>, BoundedString<T::IPFSLength>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Trader already exists
		TraderAlreadyExists,
		/// Project validator already exists
		ProjectValidatorAlreadyExists,
		/// Project validator already exists
		ProjectOwnerAlreadyExists,
		/// Insufficient funds
		InsufficientFunds,
		/// Report not found
		CFReportNotFound,
		/// Not Authorized
		Unauthorized,
		/// Invalid timeout value
		InvalidTimeoutValue,
		/// Documentation (IPFS link) was used previously
		DocumentationWasUsedPreviously,
		/// Voting cycle is over
		VotingCycleIsOver,
		/// Vote already submitted
		VoteAlreadySubmitted,
		/// Project proposal already exists
		ProjectProposalAlreadyExists,
		/// Project Proposal not found
		ProjectProposalNotFound,
		/// Carbon credit batch proposal not found
		CCBProposalNotFound,
		/// Wrong vote type
		WrongVoteType,
		/// Project doesn't exist
		ProjectDoesntExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Change specific timeout time
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn change_timeout_time(
			origin: OriginFor<T>,
			timeout_type: TimeoutType,
			new_timeout_time: BlockNumber<T>,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			// Check if caller is a Authority account
			ensure!(
				AuthorityAccounts::<T>::get().contains(&user.clone()),
				Error::<T>::Unauthorized
			);

			// Check if the new timeout time is not 0
			ensure!(
				new_timeout_time != BlockNumber::<T>::from(0u32),
				Error::<T>::InvalidTimeoutValue
			);

			match timeout_type {
				TimeoutType::Penalty => {
					PenaltyTimeoutTime::<T>::set(new_timeout_time);
				},
				TimeoutType::Voting => {
					VotingTimeoutTime::<T>::set(new_timeout_time);
				},
				TimeoutType::Sales => {
					SalesTimeoutTime::<T>::set(new_timeout_time);
				},
			}

			Self::deposit_event(Event::TimeoutTimeUpdated(timeout_type, new_timeout_time));

			Ok(().into())
		}

		// Change specific fee amount
		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn change_fee_amount(
			origin: OriginFor<T>,
			fee_type: FeeType,
			new_fee_amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			// Check if caller is a Authority account
			ensure!(
				AuthorityAccounts::<T>::get().contains(&user.clone()),
				Error::<T>::Unauthorized
			);

			match fee_type {
				FeeType::TraderAccountFee => {
					TraderAccountFee::<T>::set(new_fee_amount);
				},
				FeeType::ProjectValidatorAccountFee => {
					ProjectValidatorAccountFee::<T>::set(new_fee_amount);
				},
				FeeType::ProjectOwnerAccountFee => {
					ProjectOwnerAccountFee::<T>::set(new_fee_amount);
				},
			}

			Self::deposit_event(Event::FeeAmountUpdated(fee_type, new_fee_amount));

			Ok(().into())
		}

		// Register for a trader account
		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn register_for_trader_account(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			// Check if caller is already has a associated trader account
			ensure!(
				!TraderAccounts::<T>::get().contains(&user.clone()),
				Error::<T>::TraderAlreadyExists
			);

			// Check if caller has sufficient funds
			ensure!(
				TraderAccountFee::<T>::get() <= T::Currency::free_balance(&user.clone()),
				Error::<T>::InsufficientFunds
			);

			// Insert trader account
			let mut new_traders = TraderAccounts::<T>::get();
			new_traders.insert(user.clone());
			TraderAccounts::<T>::set(new_traders);

			// Deposit event
			Self::deposit_event(Event::TraderAccountRegistered(user.clone()));

			Ok(().into())
		}

		// Register for a project validator account
		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn register_for_project_validator_account(
			origin: OriginFor<T>,
			documentation_ipfs: BoundedString<T::IPFSLength>,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			// Check if caller is already has a associated project validator account
			ensure!(
				!ProjectValidators::<T>::contains_key(&user.clone()),
				Error::<T>::ProjectValidatorAlreadyExists
			);

			// Check if the documentation (IPFS link) has been used previously
			ensure!(
				Self::is_ipfs_available(documentation_ipfs.clone()),
				Error::<T>::DocumentationWasUsedPreviously
			);

			// Check if caller has sufficient funds
			ensure!(
				ProjectValidatorAccountFee::<T>::get() <= T::Currency::free_balance(&user.clone()),
				Error::<T>::InsufficientFunds
			);

			// Insert project validator account
			let pvalidator_info: PVoPOInfo<T::IPFSLength, BlockNumber<T>> = PVoPOInfo {
				documentation_ipfs: documentation_ipfs.clone(),
				penalty_level: 0,
				penalty_timeout: BlockNumber::<T>::from(0u32),
			};
			ProjectValidators::<T>::insert(user.clone(), pvalidator_info);

			// Deposit event
			Self::deposit_event(Event::ProjectValidatorAccountRegistered(
				user.clone(),
				documentation_ipfs.clone(),
			));

			Ok(().into())
		}

		// Register for a project owner account
		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn register_for_project_owner_account(
			origin: OriginFor<T>,
			documentation_ipfs: BoundedString<T::IPFSLength>,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			// Check if caller is already has a associated project owner account
			ensure!(
				!ProjectOwners::<T>::contains_key(&user.clone()),
				Error::<T>::ProjectOwnerAlreadyExists
			);

			// Check if the documentation (IPFS link) has been used previously
			ensure!(
				Self::is_ipfs_available(documentation_ipfs.clone()),
				Error::<T>::DocumentationWasUsedPreviously
			);

			// Check if caller has sufficient funds
			ensure!(
				ProjectOwnerAccountFee::<T>::get() <= T::Currency::free_balance(&user.clone()),
				Error::<T>::InsufficientFunds
			);

			// Insert project owner account
			let powner_info: PVoPOInfo<T::IPFSLength, BlockNumber<T>> = PVoPOInfo {
				documentation_ipfs: documentation_ipfs.clone(),
				penalty_level: 0,
				penalty_timeout: BlockNumber::<T>::from(0u32),
			};
			ProjectOwners::<T>::insert(user.clone(), powner_info);

			// Deposit event
			Self::deposit_event(Event::ProjectOwnerAccountRegistered(
				user.clone(),
				documentation_ipfs.clone(),
			));

			Ok(().into())
		}

		// Vote for/against Carbon Deficit Reports or for/against project Proposals
		#[pallet::call_index(5)]
		#[pallet::weight(0)]
		pub fn cast_vote(
			origin: OriginFor<T>,
			vote_type: VoteType,
			ipfs: BoundedString<T::IPFSLength>,
			vote: bool,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			// Check if caller is Project Validator account
			ensure!(ProjectValidators::<T>::contains_key(user.clone()), Error::<T>::Unauthorized);

			match vote_type {
				VoteType::CFReportVote => {
					// Get report info and return error if it does not exist
					let mut report =
						CFReports::<T>::get(ipfs.clone()).ok_or(Error::<T>::CFReportNotFound)?;

					// Check if the voting cycle is over
					ensure!(
						report.voting_active,
						Error::<T>::VotingCycleIsOver
					);

					// Check if vote already exists
					ensure!(
						!report.votes_for.contains(&user) && !report.votes_against.contains(&user),
						Error::<T>::VoteAlreadySubmitted
					);

					if vote {
						report.votes_for.insert(user.clone());
					} else {
						report.votes_against.insert(user.clone());
					};

					CFReports::<T>::insert(ipfs.clone(), report);
				},
				VoteType::PProposalVote => {
					// Get proposal info or return error if it does not exist
					let mut proposal = ProjectProposals::<T>::get(ipfs.clone())
						.ok_or(Error::<T>::ProjectProposalNotFound)?;

					// Check if the voting cycle is over
					ensure!(
						proposal.voting_active,
						Error::<T>::VotingCycleIsOver
					);

					// Check if vote already exists
					ensure!(
						!proposal.votes_for.contains(&user)
							&& !proposal.votes_against.contains(&user),
						Error::<T>::VoteAlreadySubmitted
					);

					if vote {
						proposal.votes_for.insert(user.clone());
					} else {
						proposal.votes_against.insert(user.clone());
					};

					ProjectProposals::<T>::insert(ipfs.clone(), proposal);
				},
				VoteType::CCBatchVote => {
					// Get carbon credit batch proposal info or return error if it does not exist
					let mut batch = CCBProposals::<T>::get(ipfs.clone())
						.ok_or(Error::<T>::CCBProposalNotFound)?;

					// Check if the voting cycle is over
					ensure!(
						batch.voting_active,
						Error::<T>::VotingCycleIsOver
					);

					// Check if vote already exists
					ensure!(
						!batch.votes_for.contains(&user) && !batch.votes_against.contains(&user),
						Error::<T>::VoteAlreadySubmitted
					);

					if vote {
						batch.votes_for.insert(user.clone());
					} else {
						batch.votes_against.insert(user.clone());
					};

					CCBProposals::<T>::insert(ipfs.clone(), batch);
				},
			}

			Self::deposit_event(Event::SuccessfulVote(user.clone(), ipfs.clone(), vote_type, vote));

			Ok(().into())
		}

		// Propose project
		#[pallet::call_index(6)]
		#[pallet::weight(0)]
		pub fn propose_project(
			origin: OriginFor<T>,
			ipfs: BoundedString<T::IPFSLength>,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			// Check if caller is Project Owner account
			ensure!(ProjectOwners::<T>::contains_key(user.clone()), Error::<T>::Unauthorized);

			// Ensure project does not exist
			ensure!(
				!ProjectProposals::<T>::contains_key(ipfs.clone()),
				Error::<T>::ProjectProposalAlreadyExists
			);

			// Check if the documentation (IPFS link) has been used previously
			ensure!(
				Self::is_ipfs_available(ipfs.clone()),
				Error::<T>::DocumentationWasUsedPreviously
			);

			// Get time
			let creation_date = T::Time::now();

			// Create project hash
			let nonce = frame_system::Pallet::<T>::account_nonce(&user);
			let encoded: [u8; 32] = (&user, nonce).using_encoded(blake2_256);
			let project_hash = H256::from(encoded);

			// Project Proposal info
			let project_proposal_info = ProjectProposalInfo {
				project_owner: user.clone(),
				creation_date,
				project_hash,
				votes_for: BTreeSet::<AccountIdOf<T>>::new(),
				votes_against: BTreeSet::<AccountIdOf<T>>::new(),
				voting_active: true,
			};

			// Write to info storage
			ProjectProposals::<T>::insert(ipfs.clone(), project_proposal_info);

			// Deposit event
			Self::deposit_event(Event::ProjectProposalCreated(user.clone(), ipfs));

			Ok(().into())
		}

		// Propose carbon credit batch
		#[pallet::call_index(7)]
		#[pallet::weight(0)]
		pub fn propose_carbon_credit_batch(
			origin: OriginFor<T>,
			project_hash: H256,
			credit_amount: i128,
			initial_credit_price: BalanceOf<T>,
			ipfs: BoundedString<T::IPFSLength>,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			// Check if caller is a Project Owner account
			ensure!(ProjectOwners::<T>::contains_key(user.clone()), Error::<T>::Unauthorized);

			// Check if project exists
			let project = Projects::<T>::get(project_hash).ok_or(Error::<T>::ProjectDoesntExist)?;

			// Check if the owner owns the mentioned project
			let project_proposal = ProjectProposals::<T>::get(project.documentation_ipfs).unwrap();
			ensure!(project_proposal.project_owner == user, Error::<T>::Unauthorized);

			// Check if the documentation (IPFS link) has been used previously
			ensure!(
				Self::is_ipfs_available(ipfs.clone()),
				Error::<T>::DocumentationWasUsedPreviously
			);

			// Create batch hash
			let nonce = frame_system::Pallet::<T>::account_nonce(&user);
			let encoded: [u8; 32] = (&user, nonce).using_encoded(blake2_256);
			let batch_hash = H256::from(encoded);

			// Get time
			let creation_date = T::Time::now();

			// CCB Proposal info
			let ccb_proposal_info = CCBProposalInfo {
				project_hash,
				batch_hash,
				creation_date,
				credit_amount,
				initial_credit_price,
				votes_for: BTreeSet::<AccountIdOf<T>>::new(),
				votes_against: BTreeSet::<AccountIdOf<T>>::new(),
				voting_active: true,
			};

			// Write to info storage
			CCBProposals::<T>::insert(ipfs.clone(), ccb_proposal_info);

			// Deposit event
			Self::deposit_event(Event::CarbonCreditBatchProposalCreated(user.clone(), ipfs));

			Ok(().into())
		}
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			match call {
				_ => InvalidTransaction::Call.into(),
			}
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(_now: BlockNumber<T>) {}
	}

	impl<T: Config> Pallet<T> {
		// Check if the documentation (ipfs link) has been used previously
		// Return false if the documentation is used
		// Return true if the documentation is available
		pub fn is_ipfs_available(ipfs: BoundedString<T::IPFSLength>) -> bool {
			// Check in reports and proposals
			if CFReports::<T>::contains_key(ipfs.clone())
				|| ProjectProposals::<T>::contains_key(ipfs.clone())
				|| CCBProposals::<T>::contains_key(ipfs.clone())
			{
				return false;
			}

			// Check in CF accounts
			for (_, cfa_info) in <CarbonFootprintAccounts<T>>::iter() {
				if cfa_info.documentation_ipfs == ipfs {
					return false;
				}
			}

			// Check in validators
			for (_, cfa_info) in <ProjectValidators<T>>::iter() {
				if cfa_info.documentation_ipfs == ipfs {
					return false;
				}
			}

			// Check in project owners
			for (_, cfa_info) in <ProjectOwners<T>>::iter() {
				if cfa_info.documentation_ipfs == ipfs {
					return false;
				}
			}

			return true;
		}

		// Check if the account is tied to any existing entity on the pallet
		// Return false if the account_id is used
		// Return true if the account_id is available
		pub fn is_account_id_available(account_id: AccountIdOf<T>) -> bool {
			// Check in carbon footprint, trader, project validator and project owner accounts
			if CarbonFootprintAccounts::<T>::contains_key(account_id.clone())
				|| TraderAccounts::<T>::get().contains(&account_id.clone())
				|| ProjectValidators::<T>::contains_key(account_id.clone())
				|| ProjectOwners::<T>::contains_key(account_id.clone())
			{
				return false;
			}

			return true;
		}
	}
}
