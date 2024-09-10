#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

//! An name service for the huione blockchain
use huione_program::entrypoint::ProgramResult;
use huione_program::program_error::ProgramError;
use huione_program::pubkey::Pubkey;

///
pub mod error;
///
pub mod instruction;
///
pub mod processor;
///
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;
mod huione_domain_account;
///
pub mod multi_sig_account_inline;
/// USDT token account id
pub mod usdt_token_account {
    huione_program::declare_id!("7v6vUyq6wNdxzHJdCr8KT2rmNn51wcQvKiAgGHnpUcfn");
}
/// Oracle program id
pub mod oracle_program {
    huione_program::declare_id!("EeVKEb2A9Xx7n5KXdu5a8PN3Eri7WPH9phUYKEHKEDCL");
}

huione_program::declare_id!("ErKyCbJc8qmPUvpWBQSTvyPFmvouD8Z1uekVP3N9HAuC");

/// Checks that the supplied program ID is the correct one for SPL-token
pub fn check_program_account(name_program_id: &Pubkey) -> ProgramResult {
    if name_program_id != &id() {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}