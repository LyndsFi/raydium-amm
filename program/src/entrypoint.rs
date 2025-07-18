//! Program entrypoint definitions

#![cfg(not(feature = "no-entrypoint"))]

use crate::processor::Processor;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    msg, pubkey::Pubkey,
};

entrypoint!(process_instruction);
fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        // Log the error using the new recommended approach
        msg!("Program error: {}", error);
        return Err(error);
    }
    Ok(())
}
