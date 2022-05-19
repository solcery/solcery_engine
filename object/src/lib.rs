use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solcery_crud as crud;

/// Struct wrapping data and providing metadata
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Object {
    pub id: u32,
    pub template: Pubkey,
    pub data: ObjectData,
}

const OBJECT_METADATA_SIZE: usize = 36;
const CRUD_METADATA_SIZE: usize = 33;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ObjectData {
    pub field_offsets: Vec<ObjectFieldData>,
    pub field_data: Vec<u8>,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ObjectFieldData {
    pub field_id: u32,
    pub start_offset: usize,
    pub end_offset: usize,
}

pub fn process_instruction(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let (tag, data) = instruction_data.split_first().unwrap();
    let accounts_iter = &mut accounts.iter();
    let signer_info = next_account_info(accounts_iter)?;
    let project_info = next_account_info(accounts_iter)?;
    match (tag) {
    	0 => {
            let template_info = next_account_info(accounts_iter)?;
    		let storage_info = next_account_info(accounts_iter)?;
    		let object_info = next_account_info(accounts_iter)?;
    		create(project_info, template_info, storage_info, object_info)
    	}
    	1 => {
    		let object_info = next_account_info(accounts_iter)?;
    		update(object_info, data.to_vec())
    	}
        2 => {
            let object_info = next_account_info(accounts_iter)?;
            let src_info = next_account_info(accounts_iter)?;
            load_data_from_src(object_info, src_info, 0, 0)
        }
        3 => {
            let object_info = next_account_info(accounts_iter)?;
            let src_info = next_account_info(accounts_iter)?;
            clone(object_info, src_info)
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
    crud::initialize(&project_info, &object_info)?;
    let object_data = Object {
        id: solcery_project::get_uniq_id(project_info),
        template: *template_info.key,
        data: ObjectData {
            field_offsets: Vec::new(),
            field_data: Vec::new(),
        },
    };
    crud::write(object_info, 0, object_data.try_to_vec().unwrap());
    solcery_storage::add(storage_info, object_info)?;
    Ok(())
}

pub fn update(
    object_info: &AccountInfo,
    data: Vec<u8>,
) -> ProgramResult {
    crud::write(object_info, 36, data);
    Ok(())
}

pub fn load_data_from_src(
    object_info: &AccountInfo,
    src_info: &AccountInfo,
    object_offset: usize,
    data_offset: usize,
) -> ProgramResult {
    let offset = OBJECT_METADATA_SIZE + CRUD_METADATA_SIZE + object_offset;
    object_info.data.borrow_mut()[offset..].copy_from_slice(&src_info.data.borrow()[data_offset..]);
    Ok(())
}

pub fn clone(
    object_info: &AccountInfo,
    src_info: &AccountInfo,
) -> ProgramResult { 
    object_info.data.borrow_mut()[CRUD_METADATA_SIZE + OBJECT_METADATA_SIZE..].copy_from_slice(&src_info.data.borrow()[CRUD_METADATA_SIZE + OBJECT_METADATA_SIZE..]);
    Ok(())
}
