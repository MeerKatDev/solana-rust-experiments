#![allow(unexpected_cfgs)]

use borsh::{BorshDeserialize, BorshSerialize};
use pinocchio::{
    ProgramResult, account_info::AccountInfo, entrypoint, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

pinocchio_pubkey::declare_id!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct Favorites {
    pub number: u64,
    pub color: String,
    pub hobbies: Vec<String>,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // difference from solana-program
    let account = &accounts[0];

    if !account.is_writable() {
        msg!("Account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    if account.is_owned_by(program_id) {
        msg!("Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let data = account.try_borrow_mut_data().unwrap();
    let favorites = Favorites::try_from_slice(&data).map_err(|_e| ProgramError::Custom(1))?;

    if favorites.color.len() > 50 {
        return Err(ProgramError::Custom(2));
    }

    if favorites.hobbies.len() > 50 || favorites.hobbies.len() < 5 {
        return Err(ProgramError::Custom(2));
    }

    // difference
    let mut data = account.try_borrow_mut_data().unwrap();
    let mut favorites = Favorites::try_from_slice(&data).map_err(|_e| ProgramError::Custom(1))?;
    favorites.color = "blue".to_string();

    // difference?
    favorites
        .serialize(&mut &mut data[..])
        .map_err(|_e| ProgramError::Custom(1))?;

    // difference: pinocchio is no_std, so it can't directly format
    msg!(format!("Color changed to {}", favorites.color).as_str());
    Ok(())
}
