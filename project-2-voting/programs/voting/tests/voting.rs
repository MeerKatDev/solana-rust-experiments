use borsh::{BorshDeserialize, BorshSerialize};
use mollusk_svm::{result::Check, sysvar::Sysvars, Mollusk};
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use std::convert::TryInto;
use voting::{
    accounts::{CandidateAccount, PollAccount},
    ixs::VotingInstruction,
};

#[test]
fn test_initialize_poll() {
    let program_id = Pubkey::new_unique();
    let mollusk = Mollusk::new(&program_id, "target/deploy/voting");
    let poll_account_pubkey = Pubkey::new_unique();
    let sys_clock = Sysvars::default().clock;
    let current_time = sys_clock
        .unix_timestamp
        .try_into()
        .expect("should be positive");

    let initialize_poll_ix = VotingInstruction::InitializePoll {
        poll_id: 42,
        name: PollAccount::checked_name("Who's a good boy"),
        description: PollAccount::checked_desc("Poll to determine who's a good boy"),
        start_time: current_time,
        end_time: current_time + 10_000u64, // 10 seconds?
    };

    let mut instr_data = vec![];
    initialize_poll_ix.serialize(&mut instr_data).unwrap();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &instr_data,
        vec![AccountMeta::new(poll_account_pubkey, false)],
    );

    // assume the account already exist --
    let account_object = PollAccount::default();
    let poll_account = make_fake_account(&program_id, account_object);

    let accounts = vec![(poll_account_pubkey, poll_account)];

    let checks = vec![Check::success()];

    let result = mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
    let (poll_account_pubkey2, updated_poll_account) = &result.resulting_accounts[0];
    assert_eq!(poll_account_pubkey, *poll_account_pubkey2);

    let new_poll_account = PollAccount::try_from_slice(&updated_poll_account.data).unwrap();
    assert_eq!(new_poll_account.poll_option_index, 0);
}

#[test]
fn test_initialize_candidate() {
    let program_id = Pubkey::new_unique();
    let mollusk = Mollusk::new(&program_id, "target/deploy/voting");
    let poll_account_pubkey = Pubkey::new_unique();
    let cand_account_pubkey = Pubkey::new_unique();

    let initialize_cand_ix = VotingInstruction::InitializeCandidate {
        poll_id: 42,
        candidate: CandidateAccount::checked_name("A good boy"),
    };

    let mut instr_data = vec![];
    initialize_cand_ix.serialize(&mut instr_data).unwrap();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &instr_data,
        vec![
            AccountMeta::new(cand_account_pubkey, false),
            AccountMeta::new(poll_account_pubkey, false),
        ],
    );

    // TODO: these two accounts shouldn't be present when doing this.
    // Create account should be called inside the instruction itself.
    let account_object = CandidateAccount::default();
    let cand_account = make_fake_account(&program_id, account_object);

    let account_object = PollAccount::default();
    let poll_account = make_fake_account(&program_id, account_object);

    let accounts = vec![
        (cand_account_pubkey, cand_account),
        (poll_account_pubkey, poll_account),
    ];

    let checks = vec![Check::success()];

    let result = mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
    let (_cand_account_pubkey, updated_cand_account) = &result.resulting_accounts[0];
    let (_poll_account_pubkey, updated_poll_account) = &result.resulting_accounts[1];

    let new_cand_account = CandidateAccount::try_from_slice(&updated_cand_account.data).unwrap();
    let new_poll_account = PollAccount::try_from_slice(&updated_poll_account.data).unwrap();
    assert_eq!(new_cand_account.candidate_name, "A good boy\0\0\0\0\0\0");
    assert_eq!(new_poll_account.poll_option_index, 1);
}

// test utils

fn make_fake_account<S: BorshSerialize>(program_id: &Pubkey, object: S) -> Account {
    let mut init_data = vec![0u8; 0];

    object.serialize(&mut &mut init_data).unwrap();

    // TODO it should not actually exist
    Account {
        lamports: 1_000_000,
        data: init_data,
        owner: *program_id,
        executable: false,
        rent_epoch: 0,
    }
}
