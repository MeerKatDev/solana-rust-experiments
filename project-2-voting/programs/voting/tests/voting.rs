use borsh::{BorshDeserialize, BorshSerialize};
use voting::VotingInstruction;
use pinocchio::instruction::Instruction;
use pinocchio::pubkey::Pubkey;

    // unique program id
#[test]
fn test_initialize_poll() {
    let program_id = generate_pubkey();
    let a = "";

    let instr = Instruction { program_id, data, accounts };

    let mollusk = Mollusk::new(&program_id, "target/deploy/voting");

}

fn generate_pubkey() {
    let mut random_bytes = [0u8; 32];
    getrandom(&mut random_bytes).expect("random generation failed");
    vec!(random_bytes)
}