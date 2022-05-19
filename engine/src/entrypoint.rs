use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    account_info::{ AccountInfo, next_account_info },
    pubkey::Pubkey,
    msg,
};
use std::str::FromStr;


entrypoint!(process_instruction);
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let super_admins: Vec<Pubkey> = vec![
        Pubkey::from_str("9kXLhvDcWc4wzuapQpWkKVnJ8wKVhEDomwoFxkn58nfX").unwrap(),
        Pubkey::from_str("ESrHRyZKaC9VjTdvd7QHppxevXpiasUAbzx2XGBRanrv").unwrap(),
        Pubkey::from_str("CmxScbqG1imzdkmehMD1VoHait6oYx7o6CLtaHbDkdG1").unwrap(),
        Pubkey::from_str("25MhYRx9CFLyQxf5HQKLPFd86QbkNFUWwUcVvQcTvPHJ").unwrap(),
    ];
    let signer_account = next_account_info(&mut accounts.iter())?;
    if !super_admins.contains(signer_account.key) {
        return Err(ProgramError::InvalidAccountData); // closed for now
    }
    let (tag, rest) = instruction_data.split_first().unwrap();
    match tag {
        0 => solcery_template::process_instruction(accounts, rest)?,
        1 => solcery_object::process_instruction(accounts, rest)?,
        2 => solcery_storage::process_instruction(accounts, rest)?,
        3 => solcery_crud::process_instruction(accounts, rest)?,
        4 => solcery_project::process_instruction(accounts, rest)?,
        _ => return Err(ProgramError::InvalidAccountData),
    }
    Ok(())
}
