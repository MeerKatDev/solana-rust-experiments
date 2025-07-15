#![allow(unexpected_cfgs)]

use borsh::{BorshDeserialize, BorshSerialize};
use pinocchio::{
    ProgramResult, account_info::AccountInfo, entrypoint, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct Counter {
    pub count: u64,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // difference from solana-program
    let account = &accounts[0];

    if !account.is_writable() {
        msg!("Account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    // difference
    let mut data = account.try_borrow_mut_data().unwrap();
    let mut counter = Counter::try_from_slice(&data).map_err(|_e| ProgramError::Custom(1))?;
    counter.count += 1;

    // difference?
    counter
        .serialize(&mut &mut data[..])
        .map_err(|_e| ProgramError::Custom(1))?;

    // difference: pinocchio is no_std, so it can't directly format
    msg!(format!("Counter incremented to {}", counter.count).as_str());
    Ok(())
}
