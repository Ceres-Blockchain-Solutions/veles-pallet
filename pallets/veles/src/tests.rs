use crate::{mock::*, Error};
use frame_support::{assert_err, assert_ok};

#[test]
fn update_base_pallet_time_zero_ok() {
	new_test_ext().execute_with(|| {
		// Check for base pallet time before block 1
		let base_pallet_time = BasePalletTime::<Test>::get();

		assert_eq!(base_pallet_time, 0);

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Check for base pallet time on block 1
		let base_pallet_time = BasePalletTime::<Test>::get();

		assert_eq!(base_pallet_time, 1);

		// Go to block 10
		run_to_block(10);

		// Check for base pallet time on block 10
		let base_pallet_time = BasePalletTime::<Test>::get();

		assert_eq!(base_pallet_time, 1);
	});
}

#[test]
fn update_base_pallet_time_new_year_ok() {
	new_test_ext().execute_with(|| {
		// Check for base pallet time before block 1
		let base_pallet_time = BasePalletTime::<Test>::get();

		assert_eq!(base_pallet_time, 0);

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Check for base pallet time on block 1
		let base_pallet_time = BasePalletTime::<Test>::get();

		assert_eq!(base_pallet_time, 1);

		// Set number of block yearly
		NumberOfBlocksYearlyStorage::<Test>::set(100);

		// Go to block 110
		run_to_block(110);

		// Check for base pallet time on block 10
		let base_pallet_time = BasePalletTime::<Test>::get();

		assert_eq!(base_pallet_time, 101);
	});
}

#[test]
fn change_timeout_time_unauthorized() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		run_to_block(1);

		// Check for Unauthorized error
		assert_err!(
			Veles::change_timeout_time(RuntimeOrigin::signed(alice()), TimeoutType::Penalty, 0),
			Error::<Test>::Unauthorized
		);
	});
}

#[test]
fn change_timeout_time_invalid_timeout_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert authority account
		let mut new_authorities = AuthorityAccounts::<Test>::get();
		new_authorities.insert(alice());
		AuthorityAccounts::<Test>::set(new_authorities);

		// Check for InvalidTimeoutValue error
		assert_err!(
			Veles::change_timeout_time(RuntimeOrigin::signed(alice()), TimeoutType::Penalty, 0),
			Error::<Test>::InvalidTimeoutValue
		);
	});
}

#[test]
fn change_timeout_time_ok_penalty() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert authority account
		let mut new_authorities = AuthorityAccounts::<Test>::get();
		new_authorities.insert(alice());
		AuthorityAccounts::<Test>::set(new_authorities);

		// Update timeout time
		assert_ok!(Veles::change_timeout_time(
			RuntimeOrigin::signed(alice()),
			TimeoutType::Penalty,
			1
		));

		// Check updated timeout time
		assert_eq!(PenaltyTimeoutTime::<Test>::get(), 1);
	});
}

#[test]
fn change_timeout_time_ok_voting() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert authority account
		let mut new_authorities = AuthorityAccounts::<Test>::get();
		new_authorities.insert(alice());
		AuthorityAccounts::<Test>::set(new_authorities);

		// Update timeout time
		assert_ok!(Veles::change_timeout_time(
			RuntimeOrigin::signed(alice()),
			TimeoutType::Voting,
			1
		));

		// Check updated timeout time
		assert_eq!(VotingTimeoutTime::<Test>::get(), 1);
	});
}

#[test]
fn change_timeout_time_ok_sales() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert authority account
		let mut new_authorities = AuthorityAccounts::<Test>::get();
		new_authorities.insert(alice());
		AuthorityAccounts::<Test>::set(new_authorities);

		// Update timeout time
		assert_ok!(Veles::change_timeout_time(
			RuntimeOrigin::signed(alice()),
			TimeoutType::Sales,
			1
		));

		// Check updated timeout time
		assert_eq!(SalesTimeoutTime::<Test>::get(), 1);
	});
}

#[test]
fn change_fee_amount_unauthorized() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		run_to_block(1);

		// Check for Unauthorized error
		assert_err!(
			Veles::change_fee_amount(RuntimeOrigin::signed(alice()), FeeType::TraderAccountFee, 0),
			Error::<Test>::Unauthorized
		);
	});
}

#[test]
fn change_fee_amount_ok_trader_account_fee() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert authority account
		let mut new_authorities = AuthorityAccounts::<Test>::get();
		new_authorities.insert(alice());
		AuthorityAccounts::<Test>::set(new_authorities);

		// Update fee amount
		assert_ok!(Veles::change_fee_amount(
			RuntimeOrigin::signed(alice()),
			FeeType::TraderAccountFee,
			0
		));

		// Check updated fee amount
		assert_eq!(TraderAccountFee::<Test>::get(), 0);
	});
}

