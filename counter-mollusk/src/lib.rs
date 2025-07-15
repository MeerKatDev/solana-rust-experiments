#![allow(unexpected_cfgs)]

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct Counter {
    pub count: u64,
}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if !account.is_writable {
        msg!("Account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    let mut counter_data = Counter::try_from_slice(&account.data.borrow())?;
    counter_data.count += 1;
    let mut data_ref = account.data.borrow_mut();
    counter_data.serialize(&mut data_ref.as_mut())?;

    msg!("Counter incremented to {}", counter_data.count);
    Ok(())
}