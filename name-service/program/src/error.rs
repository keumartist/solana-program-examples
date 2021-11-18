use {
    num_derive::FromPrimitive,
    solana_program::{decode_error::DecodeError, program_error::ProgramError},
    thiserror::Error
};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum NameServiceError {
    #[error("Out of space")]
    OutOfSpace,
}
