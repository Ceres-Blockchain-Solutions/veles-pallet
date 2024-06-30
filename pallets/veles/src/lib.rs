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
	// Carbon footprint balance
	carbon_footprint_balance: i128,
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
	// Carbon footprint balance (aka Carbon footprint)
	carbon_footprint_balance: i128,
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
	credit_amount: BalanceOf,
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
pub struct ProjectInfo<IPFSLength: Get<u32>, AccountIdOf, MomentOf, BlockNumber> {
	// IPFS link to project documentation
	documentation_ipfs: BoundedString<IPFSLength>,
	// Project owner
	project_owner: AccountIdOf,
	// Creation date
	creation_date: MomentOf,
	// Penalty level
	penalty_level: u8,
	// Penalty timeout
	penalty_timeout: BlockNumber,
}

// Carbon credit batch info structure
#[derive(Encode, Decode, Clone, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
#[scale_info(skip_type_params(IPFSLength))]
pub struct CarbonCreditBatchInfo<IPFSLength: Get<u32>, MomentOf, BalanceOf, CarbonCreditBatchStatus>
{
	// IPFS link to CFA documentation
	documentation_ipfs: BoundedString<IPFSLength>,
	// Creation date
	creation_date: MomentOf,
	// Carbon credit amount
	credit_amount: BalanceOf,
	// Initial carbon credit price
	initial_credit_price: BalanceOf,
	// Batch status
	status: CarbonCreditBatchStatus,
}

// Carbon credit holding info structure
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct CarbonCreditHoldingsInfo<BalanceOf> {
	// Amount of available tokens for sale and retirment
	available_amount: BalanceOf,
	// Amount of unavailable tokens that are currently in a sales cycle
	unavailable_amount: BalanceOf,
}

// Carbon credit sale info structure
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct CarbonCreditSaleOrderInfo<BalanceOf, AccountIdOf, BlockNumber> {
	// Carbon credit batch hash
	batch_hash: H256,
	// Amount of credit being sold
	credit_amount: BalanceOf,
	// Price of 1 credit
	credit_price: BalanceOf,
	// Seller account ID
	seller: AccountIdOf,
	// Buyer account ID
	buyer: AccountIdOf,
	// Sale status
	sale_active: bool,
	// Sale timeout
	sale_timeout: BlockNumber,
}

// Penalty level structure for carbon footprint
#[derive(Encode, Decode, Default, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct PenaltyLevelConfig {
	pub level: u8, // Penalty level
	pub base: i32, // Balance
}

// Proportion structure (used for vote ratio calculations)
#[derive(Encode, Decode, Default, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ProportionStructure {
	// Explanation:
	// Let's consider that we have the 3/4 proportion, we will say that the upper limit part of this
	// proportion is the number 4 while the proportion part is the number 3.
	//
	// Example:
	// Let's say that we need 3/4 of the total number of votes in order for a proposal to pass, this means
	// that we need 75% (0.75) vote in order for our vote to pass. Let's now say that there was a total of a 100
	// votes. In order to calculate this proportion without using a float point number for the proportion, we can
	// first multiply the total votes by the proportion part of our proportion (100 * 3 = 300) and then devide it by
	// the upper limit part of out proportion (300 / 4 = 75) which will give us the exact proportion that we need
	pub proportion_part: u16,
	pub upper_limit_part: u16,
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
	Active,   // Tokens can be traded and retired
	Frozen,   // Tokens can't be traded or retired
	Redacted, // Tokens have been removed from circulation
}

// Fee types
#[derive(Encode, Decode, PartialEq, Eq, scale_info::TypeInfo, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum FeeType {
	TraderAccountFee,           // Trader acccount registration fee
	ProjectValidatorAccountFee, // Project validator account registration fee
	ProjectOwnerAccountFee,     // Project owner account registration fee
	CarbonCreditReportFee,      // Carbon credit report submition fee
	ProjectProposalFee,         // Project proposal fee
	CarbonCreditBatchFee,       // Carbon credit batch proposition fee
	VotingFee,                  // Voting fee
	ClaimFee,                   // Claim proposal fee
}

