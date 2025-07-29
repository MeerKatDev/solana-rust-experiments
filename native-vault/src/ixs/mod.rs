pub mod create; // CreateAccount handler.
pub mod create_pda; // CreateAccount handler.
pub mod deposit; // Deposit handler.
pub mod withdraw; // Withdraw handler.

use pinocchio::program_error::ProgramError;

/// Enum representing possible vault instructions.
pub enum VaultInstructions {
    CreateAccount,    // Instruction to create a vault.
    CreatePdaAccount, // Instruction to create a PDA vault.
    Deposit,          // Instruction to deposit lamports into the vault.
    Withdraw,         // Instruction to withdraw lamports from the vault.
}

/// Convert a discriminator byte into a `VaultInstructions` enum variant.
impl TryFrom<&u8> for VaultInstructions {
    type Error = ProgramError;

    fn try_from(discriminator: &u8) -> Result<Self, Self::Error> {
        match discriminator {
            0 => Ok(VaultInstructions::CreateAccount),
            1 => Ok(VaultInstructions::CreatePdaAccount),
            2 => Ok(VaultInstructions::Deposit),
            3 => Ok(VaultInstructions::Withdraw),
            _ => Err(ProgramError::InvalidInstructionData), // Invalid discriminator.
        }
    }
}
