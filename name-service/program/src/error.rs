use {
    num_derive::FromPrimitive,
    solana_program::{
        decode_error::DecodeError, 
    },
    thiserror::Error,
};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum NameServiceError {
    #[error("Out of space")]
    OutOfSpace,
}

// impl PrintProgramError for NameServiceError {
//     fn print<E>(&self)
//     where
//         E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive
//     {
//         match self {
//             NameServiceError::OutOfSpace => msg!("Error: Registry is out of space!"),
//         }
//     }

// }

impl<T> DecodeError<T> for NameServiceError {
    fn type_of() -> &'static str {
        "NameServiceError"
    }
}
