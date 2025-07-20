#![allow(unexpected_cfgs)]

use borsh::{BorshDeserialize, BorshSerialize};
use pinocchio::{
    account_info::{next_account_info, AccountInfo}, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program_error::ProgramError,
    pubkey::Pubkey,
};
pub use crate::calculator::CalculatorInstructions;

mod calculator;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Calculator {
    pub value: u32,
}


entrypoint!(process_instruction);


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut calc = Calculator::try_from(&account.data.borrow())?;

    let calculator_instructions = CalculatorInstructions::try_from(&instruction_data)?;

    calc.value = calculator_instructions.evaluate(calc.value);

    calc.serialize(&mut &mut account.data.borrow_mut()[..])?;
    msg!("Value is now: {}", calc.value);

    Ok(())
}