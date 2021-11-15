use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ChatMessage {
    pub archive_id: String, // Arweave TX id
    pub created_at: String
}


// example arweave tx (length 43)
// 1seRanklLU_1VTGkEk7P0xAwMJfA7owA1JHW5KyZKlY
// ReUohI9tEmXQ6EN9H9IkRjY9bSdgql_OdLUCOeMEte0
pub const DUMMY_TX_ID: &str = "0000000000000000000000000000000000000000000";
pub const DUMMY_CREATED_AT: &str = "0000000000000000"; // milliseconds, 16 digits
