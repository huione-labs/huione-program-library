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
///
pub mod utils;
#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

// mainnet program id
#[cfg(not(feature = "testnet"))]
huione_program::declare_id!("PutSigDehXSkA3YdxZyNc7RySJ9djE11gSj6iLgSD8T");

/// testnet program id
#[cfg(feature = "testnet")]
huione_program::declare_id!("AyCQbQg68TKzbm66P2ACVXhm72M6o3HigrWCSVWJ7A2B");

/// Checks that the supplied program ID is the correct one for SPL-token
pub fn check_program_account(name_program_id: &Pubkey) -> ProgramResult {
    if name_program_id != &id() {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}