#[test]
fn change_fee_amount_ok_project_validator_account_fee() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert authority account
		let mut new_authorities = AuthorityAccounts::<Test>::get();
		new_authorities.insert(alice());
		AuthorityAccounts::<Test>::set(new_authorities);

		// Update fee amount
		assert_ok!(Veles::change_fee_amount(
			RuntimeOrigin::signed(alice()),
			FeeType::ProjectValidatorAccountFee,
			0
		));

		// Check updated fee amount
		assert_eq!(ProjectValidatorAccountFee::<Test>::get(), 0);
	});
}

#[test]
fn change_fee_amount_ok_project_owner_account_fee_fee() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert authority account
		let mut new_authorities = AuthorityAccounts::<Test>::get();
		new_authorities.insert(alice());
		AuthorityAccounts::<Test>::set(new_authorities);

		// Update fee amount
		assert_ok!(Veles::change_fee_amount(
			RuntimeOrigin::signed(alice()),
			FeeType::ProjectOwnerAccountFee,
			0
		));

		// Check updated fee amount
		assert_eq!(ProjectOwnerAccountFee::<Test>::get(), 0);
	});
}

#[test]
fn register_for_trader_account_account_id_already_in_use() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert trader account
		let mut new_traders = TraderAccounts::<Test>::get();
		new_traders.insert(alice());
		TraderAccounts::<Test>::set(new_traders);

		// Check for AccountIdAlreadyInUse error
		assert_err!(
			Veles::register_for_trader_account(RuntimeOrigin::signed(alice()),),
			Error::<Test>::AccountIdAlreadyInUse
		);
	});
}

#[test]
fn register_for_trader_account_insufficient_funds() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		run_to_block(1);

		// Check for InsufficientFunds error
		assert_err!(
			Veles::register_for_trader_account(RuntimeOrigin::signed(alice()),),
			Error::<Test>::InsufficientFunds
		);
	});
}

#[test]
fn register_for_trader_account_ok() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		run_to_block(1);

		// Registered trader account succesfully
		assert_ok!(Veles::register_for_trader_account(RuntimeOrigin::signed(bob())));
	});
}

