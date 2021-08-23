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

use std::convert::TryInto;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, BorshSchema, PartialEq)]
pub struct TemplateData {
    pub name: String,
    pub storages: Vec<Pubkey>,
    pub max_field_index: u32,
    pub fields: Vec<TemplateField>,
    pub custom_data: Vec<u8>,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, BorshSchema, PartialEq)]
pub struct TemplateField {
    pub id: u32,
    pub enabled: bool,
    pub field_type: u8,
    pub name: String, 
    pub field_data: Vec<u8>,
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
    		let project_templates_storage_info = next_account_info(accounts_iter)?;
    		create(project_info, template_info, storage_info, project_templates_storage_info)
    	}
    	1 => {
    		let template_info = next_account_info(accounts_iter)?;
            let (field_type, name) = data.split_first().unwrap();
    		add_field(template_info, *field_type, String::from_utf8(name.to_vec()).unwrap())
    	}
        3 => {
            let template_info = next_account_info(accounts_iter)?;
            let name = String::deserialize(&mut &data[..])?;
            change_name(template_info, name)
        }
        _ => return Err(ProgramError::InvalidAccountData)
    }
}

pub fn create(
    project_info: &AccountInfo,
    template_info: &AccountInfo,
    storage_info: &AccountInfo,
    project_templates_storage_info: &AccountInfo,
) -> ProgramResult { 
	msg!("Template/Create");
    let new_template_data = TemplateData {
        name: "New template".to_string(),
        storages: vec![*storage_info.key], // TODO: template without storage
        max_field_index: 0,
        fields: Vec::new()
    };
    solcery_crud::initialize(&project_info, &template_info);
    solcery_crud::write(template_info, 0, new_template_data.try_to_vec().unwrap());
    solcery_storage::assign(&project_info, &storage_info, &template_info)?;
    solcery_storage::add(&project_templates_storage_info, &template_info)?;
    Ok(())
}

pub fn add_field(
    template_info: &AccountInfo,
    field_type: u8,
    name: String,
) -> ProgramResult {
	msg!("Template/AddField");
    let mut template = {
        let mut template_data = &template_info.data.borrow()[solcery_crud::RecordData::WRITABLE_START_INDEX..];
        TemplateData::deserialize(&mut &template_data[..])?
    };
    template.max_field_index += 1;
    let field = TemplateField {
        id: template.max_field_index,
        enabled: true,
        field_type,
        name,
        field_data: Vec::new(),
    };
    template.fields.push(field);
    solcery_crud::write(template_info, 0, template.try_to_vec().unwrap());
    Ok(())
}

pub fn change_name(
    template_info: &AccountInfo,
    name: String,
) -> ProgramResult {
    let mut template = {
        let mut template_data = &template_info.data.borrow()[solcery_crud::RecordData::WRITABLE_START_INDEX..];
        TemplateData::deserialize(&mut &template_data[..])?
    };
    template.name = name;
    solcery_crud::write(template_info, 0, template.try_to_vec().unwrap());
    Ok(())
}