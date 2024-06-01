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
fn cast_vote_pproposal_vote_ok() {
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

/*
#[test]
fn vote_for_project_proposal_not_found() {
	new_test_ext().execute_with(|| {
		let ipfs = H256::zero();
		let pv_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from("ipfs_link"),
			penalty_level: 0,
			penalty_timeout: 0,
		};

		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Create project validator
		ProjectValidators::<Test>::insert(bob(), pv_info);

		assert_err!(
			Veles::cast_vote(RuntimeOrigin::signed(bob()), VoteType::ProposalVote, ipfs, false),
			Error::<Test>::ProjectProposalNotFound
		);
	});
}

#[test]
fn vote_for_project_proposal_ok() {
	new_test_ext().execute_with(|| {
		let ipfs = H256::zero();
		let pv_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from("ipfs_link"),
			penalty_level: 0,
			penalty_timeout: 0,
		};
		let project_proposal_info = PProposalInfo {
			project_owner: bob(),
			creation_date: 0,
			project_hash: ipfs,
			votes_for: BTreeSet::<AccountId>::new(),
			votes_against: BTreeSet::<AccountId>::new(),
		};

		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Create project proposal
		ProjectProposals::<Test>::insert(ipfs, project_proposal_info);
		// Create project validator
		ProjectValidators::<Test>::insert(bob(), pv_info);

		assert_ok!(Veles::cast_vote(
			RuntimeOrigin::signed(bob()),
			VoteType::ProposalVote,
			ipfs,
			false
		));
	});
}

#[test]
fn vote_for_cdr_ok() {
	new_test_ext().execute_with(|| {
		let ipfs = H256::zero();
		let pv_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from("ipfs_link"),
			penalty_level: 0,
			penalty_timeout: 0,
		};
		let cdr_info = CFReportInfo {
			account_id: alice(),
			creation_date: 0,
			carbon_deficit: 0,
			votes_for: BTreeSet::<AccountId>::new(),
			votes_against: BTreeSet::<AccountId>::new(),
		};

		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Create project proposal
		CarbonDeficitReports::<Test>::insert(ipfs, cdr_info);
		// Create project validator
		ProjectValidators::<Test>::insert(bob(), pv_info);

		assert_ok!(Veles::cast_vote(RuntimeOrigin::signed(bob()), VoteType::CdrVote, ipfs, false));
	});
}

#[test]
fn project_proposal_ok() {
	new_test_ext().execute_with(|| {
		let ipfs = H256::zero();
		let pv_po_info = PVoPOInfo {
			documentation_ipfs: BoundedString::<IPFSLength>::truncate_from("ipfs_link"),
			penalty_level: 0,
			penalty_timeout: 0,
		};
		let project_proposal_info = PProposalInfo {
			project_owner: bob(),
			creation_date: 0,
			project_hash: ipfs,
			votes_for: BTreeSet::<AccountId>::new(),
			votes_against: BTreeSet::<AccountId>::new(),
		};

		// Go past genesis block so events get deposited
		System::set_block_number(1);

		ProjectOwners::<Test>::insert(bob(), pv_po_info);

		// Create project proposal
		assert_ok!(Veles::propose_project(RuntimeOrigin::signed(bob()), ipfs));

		// Assert project proposal owner account equal to project_owner account
		assert_eq!(bob(), ProjectProposals::<Test>::get(ipfs).unwrap().project_owner);
	});
}

#[test]
fn project_proposal_not_authorized() {
	new_test_ext().execute_with(|| {
		let ipfs = H256::zero();
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Create project proposal
		assert_err!(
			Veles::propose_project(RuntimeOrigin::signed(alice()), ipfs),
			Error::<Test>::NotAuthorized
		);
	});
}
*/
