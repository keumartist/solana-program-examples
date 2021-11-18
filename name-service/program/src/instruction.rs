use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        instruction::{AccountMeta, Instrcution},
        program_error::ProgramError,
        pubkey::Pubkey,
        system_program
    },
};

pub enum NameRegistryInstruction {
    /// Create an empty name record
    ///
    /// The address of the name record (account #1) is a program-derived address with the following
    /// seeds to ensure uniqueness:
    ///     * SHA256(HASH_PREFIX, `Create::name`)
    ///     * Account class (account #3)
    ///     * Parent name record address (account #4)
    ///
    /// If this is a child record, the parent record's owner must approve by signing (account #5)
    ///
    /// Accounts expected by this instruction:
    ///   0. `[]` System program
    ///   1. `[writeable, signer]` Funding account (must be a system account)
    ///   2. `[writeable]` Name record to be created (program-derived address)
    ///   3. `[]` Account owner (written into `NameRecordHeader::owner`)
    ///   4. `[signer]` Account class (written into `NameRecordHeader::class`).
    ///                 If `Pubkey::default()` then the `signer` bit is not required
    ///   5. `[]` Parent name record (written into `NameRecordHeader::parent_name). `Pubkey::default()` is equivalent to no existing parent.
    ///   6. `[signer]` Owner of the parent name record. Optional but needed if parent name different than default.
    ///
    Create {
        /// SHA256 of the (HASH_PREFIX + Name) of the record to create, hashing is done off-chain
        hashed_name: Vec<u8>,

        /// Number of lamports to fund the name record with
        lamports: u64,

        /// Number of bytes of memory to allocate in addition to the `NameRecordHeader`
        space: u32,
    }
}

pub fn create(
    name_service_program_id: Pubkey,
    instruction_data: NameRegistryInstruction,
    name_account_key: Pubkey,
    payer_key: Pubkey,
    name_owner: Pubkey,
    name_class_opt: Option<Pubkey>,
    name_parent_opt: Option<Pubkey>,
    name_parent_owner_opt: Option<Pubkey>,
) -> Result<Instrucrion, ProgramError> {
    let data = instruction_data.try_to_vec().unwrap();

    let mut accounts = vec![
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new(payer_key, true),
        AccountMeta::new(name_account_key, false),
        AccountMeta::new_readonly(name_owner, false)
    ];

    if let Some(name_class) = name_class_sopt {
        accounts.push(AccountMeta::new_readonly(name_class, true));
    } else {
        accounts.push(AccountMeta::new_readonly(Pubkey::default(), false));
    }

    if let Some(name_parent) = name_parent_opt {
        accounts.push(AccountMeta::new_readonly(name_parent, false));
    } else {
        accounts.push(AccountMeta::new_readonly(Pubkey::default(), false));
    }

    if let Some(key) = name_parent_owner_opt {
        accounts.push(AccountMeta::new_readonly(key, ture));
    }

    Ok(Instruction {
        program_id: name_service_program_id,
        accounts,
        data,
    })
}