// Fee values
#[derive(Encode, Decode, Default, PartialEq, Eq, scale_info::TypeInfo, Clone)]
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
#[derive(Encode, Decode, Default, PartialEq, Eq, scale_info::TypeInfo, Clone)]
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
	pub type MomentOf<T> = <<T as Config>::Time as Time>::Moment;
	pub type BlockNumber<T> = BlockNumberFor<T>;
	pub type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

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

	// Default value for voting ratio needed for a vote to pass
	// Note: If the upper_limit_part is set to 0 then we will consider that we only need a
	// 1 vote difference to decide what is the outcome of the vote. If we set the proportion_part to
	// a number that is equal to the upper_limit_part or that is greater then it then we will need
	// a 100% of votes in order to made a passing/non passing decision
	#[pallet::type_value]
	pub fn DefaultForVotePassRatio<T: Config>() -> ProportionStructure {
		let mut proportion_part: u16 = 1;
		let upper_limit_part: u16 = 6;

		if upper_limit_part == 0 {
			proportion_part = 0;
		} else if proportion_part >= upper_limit_part {
			proportion_part = upper_limit_part;
		}

		let pass_voting_ratio = ProportionStructure { proportion_part, upper_limit_part };

		pass_voting_ratio
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

	// Pass voting ratio
	#[pallet::storage]
	#[pallet::getter(fn vote_pass_ratio)]
	pub type VotePassRatio<T: Config> =
		StorageValue<_, ProportionStructure, ValueQuery, DefaultForVotePassRatio<T>>;

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
		ProjectInfo<T::IPFSLength, AccountIdOf<T>, MomentOf<T>, BlockNumber<T>>,
		OptionQuery,
	>;

	// Carbon credit batches
	#[pallet::storage]
	#[pallet::getter(fn carbon_credit_batches)]
	pub(super) type CarbonCreditBatches<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		H256,
		Blake2_128Concat,
		H256,
		CarbonCreditBatchInfo<T::IPFSLength, MomentOf<T>, BalanceOf<T>, CarbonCreditBatchStatus>,
		OptionQuery,
	>;

	// Carbon credit holdings
	#[pallet::storage]
	#[pallet::getter(fn carbon_credit_holdings)]
	pub(super) type CarbonCreditHoldings<T: Config> = StorageDoubleMap<
		_,
		Identity,
		H256,
		Identity,
		AccountIdOf<T>,
		CarbonCreditHoldingsInfo<BalanceOf<T>>,
		OptionQuery,
	>;

	// Carbon credit sale orders
	#[pallet::storage]
	#[pallet::getter(fn carbon_credit_sale_orders)]
	pub(super) type CarbonCreditSaleOrders<T: Config> = StorageMap<
		_,
		Identity,
		H256,
		CarbonCreditSaleOrderInfo<BalanceOf<T>, AccountIdOf<T>, BlockNumber<T>>,
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
	pub(super) type SaleOrderTimeouts<T: Config> =
		StorageMap<_, Identity, BlockNumber<T>, BTreeSet<H256>, OptionQuery>;

	// Carbon footprint reports
	#[pallet::storage]
	#[pallet::getter(fn carbon_footprint_reports)]
	pub(super) type CarbonFootprintReports<T: Config> = StorageMap<
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
	pub(super) type CarbonCreditBatchProposals<T: Config> = StorageMap<
		_,
		Identity,
		BoundedString<T::IPFSLength>,
		CarbonCreditBatchProposalInfo<MomentOf<T>, BalanceOf<T>, AccountIdOf<T>>,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Time Value Updated
		TimeValueUpdated(TimeType, BlockNumber<T>),
		/// Fee Value Updated
		FeeValueUpdated(FeeType, BalanceOf<T>),
		/// Vote Pass Ration Updated
		VotePassRatioUpdated(u16, u16),
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
		/// Carbon Credit Sale Order Created
		CarbonCreditSaleOrderCreated(AccountIdOf<T>, H256, BalanceOf<T>, BalanceOf<T>),
		/// Carbon Credit Sale Order Completed
		CarbonCreditSaleOrderCompleted(AccountIdOf<T>, H256),
		/// Carbon Credit Sale Order Closed
		CarbonCreditSaleOrderClosed(AccountIdOf<T>, H256),
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
		CarbonFootprintReportNotFound,
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
		CarbonFootprintReportAlreadySubmitted,
		/// User is active in CF report voting cycle
		UserIsActiveInCarbonFootprintReportVotingCycle,
		/// User is not eligible for carbon credit transactions
		UserIsNotEligibleForCarbonCreditTransactions,
		/// Carbon credit batch does not exist
		CarbonCreditBatchDoesNotExist,
		/// Carbon credit batch is not active
		CarbonCreditBatchIsNotActive,
		/// Not enought available credits
		NotEnoughtAvailableCredits,
		/// Carbon credit sale order doesnt exist
		CarbonCreditSaleOrderDoesntExist,
		/// The buyer can't buy his own tokens
		BuyerCantBuyHisOwnTokens,
		/// User didn't create the sell order
		UserDidntCreateTheSellOrder,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Update voting ratio
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn update_vote_pass_ratio(
			origin: OriginFor<T>,
			new_proportion_part: u16,
			new_upper_limit_part: u16,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			// Check if caller is a Authority account
			ensure!(
				AuthorityAccounts::<T>::get().contains(&user.clone()),
				Error::<T>::Unauthorized
			);

			let mut temp_proportion_part = new_proportion_part;

			if new_upper_limit_part == 0 {
				temp_proportion_part = 0;
			} else if new_proportion_part >= new_upper_limit_part {
				temp_proportion_part = new_upper_limit_part;
			}

			let new_pass_voting_ratio = ProportionStructure {
				proportion_part: temp_proportion_part,
				upper_limit_part: new_upper_limit_part,
			};

			VotePassRatio::<T>::set(new_pass_voting_ratio);

			Self::deposit_event(Event::VotePassRatioUpdated(
				temp_proportion_part,
				new_upper_limit_part,
			));

			Ok(().into())
		}

		// Update specific time value
		#[pallet::call_index(1)]
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
		#[pallet::call_index(2)]
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
		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn register_for_trader_account(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			// Check if the account is in use
			ensure!(Self::is_account_id_available(user.clone()), Error::<T>::AccountIdAlreadyInUse);

			// Check if the user is active in a CF report voting cycle
			ensure!(
				!Self::is_trying_to_register_as_cfa(user.clone()),
				Error::<T>::UserIsActiveInCarbonFootprintReportVotingCycle
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
		#[pallet::call_index(4)]
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
				Error::<T>::UserIsActiveInCarbonFootprintReportVotingCycle
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
			let validator_info: ProjectValidatorOrProjectOwnerInfo<T::IPFSLength, BlockNumber<T>> =
				ProjectValidatorOrProjectOwnerInfo {
					documentation_ipfs: documentation_ipfs.clone(),
					penalty_level: 0,
					penalty_timeout: BlockNumber::<T>::from(0u32),
				};
			ProjectValidators::<T>::insert(user.clone(), validator_info);

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
		#[pallet::call_index(5)]
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
				Error::<T>::UserIsActiveInCarbonFootprintReportVotingCycle
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
			let owner_info: ProjectValidatorOrProjectOwnerInfo<T::IPFSLength, BlockNumber<T>> =
				ProjectValidatorOrProjectOwnerInfo {
					documentation_ipfs: documentation_ipfs.clone(),
					penalty_level: 0,
					penalty_timeout: BlockNumber::<T>::from(0u32),
				};
			ProjectOwners::<T>::insert(user.clone(), owner_info);

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
		pub fn submit_carbon_footprint_report(
			origin: OriginFor<T>,
			ipfs: BoundedString<T::IPFSLength>,
			carbon_footprint_balance: i128,
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
				Error::<T>::CarbonFootprintReportAlreadySubmitted
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
			let report_info = CarbonFootprintReportInfo {
				cf_account: user.clone(),
				creation_date,
				carbon_footprint_balance,
				votes_for: BTreeSet::<AccountIdOf<T>>::new(),
				votes_against: BTreeSet::<AccountIdOf<T>>::new(),
				voting_active: true,
			};

			// Write to info storage
			CarbonFootprintReports::<T>::insert(ipfs.clone(), report_info);

			// Set voting timeout
			let current_block = frame_system::Pallet::<T>::block_number();
			let timeout_block = current_block + PalletTimeValues::<T>::get().voting_timeout;

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
		#[pallet::call_index(7)]
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
				PalletFeeValues::<T>::get().voting_fee <= T::Currency::free_balance(&user.clone()),
				Error::<T>::InsufficientFunds
			);

			match vote_type {
				VoteType::CarbonFootprintReportVote => {
					// Get report info and return error if it does not exist
					let mut report = CarbonFootprintReports::<T>::get(ipfs.clone())
						.ok_or(Error::<T>::CarbonFootprintReportNotFound)?;

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

					CarbonFootprintReports::<T>::insert(ipfs.clone(), report);
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
					let mut batch = CarbonCreditBatchProposals::<T>::get(ipfs.clone())
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

					CarbonCreditBatchProposals::<T>::insert(ipfs.clone(), batch);
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
		#[pallet::call_index(8)]
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
			let project_hash = Self::generate_hash(user.clone());

			// Project Proposal info
			let proposal_info = ProjectProposalInfo {
				project_owner: user.clone(),
				creation_date,
				project_hash,
				votes_for: BTreeSet::<AccountIdOf<T>>::new(),
				votes_against: BTreeSet::<AccountIdOf<T>>::new(),
				voting_active: true,
			};

			// Write to info storage
			ProjectProposals::<T>::insert(ipfs.clone(), proposal_info);

			// Set for voting timeout
			let current_block = frame_system::Pallet::<T>::block_number();
			let timeout_block = current_block + PalletTimeValues::<T>::get().voting_timeout;

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
		#[pallet::call_index(9)]
		#[pallet::weight(0)]
		pub fn propose_carbon_credit_batch(
			origin: OriginFor<T>,
			project_hash: H256,
			credit_amount: BalanceOf<T>,
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
			let batch_hash = Self::generate_hash(user.clone());

			// Get time
			let creation_date = T::Time::now();

			// CCB Proposal info
			let proposal_info = CarbonCreditBatchProposalInfo {
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
			CarbonCreditBatchProposals::<T>::insert(ipfs.clone(), proposal_info);

			// Set for voting timeout
			let current_block = frame_system::Pallet::<T>::block_number();
			let timeout_block = current_block + PalletTimeValues::<T>::get().voting_timeout;

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

		// Create carbon credit sale order
		#[pallet::call_index(10)]
		#[pallet::weight(0)]
		pub fn create_sale_order(
			origin: OriginFor<T>,
			batch_hash: H256,
			credit_price: BalanceOf<T>,
			credit_amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let seller = ensure_signed(origin)?;

			// Check if use can create a sale order
			ensure!(
				Self::is_eligible_for_carbon_credit_transaction(seller.clone()),
				Error::<T>::UserIsNotEligibleForCarbonCreditTransactions,
			);

			// Check if carbon credit batch exists
			let carbon_credit_batch = Self::get_batch(batch_hash)
				.expect(Error::<T>::CarbonCreditBatchDoesNotExist.into());

			// Check if the carbon credit batch is active
			ensure!(
				carbon_credit_batch.status == CarbonCreditBatchStatus::Active,
				Error::<T>::CarbonCreditBatchIsNotActive,
			);

			// Check if user has enough available credits
			let mut seller_holdings =
				CarbonCreditHoldings::<T>::get(batch_hash, seller.clone()).unwrap();

			ensure!(
				seller_holdings.available_amount >= credit_amount,
				Error::<T>::NotEnoughtAvailableCredits,
			);

			// Generate sale order hash
			let sale_hash = Self::generate_hash(seller.clone());

			// Set sale timeout
			let current_block = frame_system::Pallet::<T>::block_number();
			let timeout_block = current_block + PalletTimeValues::<T>::get().voting_timeout;

			// Create a carbon credit sale order
			// Note: If the buyer ID is the same as the seller we know that
			// the sale has not been finalized
			let sale_order = CarbonCreditSaleOrderInfo {
				batch_hash,
				credit_amount,
				credit_price,
				seller: seller.clone(),
				buyer: seller.clone(),
				sale_active: true,
				sale_timeout: timeout_block,
			};

			CarbonCreditSaleOrders::<T>::insert(sale_hash, sale_order);

			// Create a carbon credit sale order timeout event
			let mut sale_timeouts = BTreeSet::<H256>::new();

			if SaleOrderTimeouts::<T>::contains_key(timeout_block) {
				sale_timeouts = SaleOrderTimeouts::<T>::get(current_block).unwrap();
			}

			sale_timeouts.insert(sale_hash);

			SaleOrderTimeouts::<T>::insert(timeout_block, sale_timeouts);

			// Update seller carbon credit holdings
			seller_holdings = CarbonCreditHoldingsInfo {
				available_amount: seller_holdings.available_amount - credit_amount,
				unavailable_amount: seller_holdings.unavailable_amount + credit_amount,
			};

			CarbonCreditHoldings::<T>::insert(batch_hash, seller.clone(), seller_holdings);

			// Deposit event
			Self::deposit_event(Event::CarbonCreditSaleOrderCreated(
				seller,
				batch_hash,
				credit_amount,
				credit_price,
			));

			Ok(().into())
		}

		// Complete sale order (buy carbon credits)
		#[pallet::call_index(11)]
		#[pallet::weight(0)]
		pub fn complete_sale_order(
			origin: OriginFor<T>,
			sale_hash: H256,
		) -> DispatchResultWithPostInfo {
			let buyer = ensure_signed(origin)?;

			// Check if the user can transact with a sale order
			ensure!(
				Self::is_eligible_for_carbon_credit_transaction(buyer.clone()),
				Error::<T>::UserIsNotEligibleForCarbonCreditTransactions,
			);

			// Check if sale order exits
			ensure!(
				CarbonCreditSaleOrders::<T>::contains_key(sale_hash),
				Error::<T>::CarbonCreditSaleOrderDoesntExist
			);

			let mut sale_order = CarbonCreditSaleOrders::<T>::get(sale_hash).unwrap();

			// Check if the buyer isn't the seller
			ensure!(buyer != sale_order.seller, Error::<T>::BuyerCantBuyHisOwnTokens);

			// Check if carbon credit batch exists
			// Note: This will always return a carbon credit batch since a sale order would
			//		 not exits without one
			let carbon_credit_batch = Self::get_batch(sale_order.batch_hash).unwrap();

			// Check if the carbon credit batch is active
			ensure!(
				carbon_credit_batch.status == CarbonCreditBatchStatus::Active,
				Error::<T>::CarbonCreditBatchIsNotActive,
			);

			// TODO: Implement penalty charges

			// Check if the buyer has enough assets
			let amount_to_pay = sale_order.credit_amount * sale_order.credit_price;

			ensure!(
				amount_to_pay <= T::Currency::free_balance(&buyer.clone()),
				Error::<T>::InsufficientFunds
			);

			// TODO: Implement profit splitting

			// Transfer funds
			T::Currency::transfer(
				&sale_order.seller,
				&buyer,
				amount_to_pay,
				ExistenceRequirement::KeepAlive,
			)?;

			// Update sell order
			sale_order = CarbonCreditSaleOrderInfo {
				buyer: buyer.clone(),
				sale_active: false,
				..sale_order
			};

			CarbonCreditSaleOrders::<T>::insert(sale_hash, sale_order.clone());

			// Update seller holdings
			let mut seller_holdings =
				CarbonCreditHoldings::<T>::get(sale_order.batch_hash, sale_order.clone().seller)
					.unwrap();

			seller_holdings = CarbonCreditHoldingsInfo {
				unavailable_amount: seller_holdings.unavailable_amount - sale_order.credit_amount,
				..seller_holdings
			};

			// Check to see if the seller sold all of his/hers tokens
			// Note: If the seller has sold all of his tokens (available + unavailable) then we will delete him/her
			//		 If not then the seller will still be active
			if seller_holdings.available_amount == BalanceOf::<T>::from(0u32)
				&& seller_holdings.unavailable_amount == BalanceOf::<T>::from(0u32)
			{
				CarbonCreditHoldings::<T>::remove(sale_order.batch_hash, sale_order.seller);
			} else {
				CarbonCreditHoldings::<T>::insert(
					sale_order.batch_hash,
					sale_order.seller,
					seller_holdings,
				);
			}

			// Update buyer holdings
			// Note: First we will check if the buyer has some preexisting holdings and if that's the case we will update them
			//		 If that is not the case we will create a new holdings entity
			let mut buyer_holdings = CarbonCreditHoldingsInfo {
				available_amount: sale_order.credit_amount,
				unavailable_amount: BalanceOf::<T>::from(0u32),
			};

			if CarbonCreditHoldings::<T>::contains_key(sale_order.batch_hash, buyer.clone()) {
				buyer_holdings =
					CarbonCreditHoldings::<T>::get(sale_order.batch_hash, buyer.clone()).unwrap();

				buyer_holdings = CarbonCreditHoldingsInfo {
					available_amount: buyer_holdings.available_amount + sale_order.credit_amount,
					..buyer_holdings
				};
			}

			CarbonCreditHoldings::<T>::insert(sale_order.batch_hash, buyer.clone(), buyer_holdings);

			// Remove sale timeout event
			let mut sale_timeouts = SaleOrderTimeouts::<T>::get(sale_order.sale_timeout).unwrap();

			sale_timeouts.remove(&sale_hash);

			SaleOrderTimeouts::<T>::insert(sale_order.sale_timeout, sale_timeouts);

			// Deposit event
			Self::deposit_event(Event::CarbonCreditSaleOrderCompleted(buyer, sale_hash));

			Ok(().into())
		}

		// Close carbon credit sale order
		#[pallet::call_index(12)]
		#[pallet::weight(0)]
		pub fn close_sale_order(
			origin: OriginFor<T>,
			sale_hash: H256,
		) -> DispatchResultWithPostInfo {
			let seller = ensure_signed(origin)?;

			// Check if the user can transact with a sale order
			ensure!(
				Self::is_eligible_for_carbon_credit_transaction(seller.clone()),
				Error::<T>::UserIsNotEligibleForCarbonCreditTransactions,
			);

			// Check if sale order exits
			ensure!(
				CarbonCreditSaleOrders::<T>::contains_key(sale_hash),
				Error::<T>::CarbonCreditSaleOrderDoesntExist
			);

			let mut sale_order = CarbonCreditSaleOrders::<T>::get(sale_hash).unwrap();

			// Check if the seller created the sale order
			ensure!(seller == sale_order.seller, Error::<T>::UserDidntCreateTheSellOrder);

			// Check if carbon credit batch exists
			// Note: This will always return a carbon credit batch since a sale order would
			//		 not exits without one
			let carbon_credit_batch = Self::get_batch(sale_order.batch_hash).unwrap();

			// Check if the carbon credit batch is active
			ensure!(
				carbon_credit_batch.status == CarbonCreditBatchStatus::Active,
				Error::<T>::CarbonCreditBatchIsNotActive,
			);

			// Update seller holdings
			let mut seller_holdings =
				CarbonCreditHoldings::<T>::get(sale_order.batch_hash, seller.clone()).unwrap();

			seller_holdings = CarbonCreditHoldingsInfo {
				unavailable_amount: seller_holdings.unavailable_amount - sale_order.credit_amount,
				available_amount: seller_holdings.available_amount + sale_order.credit_amount,
			};

			CarbonCreditHoldings::<T>::insert(
				sale_order.batch_hash,
				seller.clone(),
				seller_holdings,
			);

			// Update sale order
			sale_order = CarbonCreditSaleOrderInfo { sale_active: false, ..sale_order };

			CarbonCreditSaleOrders::<T>::insert(sale_hash, sale_order.clone());

			// Remove sale order timeout
			let mut sale_timeouts = SaleOrderTimeouts::<T>::get(sale_order.sale_timeout).unwrap();
			sale_timeouts.remove(&sale_hash);

			SaleOrderTimeouts::<T>::insert(sale_order.sale_timeout, sale_timeouts);

			// Deposit event
			Self::deposit_event(Event::CarbonCreditSaleOrderClosed(seller, sale_hash));

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
			let mut consumed_weight = Self::update_pallet_base_time(now);
			consumed_weight += Self::check_voting_timeouts(now);
			consumed_weight += Self::check_sale_timeouts(now);

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

		// Generate hash identifier
		fn generate_hash(user: AccountIdOf<T>) -> H256 {
			let nonce = frame_system::Pallet::<T>::account_nonce(&user);
			let now = T::Time::now();

			let encoded: [u8; 32] = (&user, nonce, now).using_encoded(blake2_256);

			let hash = H256::from(encoded);

			hash
		}

		// Check if the documentation (ipfs link) has been used previously
		// Return false if the documentation is used
		// Return true if the documentation is available
		pub fn is_ipfs_available(ipfs: BoundedString<T::IPFSLength>) -> bool {
			// Check in reports and proposals
			if CarbonFootprintReports::<T>::contains_key(ipfs.clone())
				|| ProjectProposals::<T>::contains_key(ipfs.clone())
				|| CarbonCreditBatchProposals::<T>::contains_key(ipfs.clone())
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

		// Check if the account is eligible for carbon credit transactions
		// Return true if the account_id can transact with carbon credits (CFA, Trader, Project Owner)
		// Return true if the account_id can't transact with carbon credits (Validator, or non-registered account)
		pub fn is_eligible_for_carbon_credit_transaction(account_id: AccountIdOf<T>) -> bool {
			if CarbonFootprintAccounts::<T>::contains_key(account_id.clone())
				|| ProjectValidators::<T>::contains_key(account_id.clone())
				|| ProjectOwners::<T>::contains_key(account_id.clone())
			{
				return true;
			}

			return false;
		}

		// Check if the account has submitted a carbon footprint report for voting
		// Note: If a user has submitted a CF report he/she then can not registed as another account type
		pub fn is_trying_to_register_as_cfa(account_id: AccountIdOf<T>) -> bool {
			// Check in CF accounts
			for (_, cf_report) in <CarbonFootprintReports<T>>::iter() {
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

		// Check if any voting timeout event has occured
		pub fn check_voting_timeouts(now: BlockNumber<T>) -> Weight {
			let mut counter: u64 = 0;

			if VotingTimeouts::<T>::contains_key(now) {
				let timeout_events: BTreeSet<BoundedString<T::IPFSLength>> =
					VotingTimeouts::<T>::get(now).unwrap();

				for ipfs in timeout_events.iter() {
					// Check if IPFS is related to a carbon footprint report
					if CarbonFootprintReports::<T>::contains_key(ipfs.clone()) {
						let report = CarbonFootprintReports::<T>::get(ipfs.clone()).unwrap();

						Self::update_carbon_footprint_account(report, ipfs.clone());

						counter += 1;
					}

					// Check if IPFS is related to a project proposal
					if ProjectProposals::<T>::contains_key(ipfs.clone()) {
						let proposal = ProjectProposals::<T>::get(ipfs.clone()).unwrap();

						Self::update_project_proposal(proposal, ipfs.clone());

						counter += 1;
					}

					// Check if IPFS is related to a carbon credits batch proposal
					if CarbonCreditBatchProposals::<T>::contains_key(ipfs.clone()) {
						let proposal = CarbonCreditBatchProposals::<T>::get(ipfs.clone()).unwrap();

						Self::update_carbon_credit_batch(proposal, ipfs.clone());

						counter += 1;
					}
				}

				VotingTimeouts::<T>::remove(now);
			}

			T::DbWeight::get()
				.reads(counter)
				.saturating_add(T::DbWeight::get().writes(counter))
		}

		// Check if any sale timeout event has occured
		pub fn check_sale_timeouts(now: BlockNumber<T>) -> Weight {
			let mut counter: u64 = 0;

			if SaleOrderTimeouts::<T>::contains_key(now) {
				let sale_events = SaleOrderTimeouts::<T>::get(now).unwrap();

				for sale_hash in sale_events.iter() {
					// Get sale order
					let mut sale_order = CarbonCreditSaleOrders::<T>::get(sale_hash).unwrap();

					// Update sale order
					sale_order = CarbonCreditSaleOrderInfo { sale_active: false, ..sale_order };

					CarbonCreditSaleOrders::<T>::insert(sale_hash, sale_order.clone());

					// Update seller holdings
					let mut seller_holdings =
						CarbonCreditHoldings::<T>::get(sale_hash, sale_order.clone().seller).unwrap();

					seller_holdings = CarbonCreditHoldingsInfo {
						available_amount: seller_holdings.available_amount
							+ sale_order.credit_amount,
						unavailable_amount: seller_holdings.unavailable_amount
							- sale_order.credit_amount,
					};

					CarbonCreditHoldings::<T>::insert(
						sale_hash,
						sale_order.seller,
						seller_holdings,
					);

					counter += 1;
				}
			}

			T::DbWeight::get()
				.reads(counter)
				.saturating_add(T::DbWeight::get().writes(counter))
		}

		// Update carbon footprint account after voting ends
		pub fn update_carbon_footprint_account(
			report: CarbonFootprintReportInfo<AccountIdOf<T>, MomentOf<T>>,
			ipfs: BoundedString<T::IPFSLength>,
		) {
			// Get the votes that were made for the report
			let votes_for: u16 = report.votes_for.len().try_into().unwrap();
			let votes_against: u16 = report.votes_against.len().try_into().unwrap();
			let votes_total: u16 = votes_for + votes_against;

			// Check if the vote has passed
			if Self::has_vote_passed(votes_total, votes_for) {
				// Create an empty carbon footprint account
				let mut new_account = CarbonFootprintAccountInfo {
					documentation_ipfses: BTreeSet::<BoundedString<T::IPFSLength>>::new(),
					carbon_footprint_balance: report.carbon_footprint_balance,
					creation_date: T::Time::now(),
				};

				// Check to see if a carbon footprint account with the given accountID exists
				if CarbonFootprintAccounts::<T>::contains_key(report.cf_account.clone()) {
					let old_account =
						CarbonFootprintAccounts::<T>::get(report.cf_account.clone()).unwrap();

					// Update documentation related to the carbon footprint account
					let mut new_documentation = old_account.documentation_ipfses;
					new_documentation.insert(ipfs.clone());

					// Update the carbon footprint account structure
					new_account = CarbonFootprintAccountInfo {
						documentation_ipfses: new_documentation,
						carbon_footprint_balance: old_account.carbon_footprint_balance
							+ report.carbon_footprint_balance,
						creation_date: old_account.creation_date,
					}
				}

				// Save the changes made to the carbon footprint account
				CarbonFootprintAccounts::<T>::insert(report.cf_account.clone(), new_account);
			}

			// Create new report
			// Note: Only change is made to the voting_active cycle status
			let new_report = CarbonFootprintReportInfo {
				cf_account: report.cf_account.clone(),
				creation_date: report.creation_date,
				carbon_footprint_balance: report.carbon_footprint_balance,
				votes_for: report.votes_for,
				votes_against: report.votes_against,
				voting_active: false,
			};

			// Save new report
			CarbonFootprintReports::<T>::insert(ipfs, new_report);
		}

		// Update project proposal after voting ends
		pub fn update_project_proposal(
			proposal: ProjectProposalInfo<AccountIdOf<T>, MomentOf<T>>,
			ipfs: BoundedString<T::IPFSLength>,
		) {
			// Get the votes that were made for the report
			let votes_for: u16 = proposal.votes_for.len().try_into().unwrap();
			let votes_against: u16 = proposal.votes_against.len().try_into().unwrap();
			let votes_total: u16 = votes_for + votes_against;

			// Check if the vote has passed
			if Self::has_vote_passed(votes_total, votes_for) {
				// Create a new project
				let new_project = ProjectInfo {
					documentation_ipfs: ipfs.clone(),
					project_owner: proposal.project_owner.clone(),
					creation_date: T::Time::now(),
					penalty_level: 0,
					penalty_timeout: BlockNumber::<T>::from(0u32),
				};

				// Save new project
				Projects::<T>::insert(proposal.project_hash, new_project);
			}

			// Create new proposal
			// Note: Only change is made to the voting_active cycle status
			let new_proposal = ProjectProposalInfo {
				project_owner: proposal.project_owner.clone(),
				creation_date: proposal.creation_date,
				project_hash: proposal.project_hash,
				votes_for: proposal.votes_for,
				votes_against: proposal.votes_against,
				voting_active: false,
			};

			// Save new proposal
			ProjectProposals::<T>::insert(ipfs, new_proposal);
		}

		// Update carbon credit batch after voting ends
		pub fn update_carbon_credit_batch(
			proposal: CarbonCreditBatchProposalInfo<MomentOf<T>, BalanceOf<T>, AccountIdOf<T>>,
			ipfs: BoundedString<T::IPFSLength>,
		) {
			// Get the votes that were made for the report
			let votes_for: u16 = proposal.votes_for.len().try_into().unwrap();
			let votes_against: u16 = proposal.votes_against.len().try_into().unwrap();
			let votes_total: u16 = votes_for + votes_against;

			// Check if the vote has passed
			if Self::has_vote_passed(votes_total, votes_for) {
				// Create a new project
				let new_batch = CarbonCreditBatchInfo {
					documentation_ipfs: ipfs.clone(),
					creation_date: T::Time::now(),
					credit_amount: proposal.credit_amount,
					initial_credit_price: proposal.initial_credit_price,
					status: CarbonCreditBatchStatus::Active,
				};

				// Save new carbon credit batch
				CarbonCreditBatches::<T>::insert(
					proposal.project_hash,
					proposal.batch_hash,
					new_batch,
				);

				// Create carbon credit holdings for project owner
				let new_holdings = CarbonCreditHoldingsInfo {
					available_amount: proposal.credit_amount.into(),
					unavailable_amount: BalanceOf::<T>::from(0u32),
				};

				// Get project owner accountID
				let project = Projects::<T>::get(proposal.project_hash).unwrap();

				// Save project owner carbon credit holdings
				CarbonCreditHoldings::<T>::insert(
					proposal.batch_hash,
					project.project_owner,
					new_holdings,
				);
			}

			// Create new proposal
			// Note: Only change is made to the voting_active cycle status
			let new_proposal = CarbonCreditBatchProposalInfo {
				project_hash: proposal.project_hash,
				batch_hash: proposal.batch_hash,
				creation_date: proposal.creation_date,
				credit_amount: proposal.credit_amount,
				initial_credit_price: proposal.initial_credit_price,
				votes_for: proposal.votes_for,
				votes_against: proposal.votes_against,
				voting_active: false,
			};

			// Save new proposal
			CarbonCreditBatchProposals::<T>::insert(ipfs, new_proposal);
		}

		// Check if vote has passed
		pub fn has_vote_passed(total_votes: u16, votes_for: u16) -> bool {
			let vote_pass_ratio = VotePassRatio::<T>::get();

			if vote_pass_ratio.upper_limit_part == 0 && votes_for >= total_votes - votes_for + 1 {
				return true;
			}

			if vote_pass_ratio.upper_limit_part == vote_pass_ratio.proportion_part
				&& votes_for == total_votes
			{
				return true;
			}

			let needed_votes =
				(vote_pass_ratio.proportion_part * total_votes) / vote_pass_ratio.upper_limit_part;

			if votes_for >= needed_votes {
				return true;
			}

			return false;
		}

		// Get batch information by hash value
		pub fn get_batch(
			batch_hash: H256,
		) -> Option<
			CarbonCreditBatchInfo<
				T::IPFSLength,
				MomentOf<T>,
				BalanceOf<T>,
				CarbonCreditBatchStatus,
			>,
		> {
			CarbonCreditBatches::<T>::iter_prefix(&batch_hash)
				.into_iter()
				.find_map(|(hash, info)| if hash == batch_hash { Some(info) } else { None })
		}
	}
}
