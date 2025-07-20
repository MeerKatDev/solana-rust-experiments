use borsh::{BorshDeserialize, BorshSerialize};
// use solana_program_test::*;
use mollusk_svm::{Mollusk, result::Check};
use solana_sdk::{
    account::Account, 
    // entrypoint::ProgramResult, 
    instruction::{AccountMeta, Instruction}, 
    pubkey::Pubkey,
    signature::Keypair, 
    signer::Signer, 
    // transaction::Transaction,
};

use calculator::{Calculator, CalculatorInstructions};

#[test]
fn test_addition_instruction() {
    let program_id = Pubkey::new_unique();
    let calc_account = Keypair::new();

    // Set up initial calculator state
    let calculator = Calculator { value: 10 };
    let mut calc_data = vec![];
    calculator.serialize(&mut calc_data).unwrap();

    // Build test environment
    // let mut program_test = ProgramTest::new(
    //     "calculator",
    //     program_id,
    //     processor!(entry), // entrypoint
    // );
    let mollusk = Mollusk::new(&program_id, "target/deploy/calculator");

    // program_test.add_account(
    //     calc_account.pubkey(),
    //     Account {
    //         lamports: 1_000_000,
    //         data: calc_data,
    //         owner: program_id,
    //         executable: false,
    //         rent_epoch: 0,
    //     },
    // );
    let user_account = Account {
        lamports: 1_000_000,
        data: calc_data,
        owner: program_id,
        executable: false,
        rent_epoch: 0,
    };

    let accounts = vec![(calc_account.pubkey(), user_account)];

    // let (banks_client, payer, recent_blockhash) = program_test.start().await;

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
        vec![AccountMeta::new(calc_account.pubkey(), false)],
    );

    // NOTE: Mollusk doesn't work with Transactions

    // let tx = Transaction::new_signed_with_payer(
    //     &[instruction],
    //     Some(&payer.pubkey()),
    //     &[&payer],
    //     recent_blockhash,
    // );

    let checks = vec![
        Check::success()
    ];

    // banks_client.process_transaction(tx).await.unwrap();
    let result = mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);

    // Fetch and verify updated account
    // let account = banks_client
    //     .get_account(calc_account.pubkey())
    //     .await
    //     .unwrap()
    //     .unwrap();
    let updated_account_data = &result.resulting_accounts[0].1.data;
    // deserializing into a new counter struct for readability 
    // (we could have used the binary data though)
    let updated_counter = Calculator::try_from_slice(updated_account_data).unwrap();
    // confirming that the counter was incremented by 1
    // so the transaction was executed.
    assert_eq!(updated_counter.value, 15);
}
