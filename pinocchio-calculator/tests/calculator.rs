use borsh::{BorshDeserialize, BorshSerialize};
use pinocchio::account_info::AccountInfo;
use solana_program_test::*;
use solana_sdk::entrypoint::__AccountInfo;
use solana_sdk::{
    account::Account, entrypoint::ProgramResult, instruction::Instruction, pubkey::Pubkey,
    signature::Keypair, signer::Signer, transaction::Transaction,
};

use calculator::{Calculator, CalculatorInstructions};

/// This is a wrapper to get the processor macro to work.
fn entry(
    program_id: &Pubkey,
    accounts: &[__AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // let accounts_unboxed = accounts.to_vec().into_iter().map(|a| AccountInfo {
    //     key: a.key
    //     // lamports:
    //     // data: Rc<RefCell<&'a mut [u8]>>,
    //     // owner: &'a Pubkey,
    //     // rent_epoch: u64,
    //     // is_signer: bool,
    //     // is_writable: bool,
    //     // executable: bool,
    // }).collect();
    let accounts1 = Box::leak(Box::new(&accounts));

    calculator::process_instruction(program_id.as_array(), accounts1, instruction_data)
        // there must be a better mapping
        .map_err(|_e| solana_sdk::program_error::ProgramError::InvalidArgument)
}

#[tokio::test]
async fn test_addition_instruction() {
    let program_id = Pubkey::new_unique();
    let calc_account = Keypair::new();

    // Set up initial calculator state
    let calculator = Calculator { value: 10 };
    let mut calc_data = vec![];
    calculator.serialize(&mut calc_data).unwrap();

    // Build test environment
    let mut program_test = ProgramTest::new(
        "calculator",
        program_id,
        processor!(entry), // entrypoint
    );

    program_test.add_account(
        calc_account.pubkey(),
        Account {
            lamports: 1_000_000,
            data: calc_data,
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    );

    let (banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create CalculatorInstructions
    let instr = CalculatorInstructions {
        operation: 1, // ADD
        operating_value: 5,
    };

    let mut instr_data = vec![];
    instr.serialize(&mut instr_data).unwrap();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &instr_data,
        vec![solana_sdk::instruction::AccountMeta::new(
            calc_account.pubkey(),
            false,
        )],
    );

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    banks_client.process_transaction(tx).await.unwrap();

    // Fetch and verify updated account
    let account = banks_client
        .get_account(calc_account.pubkey())
        .await
        .unwrap()
        .unwrap();

    let updated_calc = Calculator::try_from_slice(&account.data).unwrap();
    assert_eq!(updated_calc.value, 15);
}
