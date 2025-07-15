use borsh::{BorshDeserialize, BorshSerialize};
use litesvm::LiteSVM;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    signer::keypair::Keypair,
    transaction::Transaction,
};

use solana_account::Account;
use solana_message::Message;

use counter_litesvm::Counter;

#[test]
fn test_counter_increment() {
    let program_id = Pubkey::new_unique();
    let counter_pda = Pubkey::new_unique();

    // random number to start the count from.
    let count = 10;

    let instruction_off = Counter { count };
    let counter_data = borsh::to_vec(&instruction_off).unwrap();
    let mut svm = LiteSVM::new();
    let payer = Keypair::new();
    let bytes = include_bytes!("../target/deploy/counter_litesvm.so");

    svm.add_program(program_id, bytes);
    let lamports = 1_000_000_000;
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let ix = Instruction::new_with_borsh(
        program_id,
        &instruction_off,
        vec![AccountMeta::new(counter_pda, false)],
    );

    // Use set_account to register counter_pda
    svm.set_account(
        counter_pda,
        Account {
            lamports: 1_000_000,
            data: counter_data,
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();

    let blockhash = svm.latest_blockhash();

    let message = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);

    let mut tx = Transaction::new_unsigned(message);
    tx.sign(&[payer], blockhash);

    // let's sim it first
    let sim_res = svm.simulate_transaction(tx.clone()).unwrap();
    let meta = svm.send_transaction(tx).unwrap();

    let raw_account = svm.get_account(&counter_pda).unwrap();
    println!("raw_account {:?}", raw_account);

    assert_eq!(sim_res.meta, meta);
    assert!(meta.compute_units_consumed < 10_000);

    let updated_counter = Counter::try_from_slice(&raw_account.data).unwrap();

    assert_eq!(updated_counter.count, 11);
}
