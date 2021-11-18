use {
    solana_program::{
        account_info::AccountInfo,
        entrypoint::ProgramResult,
        msg,
        program_error::PrintProgramError,
        pubkey::Pubkey
    },
    crate::error::NameServiceError,
    // crate::processor::Processor,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Entrypoint");
    if let Err(error) = Processor::process_instruction(program_id, accounts, instruction_data) {
        error.print::<NameServiceError>();
        return Err(error);
    }
    Ok(())
}