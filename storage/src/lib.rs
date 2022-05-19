use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};
use solcery_crud as crud;

#[derive(BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
pub struct AccountStorage {
    pub template: Pubkey, // Self for abstract
    pub accounts: Vec<Pubkey>,
}

pub fn process_instruction(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let (tag, _data) = instruction_data.split_first().unwrap();
    let accounts_iter = &mut accounts.iter();
    let signer_info = next_account_info(accounts_iter)?;
    let project_info = next_account_info(accounts_iter)?;
    match (tag) {
    	0 => {
    		let storage_info = next_account_info(accounts_iter)?;
    		let account_info = next_account_info(accounts_iter)?;
    		add(storage_info, account_info)
    	}
    	1 => {
    		let storage_info = next_account_info(accounts_iter)?;
    		let account_info = next_account_info(accounts_iter)?;
    		remove(storage_info, account_info)
    	}
        _ => return Err(ProgramError::InvalidAccountData)
    }
}

pub fn assign(
    project_info: &AccountInfo,
    storage_info: &AccountInfo,
    target_info: &AccountInfo,
) -> ProgramResult {
    msg!("Storage/Assign");
    let storage = AccountStorage {
        template: *target_info.key,
        accounts: Vec::new(),
    };
    crud::initialize(project_info, storage_info);
    crud::write(storage_info, 0, storage.try_to_vec().unwrap())?;
    Ok(())
}

pub fn add(storage_info: &AccountInfo, account_info: &AccountInfo) -> ProgramResult {
    msg!("Storage/Add");
    let mut storage = {
        let storage_data =
            &storage_info.data.borrow()[crud::RecordData::WRITABLE_START_INDEX..];
        AccountStorage::deserialize(&mut &*storage_data)?
    };
    storage.accounts.push(*account_info.key);
    crud::write(storage_info, 0, storage.try_to_vec().unwrap())?;
    Ok(())
}

pub fn remove(storage_info: &AccountInfo, account_info: &AccountInfo) -> ProgramResult {
    msg!("Storage/Remove");
    let mut storage = {
        let storage_data =
            &storage_info.data.borrow()[crud::RecordData::WRITABLE_START_INDEX..];
        AccountStorage::deserialize(&mut &*storage_data)?
    };
    for i in 0..storage.accounts.len() {
        if storage.accounts[i] == *account_info.key {
            storage.accounts.remove(i);
            break;
        }
    }
    crud::write(storage_info, 0, storage.try_to_vec().unwrap())?;
    Ok(())
}
