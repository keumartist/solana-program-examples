
use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    log::sol_log_compute_units,
    msg
};

use std::io::ErrorKind::InvalidData;

use crate::state::{ChatMessage, DUMMY_CREATED_AT, DUMMY_TX_ID};

/// Amount of bytes of account data to allocate
pub const SIZE: usize = 42;

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("This account {} is not owned by this program {}, so cannot be updated", account.key, program_id);
    }

    sol_log_compute_units();

    let instruction_data_message = ChatMessage::try_from_slice(instruction_data).map_err(|err| {
        msg!("Attempt to deserialize instruction data has failed. {:?}", err);
        ProgramError::InvalidInstructionData
    })?;

    msg!("Instruction_data message object {:?}", instruction_data_message);

    let mut existing_data_messages = match <Vec<ChatMessage>>::try_from_slice(&account.data.borrow_mut()) {
        Ok(data) => data,
        Err(err) => {
            if err.kind() == InvalidData {
                msg!("Invalid data so initializing account data");
                get_init_chat_messages()
            } else {
                panic!("unknown error decoding account data {:?}", err)
            }
        }
    };

    let index = existing_data_messages.iter().position(|p| p.archive_id == String::from(DUMMY_TX_ID)).unwrap();
    msg!("Found index {}", index);
    existing_data_messages[index] = instruction_data_message;
    let updated_data = existing_data_messages.try_to_vec().expect("Failed to encode data");

    let data = &mut &mut account.data.borrow_mut();
    msg!("Attempting save data");
    data[..updated_data.len()].copy_from_slice(&updated_data);
    
    let saved_data = <Vec<ChatMessage>>::try_from_slice(data)?;

    msg!("Chat Message has been saved to account data. {:?}", saved_data);
    sol_log_compute_units();

    Ok(())
}

pub fn get_init_chat_message() -> ChatMessage {
    ChatMessage {
        archive_id: String::from(DUMMY_TX_ID),
        created_at: String::from(DUMMY_CREATED_AT)
    }
}

pub fn get_init_chat_messages() -> Vec<ChatMessage> {
    let mut messges = Vec::new();

    for _ in 0..20 {
        messges.push(get_init_chat_message());
    }
    messges
}
