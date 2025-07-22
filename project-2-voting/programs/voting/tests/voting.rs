use borsh::{BorshDeserialize, BorshSerialize};
use mollusk_svm::{result::Check, sysvar::Sysvars, Mollusk};
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use std::convert::TryInto;
use voting::accounts::PollAccount;
use voting::VotingInstruction;

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
        name: pad_string_null("Who's a good boy".to_string(), PollAccount::MAX_NAME_LENGTH),
        description: pad_string_null(
            "Poll to determine who's a good boy".to_string(),
            PollAccount::MAX_DESCRIPTION_LENGTH,
        ),
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

    let poll_account_object = PollAccount::default();

    let mut poll_account_init_data = vec![0u8; 0];

    poll_account_object
        .serialize(&mut &mut poll_account_init_data)
        .unwrap();

    let poll_account = Account {
        lamports: 1_000_000,
        data: poll_account_init_data,
        owner: program_id,
        executable: false,
        rent_epoch: 0,
    };

    let accounts = vec![(poll_account_pubkey, poll_account)];

    let checks = vec![Check::success()];

    let result = mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
    let (poll_account_pubkey2, updated_poll_account) = &result.resulting_accounts[0];
    assert_eq!(poll_account_pubkey, *poll_account_pubkey2);

    let new_poll_account = PollAccount::try_from_slice(&updated_poll_account.data).unwrap();
    assert_eq!(new_poll_account.poll_option_index, 0);
}

fn pad_string_null(mut s: String, len: usize) -> String {
    if s.len() > len {
        s.truncate(len);
    } else {
        s.push_str(&"\0".repeat(len - s.len()));
    }
    s
}
