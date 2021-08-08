use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    account_info::{ AccountInfo },
    pubkey::Pubkey,
};

// use solana_program::{
//     entrypoint,
//     instruction::{AccountMeta, Instruction},
//     program::{invoke, invoke_signed},
//     entrypoint::ProgramResult,
//     program_error::ProgramError,
//     system_instruction,
//     system_program,
//     pubkey::Pubkey,
//     msg,
//     sysvar::{ clock::Clock, Sysvar },
//     declare_id,
// };

entrypoint!(process_instruction);
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}