#[test]
fn register_for_project_validator_account_id_already_in_use() {
	new_test_ext().execute_with(|| {
		// Create documentation IPFS
		let documentation_ipfs =
			BoundedString::<IPFSLength>::truncate_from("ipfs_pvalidator_documentation");

		// Create project validator info
		let pvalidator_info = PVoPOInfo {
			documentation_ipfs: documentation_ipfs.clone(),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project validator
		ProjectValidators::<Test>::insert(alice(), pvalidator_info);

		// Check for ProjectValidatorAlreadyExists error
		assert_err!(
			Veles::register_for_project_validator_account(
				RuntimeOrigin::signed(alice()),
				documentation_ipfs
			),
			Error::<Test>::AccountIdAlreadyInUse
		);
	});
}

#[test]
fn register_for_project_validator_account_documentation_was_used_previously() {
	new_test_ext().execute_with(|| {
		// Create documentation IPFS
		let documentation_ipfs =
			BoundedString::<IPFSLength>::truncate_from("ipfs_pvalidator_documentation");

		// Create project validator info
		let pvalidator_info = PVoPOInfo {
			documentation_ipfs: documentation_ipfs.clone(),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project validator
		ProjectValidators::<Test>::insert(alice(), pvalidator_info);

		// Check for DocumentationWasUsedPreviously error
		assert_err!(
			Veles::register_for_project_validator_account(
				RuntimeOrigin::signed(bob()),
				documentation_ipfs
			),
			Error::<Test>::DocumentationWasUsedPreviously
		);
	});
}

#[test]
fn register_for_project_validator_account_insufficient_funds() {
	new_test_ext().execute_with(|| {
		// Create documentation IPFS
		let documentation_ipfs =
			BoundedString::<IPFSLength>::truncate_from("ipfs_pvalidator_documentation");

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Check for InsufficientFunds error
		assert_err!(
			Veles::register_for_project_validator_account(
				RuntimeOrigin::signed(alice()),
				documentation_ipfs
			),
			Error::<Test>::InsufficientFunds
		);
	});
}

#[test]
fn register_for_project_owner_account_insufficient_funds() {
	new_test_ext().execute_with(|| {
		// Create documentation IPFS
		let documentation_ipfs =
			BoundedString::<IPFSLength>::truncate_from("ipfs_powner_documentation");

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Check for InsufficientFunds error
		assert_err!(
			Veles::register_for_project_owner_account(
				RuntimeOrigin::signed(alice()),
				documentation_ipfs
			),
			Error::<Test>::InsufficientFunds
		);
	});
}

#[test]
fn register_for_project_validator_account_ok() {
	new_test_ext().execute_with(|| {
		// Create documentation IPFS
		let documentation_ipfs =
			BoundedString::<IPFSLength>::truncate_from("ipfs_pvalidator_documentation");

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Registered project validator account succesfully
		assert_ok!(Veles::register_for_project_validator_account(
			RuntimeOrigin::signed(bob()),
			documentation_ipfs
		));
	});
}

#[test]
fn register_for_project_owner_account_account_id_already_in_use() {
	new_test_ext().execute_with(|| {
		// Create documentation IPFS
		let documentation_ipfs =
			BoundedString::<IPFSLength>::truncate_from("ipfs_powner_documentation");

		// Create project owner info
		let powner_info = PVoPOInfo {
			documentation_ipfs: documentation_ipfs.clone(),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project owner
		ProjectOwners::<Test>::insert(alice(), powner_info);

		// Check for AccountIdAlreadyInUse error
		assert_err!(
			Veles::register_for_project_owner_account(
				RuntimeOrigin::signed(alice()),
				documentation_ipfs
			),
			Error::<Test>::AccountIdAlreadyInUse
		);
	});
}

#[test]
fn register_for_project_owner_account_documentation_was_used_previously() {
	new_test_ext().execute_with(|| {
		// Create documentation IPFS
		let documentation_ipfs =
			BoundedString::<IPFSLength>::truncate_from("ipfs_powner_documentation");

		// Create project owner info
		let powner_info = PVoPOInfo {
			documentation_ipfs: documentation_ipfs.clone(),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project owner
		ProjectOwners::<Test>::insert(alice(), powner_info);

		// Check for DocumentationWasUsedPreviously error
		assert_err!(
			Veles::register_for_project_owner_account(
				RuntimeOrigin::signed(bob()),
				documentation_ipfs
			),
			Error::<Test>::DocumentationWasUsedPreviously
		);
	});
}

#[test]
fn register_for_project_owner_account_ok() {
	new_test_ext().execute_with(|| {
		// Create documentation IPFS
		let documentation_ipfs =
			BoundedString::<IPFSLength>::truncate_from("ipfs_powner_documentation");

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Registered project owner account succesfully
		assert_ok!(Veles::register_for_project_owner_account(
			RuntimeOrigin::signed(bob()),
			documentation_ipfs
		));
	});
}

#[test]
fn cast_vote_unauthorized() {
	new_test_ext().execute_with(|| {
		// Create IPFS documentation link
		let ipfs_documentation = BoundedString::<IPFSLength>::truncate_from("ipfs_documentation");

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Check for Unauthorized errors
		assert_err!(
			Veles::cast_vote(
				RuntimeOrigin::signed(alice()),
				VoteType::CFReportVote,
				ipfs_documentation.clone(),
				false
			),
			Error::<Test>::Unauthorized
		);

		assert_err!(
			Veles::cast_vote(
				RuntimeOrigin::signed(alice()),
				VoteType::PProposalVote,
				ipfs_documentation.clone(),
				false
			),
			Error::<Test>::Unauthorized
		);

		assert_err!(
			Veles::cast_vote(
				RuntimeOrigin::signed(alice()),
				VoteType::CCBatchVote,
				ipfs_documentation.clone(),
				false
			),
			Error::<Test>::Unauthorized
		);
	})
}

#[test]
fn cast_vote_cf_report_report_not_found() {
	new_test_ext().execute_with(|| {
		// Create CF report IPFS link
		let ipfs_cfreport_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_cfreport_documentation");

		// Create project validator info
		let pvalidator_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from(
				"ipfs_pvalidator_documentation",
			),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project validator
		ProjectValidators::<Test>::insert(alice(), pvalidator_info);

		// Check for CFReportNotFound error
		assert_err!(
			Veles::cast_vote(
				RuntimeOrigin::signed(alice()),
				VoteType::CFReportVote,
				ipfs_cfreport_documentation,
				false
			),
			Error::<Test>::CFReportNotFound
		);
	});
}

#[test]
fn cast_vote_cfreport_ok() {
	new_test_ext().execute_with(|| {
		// Create CF report IPFS link
		let ipfs_cfreport_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_cfreport_documentation");

		// Create project validator info
		let pvalidator_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from(
				"ipfs_pvalidator_documentation",
			),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Create CF report info
		let cfreport_info = CFReportInfo {
			cf_account: bob(),
			creation_date: 0,
			carbon_balance: 0,
			votes_for: BTreeSet::<AccountId>::new(),
			votes_against: BTreeSet::<AccountId>::new(),
			voting_active: true,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project validator
		ProjectValidators::<Test>::insert(alice(), pvalidator_info);

		// Insert CF report
		CFReports::<Test>::insert(ipfs_cfreport_documentation.clone(), cfreport_info);

		// Vote succesfully
		assert_ok!(Veles::cast_vote(
			RuntimeOrigin::signed(alice()),
			VoteType::CFReportVote,
			ipfs_cfreport_documentation.clone(),
			false
		));
	});
}

#[test]
fn cast_vote_cfreport_voting_cycle_is_over_submitted() {
	new_test_ext().execute_with(|| {
		// Create CF report IPFS link
		let ipfs_cfreport_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_cfreport_documentation");

		// Create project validator info
		let pvalidator_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from(
				"ipfs_pvalidator_documentation",
			),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Create CF info
		let cfreport_info = CFReportInfo {
			cf_account: bob(),
			creation_date: 0,
			carbon_balance: 0,
			votes_for: BTreeSet::<AccountId>::new(),
			votes_against: BTreeSet::<AccountId>::new(),
			voting_active: false,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project validator
		ProjectValidators::<Test>::insert(alice(), pvalidator_info);

		// Insert CF report
		CFReports::<Test>::insert(ipfs_cfreport_documentation.clone(), cfreport_info);

		// Check for VotingCycleIsOver error
		assert_err!(
			Veles::cast_vote(
				RuntimeOrigin::signed(alice()),
				VoteType::CFReportVote,
				ipfs_cfreport_documentation,
				false
			),
			Error::<Test>::VotingCycleIsOver
		);
	});
}

#[test]
fn cast_vote_cfreport_vote_already_submitted() {
	new_test_ext().execute_with(|| {
		// Create CF report IPFS link
		let ipfs_cfreport_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_cfreport_documentation");

		// Create project validator info
		let pvalidator_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from(
				"ipfs_pvalidator_documentation",
			),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Create CF info
		let cfreport_info = CFReportInfo {
			cf_account: bob(),
			creation_date: 0,
			carbon_balance: 0,
			votes_for: BTreeSet::<AccountId>::new(),
			votes_against: BTreeSet::<AccountId>::new(),
			voting_active: true,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project validator
		ProjectValidators::<Test>::insert(alice(), pvalidator_info);

		// Insert CF report
		CFReports::<Test>::insert(ipfs_cfreport_documentation.clone(), cfreport_info);

		// Vote succesfully
		assert_ok!(Veles::cast_vote(
			RuntimeOrigin::signed(alice()),
			VoteType::CFReportVote,
			ipfs_cfreport_documentation.clone(),
			false
		));

		// Check for VoteAlreadySubmitted error
		assert_err!(
			Veles::cast_vote(
				RuntimeOrigin::signed(alice()),
				VoteType::CFReportVote,
				ipfs_cfreport_documentation,
				false
			),
			Error::<Test>::VoteAlreadySubmitted
		);
	});
}

#[test]
fn cast_vote_pproposal_project_proposal_not_found() {
	new_test_ext().execute_with(|| {
		// Create project proposal IPFS link
		let ipfs_project_proposal_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_project_proposal_documentation");

		// Create project validator info
		let project_validator_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from(
				"ipfs_project_validator_documentation",
			),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project validator
		ProjectValidators::<Test>::insert(alice(), project_validator_info);

		// Check for ProjectProposalNotFound error
		assert_err!(
			Veles::cast_vote(
				RuntimeOrigin::signed(alice()),
				VoteType::PProposalVote,
				ipfs_project_proposal_documentation,
				false
			),
			Error::<Test>::ProjectProposalNotFound
		);
	});
}

#[test]
fn cast_vote_pproposal_ok() {
	new_test_ext().execute_with(|| {
		// Create project proposal IPFS link
		let ipfs_project_proposal_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_project_proposal_documentation");

		// Create project validator info
		let project_validator_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from(
				"ipfs_project_validator_documentation",
			),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Create project hash
		let nonce = frame_system::Pallet::<Test>::account_nonce(bob());
		let encoded: [u8; 32] = (bob(), nonce).using_encoded(blake2_256);
		let project_hash = H256::from(encoded);

		// Create project proposal info
		let project_proposal_info = ProjectProposalInfo {
			project_owner: bob(),
			creation_date: 0,
			project_hash,
			votes_for: BTreeSet::<AccountId>::new(),
			votes_against: BTreeSet::<AccountId>::new(),
			voting_active: true,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project validator
		ProjectValidators::<Test>::insert(alice(), project_validator_info);

		// Insert project proposal info
		ProjectProposals::<Test>::insert(
			ipfs_project_proposal_documentation.clone(),
			project_proposal_info,
		);

		// Vote succesfully
		assert_ok!(Veles::cast_vote(
			RuntimeOrigin::signed(alice()),
			VoteType::PProposalVote,
			ipfs_project_proposal_documentation,
			false
		));
	});
}

#[test]
fn cast_vote_pproposal_vote_already_submitted() {
	new_test_ext().execute_with(|| {
		// Create project proposal IPFS link
		let ipfs_project_proposal_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_project_proposal_documentation");

		// Create project validator info
		let project_validator_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from(
				"ipfs_project_validator_documentation",
			),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Create project hash
		let nonce = frame_system::Pallet::<Test>::account_nonce(alice());
		let encoded: [u8; 32] = (alice(), nonce).using_encoded(blake2_256);
		let project_hash = H256::from(encoded);

		// Create project proposal info
		let project_proposal_info = ProjectProposalInfo {
			project_owner: bob(),
			creation_date: 0,
			project_hash,
			votes_for: BTreeSet::<AccountId>::new(),
			votes_against: BTreeSet::<AccountId>::new(),
			voting_active: true,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project validator
		ProjectValidators::<Test>::insert(alice(), project_validator_info);

		// Insert project proposal info
		ProjectProposals::<Test>::insert(
			ipfs_project_proposal_documentation.clone(),
			project_proposal_info,
		);

		// Vote succesfully
		assert_ok!(Veles::cast_vote(
			RuntimeOrigin::signed(alice()),
			VoteType::PProposalVote,
			ipfs_project_proposal_documentation.clone(),
			false
		));

		// Check for VoteAlreadySubmitted error
		assert_err!(
			Veles::cast_vote(
				RuntimeOrigin::signed(alice()),
				VoteType::PProposalVote,
				ipfs_project_proposal_documentation,
				false
			),
			Error::<Test>::VoteAlreadySubmitted
		);
	});
}

#[test]
fn cast_vote_pproposal_voting_cycle_is_over_submitted() {
	new_test_ext().execute_with(|| {
		// Create project proposal IPFS link
		let ipfs_project_proposal_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_project_proposal_documentation");

		// Create project validator info
		let project_validator_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from(
				"ipfs_project_validator_documentation",
			),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Create project hash
		let nonce = frame_system::Pallet::<Test>::account_nonce(alice());
		let encoded: [u8; 32] = (alice(), nonce).using_encoded(blake2_256);
		let project_hash = H256::from(encoded);

		// Create project proposal info
		let project_proposal_info = ProjectProposalInfo {
			project_owner: bob(),
			creation_date: 0,
			project_hash,
			votes_for: BTreeSet::<AccountId>::new(),
			votes_against: BTreeSet::<AccountId>::new(),
			voting_active: false,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project validator
		ProjectValidators::<Test>::insert(alice(), project_validator_info);

		// Insert project proposal info
		ProjectProposals::<Test>::insert(
			ipfs_project_proposal_documentation.clone(),
			project_proposal_info,
		);

		// Check for VotingCycleIsOver error
		assert_err!(
			Veles::cast_vote(
				RuntimeOrigin::signed(alice()),
				VoteType::PProposalVote,
				ipfs_project_proposal_documentation,
				false
			),
			Error::<Test>::VotingCycleIsOver
		);
	});
}

#[test]
fn cast_vote_ccbatch_ccb_proposal_not_found() {
	new_test_ext().execute_with(|| {
		// Create CC batch proposal IPFS link
		let ipfs_ccbatch_proposal_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_ccbatch_proposal_documentation");

		// Create project validator info
		let project_validator_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from(
				"ipfs_project_validator_documentation",
			),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project validator
		ProjectValidators::<Test>::insert(alice(), project_validator_info);

		// Check for CCBProposalNotFound error
		assert_err!(
			Veles::cast_vote(
				RuntimeOrigin::signed(alice()),
				VoteType::CCBatchVote,
				ipfs_ccbatch_proposal_documentation,
				false
			),
			Error::<Test>::CCBProposalNotFound
		);
	});
}

#[test]
fn cast_vote_ccbatch_ok() {
	new_test_ext().execute_with(|| {
		// Create CC batch proposal IPFS link
		let ipfs_ccbatch_proposal_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_ccbatch_proposal_documentation");

		// Create project validator info
		let project_validator_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from(
				"ipfs_project_validator_documentation",
			),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Create CC batch and project hash
		let nonce = frame_system::Pallet::<Test>::account_nonce(bob());
		let encoded: [u8; 32] = (bob(), nonce).using_encoded(blake2_256);
		let hash = H256::from(encoded);

		// Create CC batch proposal info
		let ccb_proposal_info = CCBProposalInfo {
			project_hash: hash,
			batch_hash: hash,
			creation_date: 0,
			credit_amount: 0,
			initial_credit_price: 0,
			votes_for: BTreeSet::<AccountId>::new(),
			votes_against: BTreeSet::<AccountId>::new(),
			voting_active: true,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project validator
		ProjectValidators::<Test>::insert(alice(), project_validator_info);

		// Insert CC batch proposal info
		CCBProposals::<Test>::insert(
			ipfs_ccbatch_proposal_documentation.clone(),
			ccb_proposal_info,
		);

		// Vote succesfully
		assert_ok!(Veles::cast_vote(
			RuntimeOrigin::signed(alice()),
			VoteType::CCBatchVote,
			ipfs_ccbatch_proposal_documentation,
			false
		));
	});
}

#[test]
fn cast_vote_ccbatch_vote_already_submitted() {
	new_test_ext().execute_with(|| {
		// Create CC batch proposal IPFS link
		let ipfs_ccbatch_proposal_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_ccbatch_proposal_documentation");

		// Create project validator info
		let project_validator_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from(
				"ipfs_project_validator_documentation",
			),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Create CC batch hash
		let nonce = frame_system::Pallet::<Test>::account_nonce(bob());
		let encoded: [u8; 32] = (bob(), nonce).using_encoded(blake2_256);
		let batch_hash = H256::from(encoded);

		// Create CC batch proposal info
		let ccb_proposal_info = CCBProposalInfo {
			project_hash: batch_hash,
			batch_hash,
			creation_date: 0,
			credit_amount: 0,
			initial_credit_price: 0,
			votes_for: BTreeSet::<AccountId>::new(),
			votes_against: BTreeSet::<AccountId>::new(),
			voting_active: true,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project validator
		ProjectValidators::<Test>::insert(alice(), project_validator_info);

		// Insert CC batch proposal info
		CCBProposals::<Test>::insert(
			ipfs_ccbatch_proposal_documentation.clone(),
			ccb_proposal_info,
		);

		// Vote succesfully
		assert_ok!(Veles::cast_vote(
			RuntimeOrigin::signed(alice()),
			VoteType::CCBatchVote,
			ipfs_ccbatch_proposal_documentation.clone(),
			false
		));

		// Check for VoteAlreadySubmitted error
		assert_err!(
			Veles::cast_vote(
				RuntimeOrigin::signed(alice()),
				VoteType::CCBatchVote,
				ipfs_ccbatch_proposal_documentation,
				false
			),
			Error::<Test>::VoteAlreadySubmitted
		);
	});
}

#[test]
fn cast_vote_ccbatch_voting_cycle_is_over() {
	new_test_ext().execute_with(|| {
		// Create CC batch proposal IPFS link
		let ipfs_ccbatch_proposal_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_ccbatch_proposal_documentation");

		// Create project validator info
		let project_validator_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from(
				"ipfs_project_validator_documentation",
			),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Create CC batch hash
		let nonce = frame_system::Pallet::<Test>::account_nonce(bob());
		let encoded: [u8; 32] = (bob(), nonce).using_encoded(blake2_256);
		let batch_hash = H256::from(encoded);

		// Create CC batch proposal info
		let ccb_proposal_info = CCBProposalInfo {
			project_hash: batch_hash,
			batch_hash,
			creation_date: 0,
			credit_amount: 0,
			initial_credit_price: 0,
			votes_for: BTreeSet::<AccountId>::new(),
			votes_against: BTreeSet::<AccountId>::new(),
			voting_active: false,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project validator
		ProjectValidators::<Test>::insert(alice(), project_validator_info);

		// Insert CC batch proposal info
		CCBProposals::<Test>::insert(
			ipfs_ccbatch_proposal_documentation.clone(),
			ccb_proposal_info,
		);

		// Check for VotingCycleIsOver error
		assert_err!(
			Veles::cast_vote(
				RuntimeOrigin::signed(alice()),
				VoteType::CCBatchVote,
				ipfs_ccbatch_proposal_documentation,
				false
			),
			Error::<Test>::VotingCycleIsOver
		);
	});
}

#[test]
fn propose_project_unauthorized() {
	new_test_ext().execute_with(|| {
		// Create project proposal IPFS link
		let ipfs_project_proposal_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_project_proposal_documentation");

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Check for Unauthorized error
		assert_err!(
			Veles::propose_project(
				RuntimeOrigin::signed(alice()),
				ipfs_project_proposal_documentation,
			),
			Error::<Test>::Unauthorized
		);
	});
}

#[test]
fn propose_project_ok() {
	new_test_ext().execute_with(|| {
		// Create project proposal IPFS link
		let ipfs_project_proposal_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_project_proposal_documentation");

		// Create project owner info
		let project_owner_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from(
				"ipfs_project_owner_documentation",
			),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project owner
		ProjectOwners::<Test>::insert(alice(), project_owner_info);

		// Propose project succesfully
		assert_ok!(Veles::propose_project(
			RuntimeOrigin::signed(alice()),
			ipfs_project_proposal_documentation,
		));
	});
}

#[test]
fn propose_project_project_proposal_already_exists() {
	new_test_ext().execute_with(|| {
		// Create project proposal IPFS link
		let ipfs_project_proposal_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_project_proposal_documentation");

		// Create project owner info
		let project_owner_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from(
				"ipfs_project_owner_documentation",
			),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project owner
		ProjectOwners::<Test>::insert(alice(), project_owner_info);

		// Propose project succesfully
		assert_ok!(Veles::propose_project(
			RuntimeOrigin::signed(alice()),
			ipfs_project_proposal_documentation.clone(),
		));

		// Check for ProjectProposalAlreadyExists error
		assert_err!(
			Veles::propose_project(
				RuntimeOrigin::signed(alice()),
				ipfs_project_proposal_documentation,
			),
			Error::<Test>::ProjectProposalAlreadyExists
		);
	});
}

#[test]
fn propose_carbon_credit_batch_unauthorized() {
	new_test_ext().execute_with(|| {
		// Create CC batch proposal IPFS link
		let ipfs_ccbatch_proposal_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_ccbatch_proposal_documentation");

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Create project hash
		let nonce = frame_system::Pallet::<Test>::account_nonce(bob());
		let encoded: [u8; 32] = (bob(), nonce).using_encoded(blake2_256);
		let hash = H256::from(encoded);

		// Check for Unauthorized error
		assert_err!(
			Veles::propose_carbon_credit_batch(
				RuntimeOrigin::signed(alice()),
				hash,
				0,
				0,
				ipfs_ccbatch_proposal_documentation,
			),
			Error::<Test>::Unauthorized
		);
	});
}

#[test]
fn propose_carbon_credit_batch_project_doesnt_exist() {
	new_test_ext().execute_with(|| {
		// Create CC batch proposal IPFS link
		let ipfs_ccbatch_proposal_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_ccbatch_proposal_documentation");

		// Create project owner info
		let project_owner_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from(
				"ipfs_project_owner_documentation",
			),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project owner
		ProjectOwners::<Test>::insert(alice(), project_owner_info);

		// Create project hash
		let nonce = frame_system::Pallet::<Test>::account_nonce(bob());
		let encoded: [u8; 32] = (bob(), nonce).using_encoded(blake2_256);
		let hash = H256::from(encoded);

		// Check for Unauthorized error
		assert_err!(
			Veles::propose_carbon_credit_batch(
				RuntimeOrigin::signed(alice()),
				hash,
				0,
				0,
				ipfs_ccbatch_proposal_documentation,
			),
			Error::<Test>::ProjectDoesntExist
		);
	});
}

#[test]
fn propose_carbon_credit_batch_unauthorized_project_owner() {
	new_test_ext().execute_with(|| {
		// Create CC batch proposal IPFS link
		let ipfs_ccbatch_proposal_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_ccbatch_proposal_documentation");
		// Create project proposal IPFS link
		let ipfs_project_proposal_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_project_proposal_documentation");

		// Create project owner info (ALICE)
		let project_owner_info_1 = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from(
				"ipfs_project_owner_documentation_1",
			),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Create project owner info (BOB)
		let project_owner_info_2 = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from(
				"ipfs_project_owner_documentation_2",
			),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project owner
		ProjectOwners::<Test>::insert(alice(), project_owner_info_1);
		ProjectOwners::<Test>::insert(bob(), project_owner_info_2);

		// Create project hash
		let nonce = frame_system::Pallet::<Test>::account_nonce(alice());
		let encoded: [u8; 32] = (alice(), nonce).using_encoded(blake2_256);
		let project_hash = H256::from(encoded);

		// Create project proposal info
		let project_proposal_info = ProjectProposalInfo {
			project_owner: alice(),
			creation_date: 0,
			project_hash,
			votes_for: BTreeSet::<AccountId>::new(),
			votes_against: BTreeSet::<AccountId>::new(),
			voting_active: true,
		};

		// Insert project proposal info
		ProjectProposals::<Test>::insert(
			ipfs_project_proposal_documentation.clone(),
			project_proposal_info,
		);

		// Create project info
		let project_info = ProjectInfo {
			documentation_ipfs: ipfs_project_proposal_documentation.clone(),
			creation_date: 0,
			penalty_level: 0,
			penalty_timeout: 0,
		};

		Projects::<Test>::insert(project_hash, project_info);

		// Check for Unauthorized error (Other project owner)
		assert_err!(
			Veles::propose_carbon_credit_batch(
				RuntimeOrigin::signed(bob()),
				project_hash,
				0,
				0,
				ipfs_ccbatch_proposal_documentation,
			),
			Error::<Test>::Unauthorized
		);
	});
}

#[test]
fn propose_carbon_credit_batch_ok() {
	new_test_ext().execute_with(|| {
		// Create CC batch proposal IPFS link
		let ipfs_ccbatch_proposal_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_ccbatch_proposal_documentation");
		// Create project proposal IPFS link
		let ipfs_project_proposal_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_project_proposal_documentation");

		// Create project owner info
		let project_owner_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from(
				"ipfs_project_owner_documentation",
			),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project owner
		ProjectOwners::<Test>::insert(alice(), project_owner_info);

		// Create project hash
		let nonce = frame_system::Pallet::<Test>::account_nonce(alice());
		let encoded: [u8; 32] = (alice(), nonce).using_encoded(blake2_256);
		let project_hash = H256::from(encoded);

		// Create project proposal info
		let project_proposal_info = ProjectProposalInfo {
			project_owner: alice(),
			creation_date: 0,
			project_hash,
			votes_for: BTreeSet::<AccountId>::new(),
			votes_against: BTreeSet::<AccountId>::new(),
			voting_active: true,
		};

		// Insert project proposal info
		ProjectProposals::<Test>::insert(
			ipfs_project_proposal_documentation.clone(),
			project_proposal_info,
		);

		// Create project info
		let project_info = ProjectInfo {
			documentation_ipfs: ipfs_project_proposal_documentation.clone(),
			creation_date: 0,
			penalty_level: 0,
			penalty_timeout: 0,
		};

		Projects::<Test>::insert(project_hash, project_info);

		// Propose carbon credit batch succesfully
		assert_ok!(Veles::propose_carbon_credit_batch(
			RuntimeOrigin::signed(alice()),
			project_hash,
			0,
			0,
			ipfs_ccbatch_proposal_documentation,
		));
	});
}

#[test]
fn propose_project_documentation_was_used_previously() {
	new_test_ext().execute_with(|| {
		// Create project proposal IPFS link
		let ipfs_documentation = BoundedString::<IPFSLength>::truncate_from("ipfs_documentation");

		// Create project owner info
		let project_owner_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from("ipfs_documentation"),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project owner
		ProjectOwners::<Test>::insert(alice(), project_owner_info);

		// Check for DocumentationWasUsedPreviously error
		assert_err!(
			Veles::propose_project(RuntimeOrigin::signed(alice()), ipfs_documentation),
			Error::<Test>::DocumentationWasUsedPreviously
		);
	});
}

#[test]
fn propose_carbon_credit_batch_documentation_was_used_previously() {
	new_test_ext().execute_with(|| {
		// Create IPFS link
		let ipfs_documentation = BoundedString::<IPFSLength>::truncate_from("ipfs_documentation");

		// Create project owner info
		let project_owner_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from("ipfs_documentation"),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Go past genesis block so events get deposited
		run_to_block(1);

		// Insert project owner
		ProjectOwners::<Test>::insert(alice(), project_owner_info);

		// Create project hash
		let nonce = frame_system::Pallet::<Test>::account_nonce(alice());
		let encoded: [u8; 32] = (alice(), nonce).using_encoded(blake2_256);
		let project_hash = H256::from(encoded);

		// Create project proposal info
		let project_proposal_info = ProjectProposalInfo {
			project_owner: alice(),
			creation_date: 0,
			project_hash,
			votes_for: BTreeSet::<AccountId>::new(),
			votes_against: BTreeSet::<AccountId>::new(),
			voting_active: true,
		};

		// Insert project proposal info
		ProjectProposals::<Test>::insert(ipfs_documentation.clone(), project_proposal_info);

		// Create project info
		let project_info = ProjectInfo {
			documentation_ipfs: ipfs_documentation.clone(),
			creation_date: 0,
			penalty_level: 0,
			penalty_timeout: 0,
		};

		Projects::<Test>::insert(project_hash, project_info);

		// Check for DocumentationWasUsedPreviously error
		assert_err!(
			Veles::propose_carbon_credit_batch(
				RuntimeOrigin::signed(alice()),
				project_hash,
				0,
				0,
				ipfs_documentation,
			),
			Error::<Test>::DocumentationWasUsedPreviously
		);
	});
}
