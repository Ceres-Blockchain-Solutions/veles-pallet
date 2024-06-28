#![cfg_attr(not(feature = "std"), no_std)]

pub use codec::{Decode, Encode, MaxEncodedLen};
pub use common::BoundedString;
pub use frame_support::pallet_prelude::Get;
pub use frame_support::sp_runtime::traits::AccountIdConversion;
pub use frame_support::traits::Currency;
pub use frame_support::traits::ExistenceRequirement;
pub use pallet::*;
pub use sp_core::{blake2_256, H256};
pub use sp_std::collections::btree_set::BTreeSet;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Global data structures
// Project Validator / Project Owner data structure
#[derive(Encode, Decode, Default, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
#[scale_info(skip_type_params(IPFSLength))]
pub struct ProjectValidatorOrProjectOwnerInfo<IPFSLength: Get<u32>, BlockNumber> {
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
pub struct CarbonFootprintAccountInfo<MomentOf, IPFSLength: Get<u32>> {
	// IPFS links to CFA documentation
	documentation_ipfses: BTreeSet<BoundedString<IPFSLength>>,
	// Carbon credit balance
	carbon_credit_balance: i128,
	// Creation date
	creation_date: MomentOf,
}

// Carbon Footprint report data structure
#[derive(Encode, Decode, Default, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct CarbonFootprintReportInfo<AccountIdOf, MomentOf> {
	// Carbon footprint account
	cf_account: AccountIdOf,
	// Creation date
	creation_date: MomentOf,
	// Carbon balance (aka Carbon footprint)
	carbon_balance: i128,
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
pub struct CarbonCreditBatchProposalInfo<MomentOf, BalanceOf, AccountIdOf> {
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
pub struct CarbonCreditBatchInfo<IPFSLength: Get<u32>, MomentOf, BalanceOf, VoteType> {
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
	pub level: u8, 	// Penalty level
	pub base: i32, 	// Balance
}

// Vote type enum
#[derive(Encode, Decode, PartialEq, Eq, scale_info::TypeInfo, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum VoteType {
	CarbonFootprintReportVote,
	ProjectProposalVote,
	CarbonCreditBatchVote,
}

// Carbon credit batch status
#[derive(Encode, Decode, PartialEq, Eq, scale_info::TypeInfo, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum CarbonCreditBatchStatus {
	Active,   	// Tokens can be traded and retired
	Frozen,   	// Tokens can't be traded or retired
	Redacted, 	// Tokens have been removed from circulation
}

// Fee types
#[derive(Encode, Decode, PartialEq, Eq, scale_info::TypeInfo, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum FeeType {
	TraderAccountFee,           	// Trader acccount registration fee
	ProjectValidatorAccountFee, 	// Project validator account registration fee
	ProjectOwnerAccountFee,     	// Project owner account registration fee
	CarbonCreditReportFee,      	// Carbon credit report submition fee
	ProjectProposalFee,         	// Project proposal fee
	CarbonCreditBatchFee,       	// Carbon credit batch proposition fee
	VotingFee,                  	// Voting fee
	ClaimFee,                   	// Claim proposal fee
}

// Fee values
#[derive(Encode, Decode, PartialEq, Eq, scale_info::TypeInfo, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct FeeValues<BalanceOf> {
	// Trader acccount registration fee
	trader_account_fee: BalanceOf,
	// Project validator account registration fee
	project_validator_account_fee: BalanceOf,
	// Project owner account registration fee
	project_owner_account_fee: BalanceOf,
	// Carbon footprint report submition fee
	carbon_footprint_report_fee: BalanceOf,
	// Project proposal fee 
	project_proposal_fee: BalanceOf, 
	// Carbon credit batch proposition fee
	carbon_credit_batch_fee: BalanceOf,
	// Voting fee
	voting_fee: BalanceOf,
	// Claim proposal fee     
	claim_fee: BalanceOf,          
}

// Time type
#[derive(Encode, Decode, PartialEq, Eq, scale_info::TypeInfo, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum TimeType {
	NumberOfBlocksYearly,
	PalletBaseTime,
	PenaltyTimeout,
	VotingTimeout,
	SalesTimeout,
}

