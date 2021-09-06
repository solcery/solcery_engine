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

#[derive(BorshSerialize, BorshDeserialize, BorshSchema, Debug, PartialEq)]
pub enum SolceryTypes {
    Error,
    SInt,
    SString,
    SLink { template: Pubkey },
    SBrick { brick_type: u32 },
}

use std::convert::TryInto;

#[derive(Debug, BorshSerialize, BorshDeserialize, BorshSchema, PartialEq)]
pub struct TemplateData {
    pub id: u32,
    pub name: String,
    pub storages: Vec<Pubkey>,
    pub max_field_index: u32,
    pub fields: Vec<Field>,
    pub custom_data: Vec<u8>,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, BorshSchema, PartialEq)]
pub struct Field {
    pub id: u32,
    pub enabled: bool,
    pub params: FieldParams,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, BorshSchema, PartialEq)]
pub struct FieldParams {
    pub field_type: SolceryTypes,
    pub name: String, 
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
            let field_params = FieldParams::deserialize(&mut &data[..])?;
    		add_field(template_info, field_params)
    	}
        2 => {
            let template_info = next_account_info(accounts_iter)?;
            msg!("{:?}", data);
            let field_id = u32::deserialize(&mut &data[..])?;
            delete_field(template_info, field_id)
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
        id: solcery_project::get_uniq_id(project_info),
        name: "New template".to_string(),
        storages: vec![*storage_info.key], // TODO: template without storage
        max_field_index: 0,
        fields: Vec::new(),
        custom_data: Vec::new(),
    };
    solcery_crud::initialize(&project_info, &template_info);
    solcery_crud::write(template_info, 0, new_template_data.try_to_vec().unwrap());
    solcery_storage::assign(&project_info, &storage_info, &template_info)?;
    solcery_storage::add(&project_templates_storage_info, &template_info)?;
    Ok(())
}

pub fn add_field(
    template_info: &AccountInfo,
    field_params: FieldParams,
) -> ProgramResult {
	msg!("Template/AddField");
    let mut template = {
        let mut template_data = &template_info.data.borrow()[solcery_crud::RecordData::WRITABLE_START_INDEX..];
        TemplateData::deserialize(&mut &template_data[..])?
    };
    template.max_field_index += 1;
    let field = Field {
        id: template.max_field_index,
        enabled: true,
        params: field_params,
    };
    template.fields.push(field);
    solcery_crud::write(template_info, 0, template.try_to_vec().unwrap());
    Ok(())
}

pub fn delete_field(
    template_info: &AccountInfo,
    field_id: u32,
) -> ProgramResult {
    msg!("Template/DeleteField");
    let mut template = {
        let mut template_data = &template_info.data.borrow()[solcery_crud::RecordData::WRITABLE_START_INDEX..];
        TemplateData::deserialize(&mut &template_data[..])?
    };
    msg!("{:?}", template.fields);
    msg!("{:?}", field_id);
    let index_of_field_id = template.fields.iter().position(|x| x.id == field_id);
    msg!("{:?}", index_of_field_id);
    match index_of_field_id {
        Some(ind) => {
            template.fields.remove(ind);
            solcery_crud::write(template_info, 0, template.try_to_vec().unwrap());
            Ok(())
        }
        _ => Err(ProgramError::InvalidAccountData)
    }
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