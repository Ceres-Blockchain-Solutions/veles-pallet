use crate::{mock::*, Error};
use frame_support::{assert_err, assert_ok};

#[test]
fn cast_vote_unauthorized() {
	new_test_ext().execute_with(|| {
		// Create IPFS documentation link
		let ipfs_documentation = BoundedString::<IPFSLength>::truncate_from("ipfs_documentation");

		// Go past genesis block so events get deposited
		System::set_block_number(1);

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
		System::set_block_number(1);

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
			carbon_deficit: 0,
			votes_for: BTreeSet::<AccountId>::new(),
			votes_against: BTreeSet::<AccountId>::new(),
		};

		// Go past genesis block so events get deposited
		System::set_block_number(1);

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
			carbon_deficit: 0,
			votes_for: BTreeSet::<AccountId>::new(),
			votes_against: BTreeSet::<AccountId>::new(),
		};

		// Go past genesis block so events get deposited
		System::set_block_number(1);

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
		System::set_block_number(1);

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
		};

		// Go past genesis block so events get deposited
		System::set_block_number(1);

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
		};

		// Go past genesis block so events get deposited
		System::set_block_number(1);

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
		System::set_block_number(1);

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
		};

		// Go past genesis block so events get deposited
		System::set_block_number(1);

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
		};

		// Go past genesis block so events get deposited
		System::set_block_number(1);

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
fn propose_project_unauthorized() {
	new_test_ext().execute_with(|| {
		// Create project proposal IPFS link
		let ipfs_project_proposal_documentation =
			BoundedString::<IPFSLength>::truncate_from("ipfs_project_proposal_documentation");

		// Go past genesis block so events get deposited
		System::set_block_number(1);

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
		System::set_block_number(1);

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
		System::set_block_number(1);

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
		System::set_block_number(1);

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
		System::set_block_number(1);

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
		System::set_block_number(1);

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
		System::set_block_number(1);

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
		System::set_block_number(1);

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
