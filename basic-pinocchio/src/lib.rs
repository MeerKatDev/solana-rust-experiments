use pinocchio::{
  account_info::AccountInfo,
  entrypoint,
  // msg,
  ProgramResult,
  pubkey::Pubkey
};

entrypoint!(process_instruction);

pub fn process_instruction(
  _program_id: &Pubkey,
  _accounts: &[AccountInfo],
  _instruction_data: &[u8],
) -> ProgramResult {
  // msg!("Hello from my program!");
  Ok(())
}