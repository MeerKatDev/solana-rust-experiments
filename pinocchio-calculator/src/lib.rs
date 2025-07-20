#![allow(unexpected_cfgs)]

pub use crate::calculator::CalculatorInstructions;
use borsh::{BorshDeserialize, BorshSerialize};
use pinocchio::{
    ProgramResult, account_info::AccountInfo, entrypoint, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

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
    let account = &accounts[0];

    if account.is_owned_by(program_id) {
        msg!("Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut data = account.try_borrow_mut_data().unwrap();
    let mut calc = Calculator::try_from_slice(&data).map_err(|_e| ProgramError::BorshIoError)?;

    let calculator_instructions = CalculatorInstructions::try_from_slice(&instruction_data)
        .map_err(|_e| ProgramError::BorshIoError)?;

    calc.value = calculator_instructions.evaluate(calc.value);

    calc.serialize(&mut &mut data[..])
        .map_err(|_e| ProgramError::BorshIoError)?;

    msg!(format!("Value is now: {}", calc.value).as_str());

    Ok(())
}
