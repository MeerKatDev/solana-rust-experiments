use mollusk_svm::Mollusk;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use anchor_counter::instruction::Increment;
use anchor_counter::instruction::Initialize;
use anchor_counter::Counter;
use anchor_lang::{AnchorSerialize, AnchorDeserialize};
// use anchor_lang::Discriminator;
use anchor_counter::accounts::Initialize as InitializeAccounts;

// use borsh::{BorshSerialize, BorshDeserialize};
use anchor_counter::ID;
use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;

#[test]
fn test_counter_increment() {
    let program_id = ID;
    let counter_pda = Pubkey::new_unique();
    let payer = Pubkey::new_unique();
    let initial_count: u64 = 10;
    // Serialize instruction data — this should be the *arguments* sent to your program.
    let counter = Counter { count: initial_count };

    let ix_data = Increment { }.data();

    let accounts = InitializeAccounts {
	        counter: counter_pda,
	        user: payer,
	        system_program: program_id,
	    }
	    .to_account_metas(None);

	let ix = Instruction {
	    program_id,
	    accounts,
	    data: Initialize {}.data(),
	};

	// 2. Serialize Counter + discriminator into a data buffer
	let mut counter_data = vec![0u8; 8];
	counter_data.extend_from_slice(&counter.try_to_vec().unwrap());  // serialized Counter

	// let anchor_account = Account {
	//     lamports: 0,
	//     data: counter_data.clone(),
	//     owner: program_id,  // your program's Pubkey
	//     executable: false,
	//     rent_epoch: 0,
	// };

    let instruction_data = counter.try_to_vec().unwrap();

    // AnchorSerialize::serialize(&initial_count, &mut cursor).unwrap();
    counter.serialize(&mut &mut counter_data[8..]).unwrap();

    // Create account owned by the program with initial data
    let mut account1 = Account::default();
    account1.owner = program_id;
    account1.data = counter_data;

    // let accounts = vec![(counter_pda, account1)];

    // Instruction with the program id, instruction data, and accounts
    // let ix = Instruction {
    //     program_id,
    //     accounts: vec![AccountMeta::new(counter_pda, false)],
    //     data: instruction_data,
    // };

    // Create Mollusk runtime
    let mollusk = Mollusk::new(&program_id, "../../target/deploy/anchor_counter");

    // Process instruction with initial accounts
    let result1 = mollusk.process_instruction(&ix, &accounts);
    // let result2 = mollusk.process_instruction(&ix_data, &accounts);

    // Deserialize updated data after execution
    let updated_counter_data1 = &result1.resulting_accounts[0].1.data;
    println!("updated_counter_data: {:?}", &updated_counter_data1[8..]);

    // let updated_counter_data2 = &result2.resulting_accounts[0].1.data;
    // println!("updated_counter_data: {:?}", &updated_counter_data2[8..]);

    // The data includes discriminator + count, skip discriminator to deserialize Counter
    let updated_counter = Counter::try_from_slice(&updated_counter_data1[8..]).unwrap();

    // Counter should be incremented by 1
    assert_eq!(updated_counter.count, initial_count + 1);
}
