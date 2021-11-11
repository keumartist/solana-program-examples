use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
    pub text: String
}

impl GreetingAccount {
    fn new(counter: u32, text: String) -> Self { 
        GreetingAccount {
            counter, text
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Message {
    pub text: String,
    pub from: String
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let message = Message::try_from_slice(instruction_data).map_err(|err| {
        msg!("Receiveing message failed, {:?}", err);
        ProgramError::InvalidInstructionData
    })?;

    msg!("Message received! {:?}", message);

    // Increment and store the number of times the account has been greeted
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    // let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow()).unwrap();
    greeting_account.counter += 1;
    msg!("greeting_account.counter : {:?}", greeting_account.counter);
    greeting_account.text = message.text;


    // TODO: Fix error(BorshIoError) here
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    // GreetingAccount::pack (&mut account.data.borrow_mut())?;
    // greeting_account.try_to_vec().unwrap();

    msg!("Greeted {} time(s)!", greeting_account.counter);
    msg!("Greeted text : {}", greeting_account.text);

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        // let data: GreetingAccount = GreetingAccount::new(0, "nope".to_string());
        let data: GreetingAccount = GreetingAccount {
            counter: 0, 
            text: "nope".to_string()
        };
        let mut serialized_data = &mut data.try_to_vec().unwrap();
        // let mut serialized_data = vec![0; mem::size_of::<u32>()];

        // let deserialized_data = GreetingAccount::try_from_slice(&serialized_data).unwrap();
        // println!("deserialized_data : {:?}", deserialized_data);

        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut serialized_data,
            &owner,
            false,
            Epoch::default(),
        );


        let message =  Message {
            text: "안뇽하세요".to_string(),
            from: "JKeum".to_string()
        };

        let serialized_message = message.try_to_vec().unwrap();
        let instruction_data = serialized_message;

        println!("instruction_data, {:?}", instruction_data);

        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );

        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );

        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_ne!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            4
        );
    }
}