// Time values (in blocks)
#[derive(Encode, Decode, PartialEq, Eq, scale_info::TypeInfo, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct TimeValues<BlockNumber> {
	number_of_blocks_per_year: BlockNumber,
	pallet_base_time: BlockNumber,
	penalty_timeout: BlockNumber,
	voting_timeout: BlockNumber,
	sales_timeout: BlockNumber,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_support::traits::ReservableCurrency;
	use frame_support::traits::Time;
	use frame_support::PalletId;
	use frame_system::offchain::{SendTransactionTypes, SubmitTransaction};
	use frame_system::pallet_prelude::*;
	use log::{info, warn};
	use sp_std::collections::btree_set::BTreeSet;

	const PALLET_ID: PalletId = PalletId(*b"velesplt");

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Pallet configuration
	#[pallet::config]
	pub trait Config: frame_system::Config + SendTransactionTypes<Call<Self>> {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type IPFSLength: Get<u32>;
		type BlockFinalizationTime: Get<u32>;
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

	// Default values for all fees
	#[pallet::type_value]
	pub fn DefaultForPalletFeeValues<T: Config>() -> FeeValues<BalanceOf<T>> {
		let fee_values: FeeValues<BalanceOf<T>> = FeeValues {
			trader_account_fee: BalanceOf::<T>::from(100u32),
			project_validator_account_fee: BalanceOf::<T>::from(100u32),
			project_owner_account_fee: BalanceOf::<T>::from(100u32),
			carbon_footprint_report_fee: BalanceOf::<T>::from(300u32),
			project_proposal_fee: BalanceOf::<T>::from(100u32),
			carbon_credit_batch_fee: BalanceOf::<T>::from(50u32),
			voting_fee: BalanceOf::<T>::from(100u32),
			claim_fee: BalanceOf::<T>::from(100u32),
		};

		fee_values
	}

	// Default values for all timeouts/block values
	#[pallet::type_value]
	pub fn DefaultForPalletTimeValues<T: Config>() -> TimeValues<BlockNumber<T>> {
		let block_finalization_time: u32 = T::BlockFinalizationTime::get().into();

		let seconds_in_year: u32 = 365 * 24 * 60 * 60;
		let seconds_in_month: u32 = 31 * 24 * 60 * 60;
		let seconds_in_week: u32 = 7 * 24 * 60 * 60;

		let blocks_in_year = seconds_in_year / block_finalization_time;
		let blocks_in_month = seconds_in_month / block_finalization_time;
		let blocks_in_week = seconds_in_week / block_finalization_time;

		let pallet_time_values: TimeValues<BlockNumber<T>> = TimeValues {
			number_of_blocks_per_year: BlockNumber::<T>::from(blocks_in_year),
			pallet_base_time: BlockNumber::<T>::from(0u32),
			penalty_timeout: BlockNumber::<T>::from(blocks_in_month),
			voting_timeout: BlockNumber::<T>::from(blocks_in_week),
			sales_timeout: BlockNumber::<T>::from(blocks_in_week),
		};

		pallet_time_values
	}

	/// Pallet storages
	// Fee values
	#[pallet::storage]
	#[pallet::getter(fn pallet_fee_values)]
	pub type PalletFeeValues<T: Config> =
		StorageValue<_, FeeValues<BalanceOf<T>>, ValueQuery, DefaultForPalletFeeValues<T>>;

	// Time values
	#[pallet::storage]
	#[pallet::getter(fn pallet_time_values)]
	pub type PalletTimeValues<T: Config> =
		StorageValue<_, TimeValues<BlockNumber<T>>, ValueQuery, DefaultForPalletTimeValues<T>>;

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
		CarbonFootprintAccountInfo<MomentOf<T>, T::IPFSLength>,
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
		ProjectValidatorOrProjectOwnerInfo<T::IPFSLength, BlockNumber<T>>,
		OptionQuery,
	>;

	// Project Owner accounts
	#[pallet::storage]
	#[pallet::getter(fn project_owners)]
	pub(super) type ProjectOwners<T: Config> = StorageMap<
		_,
		Identity,
		AccountIdOf<T>,
		ProjectValidatorOrProjectOwnerInfo<T::IPFSLength, BlockNumber<T>>,
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
		CarbonFootprintReportInfo<AccountIdOf<T>, MomentOf<T>>,
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
		CarbonCreditBatchProposalInfo<MomentOf<T>, BalanceOf<T>, AccountIdOf<T>>,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Time value updated
		TimeValueUpdated(TimeType, BlockNumber<T>),
		/// Fee value updated
		FeeValueUpdated(FeeType, BalanceOf<T>),
		/// Trader Account Registered
		TraderAccountRegistered(AccountIdOf<T>),
		/// Project Validator Account Registered
		ProjectValidatorAccountRegistered(AccountIdOf<T>, BoundedString<T::IPFSLength>),
		/// Project Owner Account Registered
		ProjectOwnerAccountRegistered(AccountIdOf<T>, BoundedString<T::IPFSLength>),
		/// Successful Vote Cast
		SuccessfulVote(AccountIdOf<T>, BoundedString<T::IPFSLength>, VoteType, bool),
		/// Carbon Footprint Report Submitted
		CarbonFootprintReportSubmitted(AccountIdOf<T>, BoundedString<T::IPFSLength>),
		/// Successful Project Proposal Created
		ProjectProposalCreated(AccountIdOf<T>, BoundedString<T::IPFSLength>),
		/// Carbon Credit Batch Proposal Created
		CarbonCreditBatchProposalCreated(AccountIdOf<T>, BoundedString<T::IPFSLength>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Unable to change pallet base time
		UnableToChangePalletBaseTime,
		/// UpdatingToCurrentValue
		UpdatingToCurrentValue,
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
		/// Account ID already in use
		AccountIdAlreadyInUse,
		/// Already submitted a CF report
		CFReportAlreadySubmitted,
		/// User is active in CF report voting cycle
		UserIsActiveInCFReportVotingCycle,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Update specific time value
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn update_time_value(
			origin: OriginFor<T>,
			time_type: TimeType,
			new_time_value: BlockNumber<T>,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			// Check if caller is a Authority account
			ensure!(
				AuthorityAccounts::<T>::get().contains(&user.clone()),
				Error::<T>::Unauthorized
			);

			// Check if the user is trying to update the pallet base time
			ensure!(
				time_type != TimeType::PalletBaseTime,
				Error::<T>::UnableToChangePalletBaseTime
			);

			// Check if the new time value is not 0
			ensure!(
				new_time_value != BlockNumber::<T>::from(0u32),
				Error::<T>::InvalidTimeoutValue
			);

			let mut pallet_times = PalletTimeValues::<T>::get();

			match time_type {
				TimeType::NumberOfBlocksYearly => {
					ensure!(
						new_time_value != pallet_times.number_of_blocks_per_year,
						Error::<T>::UpdatingToCurrentValue,
					);

					pallet_times =
						TimeValues { number_of_blocks_per_year: new_time_value, ..pallet_times };
				},
				TimeType::PenaltyTimeout => {
					ensure!(
						new_time_value != pallet_times.penalty_timeout,
						Error::<T>::UpdatingToCurrentValue,
					);

					pallet_times = TimeValues { penalty_timeout: new_time_value, ..pallet_times };
				},
				TimeType::VotingTimeout => {
					ensure!(
						new_time_value != pallet_times.voting_timeout,
						Error::<T>::UpdatingToCurrentValue,
					);

					pallet_times = TimeValues { voting_timeout: new_time_value, ..pallet_times };
				},
				TimeType::SalesTimeout => {
					ensure!(
						new_time_value != pallet_times.sales_timeout,
						Error::<T>::UpdatingToCurrentValue,
					);

					pallet_times = TimeValues { sales_timeout: new_time_value, ..pallet_times };
				},
				_ => {},
			}

			PalletTimeValues::<T>::set(pallet_times);

			Self::deposit_event(Event::TimeValueUpdated(time_type, new_time_value));

			Ok(().into())
		}

		// Update specific fee value
		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn update_fee_value(
			origin: OriginFor<T>,
			fee_type: FeeType,
			new_fee_value: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			// Check if caller is a Authority account
			ensure!(
				AuthorityAccounts::<T>::get().contains(&user.clone()),
				Error::<T>::Unauthorized
			);

			let mut pallet_fees = PalletFeeValues::<T>::get();

			match fee_type {
				FeeType::TraderAccountFee => {
					ensure!(
						new_fee_value != pallet_fees.trader_account_fee,
						Error::<T>::UpdatingToCurrentValue,
					);

					pallet_fees = FeeValues { trader_account_fee: new_fee_value, ..pallet_fees };
				},
				FeeType::ProjectValidatorAccountFee => {
					ensure!(
						new_fee_value != pallet_fees.project_validator_account_fee,
						Error::<T>::UpdatingToCurrentValue,
					);

					pallet_fees =
						FeeValues { project_validator_account_fee: new_fee_value, ..pallet_fees };
				},
				FeeType::ProjectOwnerAccountFee => {
					ensure!(
						new_fee_value != pallet_fees.project_owner_account_fee,
						Error::<T>::UpdatingToCurrentValue,
					);

					pallet_fees =
						FeeValues { project_owner_account_fee: new_fee_value, ..pallet_fees };
				},
				FeeType::CarbonCreditReportFee => {
					ensure!(
						new_fee_value != pallet_fees.carbon_footprint_report_fee,
						Error::<T>::UpdatingToCurrentValue,
					);

					pallet_fees =
						FeeValues { carbon_footprint_report_fee: new_fee_value, ..pallet_fees };
				},
				FeeType::ProjectProposalFee => {
					ensure!(
						new_fee_value != pallet_fees.project_proposal_fee,
						Error::<T>::UpdatingToCurrentValue,
					);

					pallet_fees = FeeValues { project_proposal_fee: new_fee_value, ..pallet_fees };
				},
				FeeType::CarbonCreditBatchFee => {
					ensure!(
						new_fee_value != pallet_fees.carbon_credit_batch_fee,
						Error::<T>::UpdatingToCurrentValue,
					);

					pallet_fees =
						FeeValues { carbon_credit_batch_fee: new_fee_value, ..pallet_fees };
				},
				FeeType::VotingFee => {
					ensure!(
						new_fee_value != pallet_fees.voting_fee,
						Error::<T>::UpdatingToCurrentValue,
					);

					pallet_fees = FeeValues { voting_fee: new_fee_value, ..pallet_fees };
				},
				FeeType::ClaimFee => {
					ensure!(
						new_fee_value != pallet_fees.claim_fee,
						Error::<T>::UpdatingToCurrentValue,
					);

					pallet_fees = FeeValues { claim_fee: new_fee_value, ..pallet_fees };
				},
			}

			PalletFeeValues::<T>::set(pallet_fees);

			Self::deposit_event(Event::FeeValueUpdated(fee_type, new_fee_value));

			Ok(().into())
		}

		// Register for a trader account
		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn register_for_trader_account(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			// Check if the account is in use
			ensure!(Self::is_account_id_available(user.clone()), Error::<T>::AccountIdAlreadyInUse);

			// Check if the user is active in a CF report voting cycle
			ensure!(
				!Self::is_trying_to_register_as_cfa(user.clone()),
				Error::<T>::UserIsActiveInCFReportVotingCycle
			);

			// Check if caller has sufficient funds
			ensure!(
				PalletFeeValues::<T>::get().trader_account_fee
					<= T::Currency::free_balance(&user.clone()),
				Error::<T>::InsufficientFunds
			);

			// Insert trader account
			let mut new_traders = TraderAccounts::<T>::get();
			new_traders.insert(user.clone());
			TraderAccounts::<T>::set(new_traders);

			// Transfer funds
			T::Currency::transfer(
				&Self::account_id(),
				&user,
				PalletFeeValues::<T>::get().trader_account_fee,
				ExistenceRequirement::KeepAlive,
			)?;

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

			// Check if the account is in use
			ensure!(Self::is_account_id_available(user.clone()), Error::<T>::AccountIdAlreadyInUse);

			// Check if the user is active in a CF report voting cycle
			ensure!(
				!Self::is_trying_to_register_as_cfa(user.clone()),
				Error::<T>::UserIsActiveInCFReportVotingCycle
			);

			// Check if the documentation (IPFS link) has been used previously
			ensure!(
				Self::is_ipfs_available(documentation_ipfs.clone()),
				Error::<T>::DocumentationWasUsedPreviously
			);

			// Check if caller has sufficient funds
			ensure!(
				PalletFeeValues::<T>::get().project_validator_account_fee
					<= T::Currency::free_balance(&user.clone()),
				Error::<T>::InsufficientFunds
			);

			// Insert project validator account
			let pvalidator_info: ProjectValidatorOrProjectOwnerInfo<T::IPFSLength, BlockNumber<T>> =
				ProjectValidatorOrProjectOwnerInfo {
					documentation_ipfs: documentation_ipfs.clone(),
					penalty_level: 0,
					penalty_timeout: BlockNumber::<T>::from(0u32),
				};
			ProjectValidators::<T>::insert(user.clone(), pvalidator_info);

			// Transfer funds
			T::Currency::transfer(
				&Self::account_id(),
				&user,
				PalletFeeValues::<T>::get().project_validator_account_fee,
				ExistenceRequirement::KeepAlive,
			)?;

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

			// Check if the account is in use
			ensure!(Self::is_account_id_available(user.clone()), Error::<T>::AccountIdAlreadyInUse);

			// Check if the user is active in a CF report voting cycle
			ensure!(
				!Self::is_trying_to_register_as_cfa(user.clone()),
				Error::<T>::UserIsActiveInCFReportVotingCycle
			);

			// Check if the documentation (IPFS link) has been used previously
			ensure!(
				Self::is_ipfs_available(documentation_ipfs.clone()),
				Error::<T>::DocumentationWasUsedPreviously
			);

			// Check if caller has sufficient funds
			ensure!(
				PalletFeeValues::<T>::get().project_owner_account_fee
					<= T::Currency::free_balance(&user.clone()),
				Error::<T>::InsufficientFunds
			);

			// Insert project owner account
			let powner_info: ProjectValidatorOrProjectOwnerInfo<T::IPFSLength, BlockNumber<T>> =
				ProjectValidatorOrProjectOwnerInfo {
					documentation_ipfs: documentation_ipfs.clone(),
					penalty_level: 0,
					penalty_timeout: BlockNumber::<T>::from(0u32),
				};
			ProjectOwners::<T>::insert(user.clone(), powner_info);

			// Transfer funds
			T::Currency::transfer(
				&Self::account_id(),
				&user,
				PalletFeeValues::<T>::get().project_owner_account_fee,
				ExistenceRequirement::KeepAlive,
			)?;

			// Deposit event
			Self::deposit_event(Event::ProjectOwnerAccountRegistered(
				user.clone(),
				documentation_ipfs.clone(),
			));

			Ok(().into())
		}

		// Submit carbon footprint report
		#[pallet::call_index(6)]
		#[pallet::weight(0)]
		pub fn submit_cfreport(
			origin: OriginFor<T>,
			ipfs: BoundedString<T::IPFSLength>,
			carbon_balance: i128,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			// Check if the account is a CFAccount
			// Note: An accountID value can be already in use for as a CF account
			// 		 But it can also be available if the user is a new user of the pallet
			ensure!(Self::is_eligible_for_cfa(user.clone()), Error::<T>::AccountIdAlreadyInUse);

			// Check if the documentation (IPFS link) has been used previously
			ensure!(
				Self::is_ipfs_available(ipfs.clone()),
				Error::<T>::DocumentationWasUsedPreviously
			);

			// Check if the user has already submited a CF report
			ensure!(
				!Self::is_trying_to_register_as_cfa(user.clone()),
				Error::<T>::CFReportAlreadySubmitted
			);

			// Check if caller has sufficient funds
			ensure!(
				PalletFeeValues::<T>::get().carbon_footprint_report_fee
					<= T::Currency::free_balance(&user.clone()),
				Error::<T>::InsufficientFunds
			);

			// Get time
			let creation_date = T::Time::now();

			// Carbon footprint info
			let cf_report_info = CarbonFootprintReportInfo {
				cf_account: user.clone(),
				creation_date,
				carbon_balance,
				votes_for: BTreeSet::<AccountIdOf<T>>::new(),
				votes_against: BTreeSet::<AccountIdOf<T>>::new(),
				voting_active: true,
			};

			// Write to info storage
			CFReports::<T>::insert(ipfs.clone(), cf_report_info);

			// Set for voting timeout
			let current_block = frame_system::Pallet::<T>::block_number(); // Get current block number
			let timeout_block = current_block + PalletTimeValues::<T>::get().voting_timeout; // Calculate voting timeout time

			let mut timeout_events = BTreeSet::<BoundedString<T::IPFSLength>>::new();

			if VotingTimeouts::<T>::contains_key(timeout_block) {
				timeout_events = VotingTimeouts::<T>::get(timeout_block).unwrap();
			}

			timeout_events.insert(ipfs.clone());

			VotingTimeouts::<T>::insert(timeout_block, timeout_events);

			// Transfer funds
			T::Currency::transfer(
				&Self::account_id(),
				&user,
				PalletFeeValues::<T>::get().carbon_footprint_report_fee,
				ExistenceRequirement::KeepAlive,
			)?;

			// Deposit event
			Self::deposit_event(Event::CarbonFootprintReportSubmitted(user.clone(), ipfs));

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

			// Check if caller has sufficient funds
			ensure!(
				PalletFeeValues::<T>::get().voting_fee
					<= T::Currency::free_balance(&user.clone()),
				Error::<T>::InsufficientFunds
			);

			match vote_type {
				VoteType::CarbonFootprintReportVote => {
					// Get report info and return error if it does not exist
					let mut report =
						CFReports::<T>::get(ipfs.clone()).ok_or(Error::<T>::CFReportNotFound)?;

					// Check if the voting cycle is over
					ensure!(report.voting_active, Error::<T>::VotingCycleIsOver);

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
				VoteType::ProjectProposalVote => {
					// Get proposal info or return error if it does not exist
					let mut proposal = ProjectProposals::<T>::get(ipfs.clone())
						.ok_or(Error::<T>::ProjectProposalNotFound)?;

					// Check if the voting cycle is over
					ensure!(proposal.voting_active, Error::<T>::VotingCycleIsOver);

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
				VoteType::CarbonCreditBatchVote => {
					// Get carbon credit batch proposal info or return error if it does not exist
					let mut batch = CCBProposals::<T>::get(ipfs.clone())
						.ok_or(Error::<T>::CCBProposalNotFound)?;

					// Check if the voting cycle is over
					ensure!(batch.voting_active, Error::<T>::VotingCycleIsOver);

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

			// Transfer funds
			T::Currency::transfer(
				&Self::account_id(),
				&user,
				PalletFeeValues::<T>::get().voting_fee,
				ExistenceRequirement::KeepAlive,
			)?;

			Self::deposit_event(Event::SuccessfulVote(user.clone(), ipfs.clone(), vote_type, vote));

			Ok(().into())
		}

		// Propose project
		#[pallet::call_index(7)]
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

			// Check if caller has sufficient funds
			ensure!(
				PalletFeeValues::<T>::get().project_proposal_fee
					<= T::Currency::free_balance(&user.clone()),
				Error::<T>::InsufficientFunds
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

			// Set for voting timeout
			let current_block = frame_system::Pallet::<T>::block_number(); // Get current block number
			let timeout_block = current_block + PalletTimeValues::<T>::get().voting_timeout; // Calculate voting timeout time

			let mut timeout_events = BTreeSet::<BoundedString<T::IPFSLength>>::new();

			if VotingTimeouts::<T>::contains_key(timeout_block) {
				timeout_events = VotingTimeouts::<T>::get(timeout_block).unwrap();
			}

			timeout_events.insert(ipfs.clone());

			VotingTimeouts::<T>::insert(timeout_block, timeout_events);

			// Transfer funds
			T::Currency::transfer(
				&Self::account_id(),
				&user,
				PalletFeeValues::<T>::get().project_proposal_fee,
				ExistenceRequirement::KeepAlive,
			)?;


			// Deposit event
			Self::deposit_event(Event::ProjectProposalCreated(user.clone(), ipfs));

			Ok(().into())
		}

		// Propose carbon credit batch
		#[pallet::call_index(8)]
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

			// Check if caller has sufficient funds
			ensure!(
				PalletFeeValues::<T>::get().carbon_credit_batch_fee
					<= T::Currency::free_balance(&user.clone()),
				Error::<T>::InsufficientFunds
			);

			// Create batch hash
			let nonce = frame_system::Pallet::<T>::account_nonce(&user);
			let encoded: [u8; 32] = (&user, nonce).using_encoded(blake2_256);
			let batch_hash = H256::from(encoded);

			// Get time
			let creation_date = T::Time::now();

			// CCB Proposal info
			let ccb_proposal_info = CarbonCreditBatchProposalInfo {
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

			// Set for voting timeout
			let current_block = frame_system::Pallet::<T>::block_number(); // Get current block number
			let timeout_block = current_block + PalletTimeValues::<T>::get().voting_timeout; // Calculate voting timeout time

			let mut timeout_events = BTreeSet::<BoundedString<T::IPFSLength>>::new();

			if VotingTimeouts::<T>::contains_key(timeout_block) {
				timeout_events = VotingTimeouts::<T>::get(timeout_block).unwrap();
			}

			timeout_events.insert(ipfs.clone());

			VotingTimeouts::<T>::insert(timeout_block, timeout_events);

			// Transfer funds
			T::Currency::transfer(
				&Self::account_id(),
				&user,
				PalletFeeValues::<T>::get().carbon_credit_batch_fee,
				ExistenceRequirement::KeepAlive,
			)?;

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
				// Call::update_base_pallet_time { new_base_pallet_time } => {
				// 	ValidTransaction::with_tag_prefix("Veles::update_base_pallet_time")
				// 		.priority(T::UnsignedPriority::get())
				// 		.longevity(T::UnsignedLongevity::get())
				// 		.and_provides([new_base_pallet_time])
				// 		.propagate(true)
				// 		.build()
				// },
				_ => {
					warn!("Unknown unsigned call {:?}", call);
					InvalidTransaction::Call.into()
				},
			}
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(now: BlockNumber<T>) -> Weight {
			let consumed_weight = Self::update_pallet_base_time(now);

			consumed_weight
		}

		// fn offchain_worker(now: BlockNumber<T>) {

		// 	let base_pallet_time = BasePalletTime::<T>::get();

		// 	// Check if a year has passed and update pallet base time if it has
		// 	// Note: The base pallet time will update once a year has passed (in blocks)
		// 	if base_pallet_time == 0u32.into() || now > base_pallet_time + T::NumberOfBlocksYearly::get().into() {
		// 		info!("👷 Offchain worker: Running base pallet time check");

		// 		let call = Call::<T>::update_base_pallet_time { new_base_pallet_time: now.clone() };

		// 		if let Err(err) = SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into()) {
		// 			warn!("👷 Offchain worker: Failed to liquidate user position.🚧 Error: {:?}", err);
		// 		} else {
		// 			info!("👷 Offchain worker: Updated base pallet time");
		// 		}
		// 	}
		// }
	}

	impl<T: Config> Pallet<T> {
		// Get account ID of pallet
		fn account_id() -> T::AccountId {
			PALLET_ID.into_account_truncating()
		}

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
				let documentation = cfa_info.documentation_ipfses;

				if documentation.contains(&ipfs) {
					return false;
				}
			}

			// Check in validators
			for (_, validator) in <ProjectValidators<T>>::iter() {
				if validator.documentation_ipfs == ipfs {
					return false;
				}
			}

			// Check in project owners
			for (_, project_owner) in <ProjectOwners<T>>::iter() {
				if project_owner.documentation_ipfs == ipfs {
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

		// Check if the account is eligible to be a carbon footpring account
		// Return true if the account_id isn't in use for another account type
		// Return false if the account_id is in use for another account type
		pub fn is_eligible_for_cfa(account_id: AccountIdOf<T>) -> bool {
			// Check in carbon footprint, trader, project validator and project owner accounts
			if TraderAccounts::<T>::get().contains(&account_id.clone())
				|| ProjectValidators::<T>::contains_key(account_id.clone())
				|| ProjectOwners::<T>::contains_key(account_id.clone())
			{
				return false;
			}

			return true;
		}

		// Check if the account has submitted a carbon footprint report for voting
		// Note: If a user has submitted a CF report he/she then can not registed as another account type
		pub fn is_trying_to_register_as_cfa(account_id: AccountIdOf<T>) -> bool {
			// Check in CF accounts
			for (_, cf_report) in <CFReports<T>>::iter() {
				// Check if user has an active CF report that is up for voting
				if cf_report.cf_account == account_id && cf_report.voting_active {
					return true;
				}
			}

			return false;
		}

		// Check if a year has passed and update pallet base time if it has
		// Note: The base pallet time will update once a year has passed (in blocks)
		pub fn update_pallet_base_time(now: BlockNumber<T>) -> Weight {
			let mut counter: u64 = 0;

			let mut pallet_times = PalletTimeValues::<T>::get();

			if pallet_times.pallet_base_time == BlockNumber::<T>::from(0u32)
				|| now == pallet_times.pallet_base_time + pallet_times.number_of_blocks_per_year
			{
				pallet_times = TimeValues { pallet_base_time: now, ..pallet_times };

				PalletTimeValues::<T>::set(pallet_times);

				counter += 1;
			}

			T::DbWeight::get()
				.reads(counter)
				.saturating_add(T::DbWeight::get().writes(counter))
		}
	}
}
