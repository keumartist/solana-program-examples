use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::AccountInfo,
        msg,
        program_error::ProgramError,
        program_pack::{IsInitialized, Pack, Sealed},
        pubkey::Pubkey
    },
};

/// The data for a Name Registry account is always prefixed a `NameRecordHeader` structure.
///
/// The layout of the remaining bytes in the account data are determined by the record `class`
/// 
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct NameRecordHeader {
    // Names are hierarchical.  `parent_name` contains the account address of the parent
    // name, or `Pubkey::default()` if no parent exists.
    pub parent_name: Pubkey,

    // The owner of this name
    pub owner: Pubkey,

    // The class of data this account represents (DNS record, twitter handle, SPL Token name/symbol, etc)
    //
    // If `Pubkey::default()` the data is unspecified.
    pub class: Pubkey,
}


impl Sealed for NameRecordHeader {}

impl Pack for NameRecordHeader {
    const LEN: usize = 96; // Pubkey(32) * 3

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        NameRecordHeader::deserialize(&mut src).map_err(|err| {
            msg!("Failed to deserialize name record");
            ProgramError::InvalidAccountData
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let mut slice = dst;
        self.serialize(&mut slice).unwrap()
    }
}
