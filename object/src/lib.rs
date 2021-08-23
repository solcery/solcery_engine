use {
    borsh::{ BorshDeserialize, BorshSerialize, BorshSchema },
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        entrypoint,
        msg,
        program_error::ProgramError,
        program_pack::IsInitialized,
        pubkey::Pubkey,
        program::invoke,
    },
};

/// Struct wrapping data and providing metadata
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, BorshSchema, PartialEq)]
pub struct ObjectData {
    pub template: Pubkey,
    pub fields: Vec<ObjectField>,
}


#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, BorshSchema, PartialEq)]
pub struct ObjectField {
    pub id: u32,
    pub value: Vec<u8>,
}

pub fn process_instruction(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (tag, data) = instruction_data.split_first().unwrap();
    let accounts_iter = &mut accounts.iter();
    match (tag) {
    	0 => {
            let project_info = next_account_info(accounts_iter)?;
            let template_info = next_account_info(accounts_iter)?;
    		let storage_info = next_account_info(accounts_iter)?;
    		let object_info = next_account_info(accounts_iter)?;
    		create(project_info, template_info, storage_info, object_info)
    	}
    	1 => {
    		let object_info = next_account_info(accounts_iter)?;
    		update(object_info, data.to_vec())
    	}
        _ => return Err(ProgramError::InvalidAccountData)
    }
}

pub fn create(
    project_info: &AccountInfo,
    template_info: &AccountInfo,
    storage_info: &AccountInfo,
    object_info: &AccountInfo,
) -> ProgramResult {
    msg!("Object/Create");
    solcery_crud::initialize(&project_info, &object_info)?;
    let object_data = ObjectData {
        template: *template_info.key,
        fields: Vec::new(),
    };
    solcery_crud::write(&object_info, 0, object_data.try_to_vec().unwrap());
    solcery_storage::add(&storage_info, &object_info)?;
    Ok(())
}

pub fn update(
    object_info: &AccountInfo,
    data: Vec<u8>,
) -> ProgramResult {
	msg!("Object/Update");
    solcery_crud::write(object_info, 32, data); //TODO
    Ok(())
}
