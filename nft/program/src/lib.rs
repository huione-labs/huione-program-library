#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

//! An ERC20-like Token program for the huione blockchain

extern crate core;

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use huione_program;
use huione_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};
// [188,56,115,111,181,89,4,188,200,72,252,58,100,25,31,200,130,101,53,176,238,183,201,241,196,18,87,196,250,201,74,84,240,249,253,190,149,225,133,127,116,215,180,243,76,162,212,32,25,213,150,194,142,63,213,76,27,53,119,129,48,252,80,17]
// huione_program::declare_id!("HDfwaKGaPncmD1veA2Gv5y73zq1RAUgd221gLsmc9SCY");

//[81,48,31,55,194,243,251,115,53,94,109,33,165,106,149,2,30,82,212,227,250,205,25,245,254,99,40,174,67,232,254,97,156,116,227,214,135,90,134,187,23,177,149,137,108,86,156,94,61,148,205,203,105,99,82,101,29,247,167,72,154,254,143,44]
huione_program::declare_id!("BXjynCN65MQBZCzqiHxPr2W1qCkK2RBiGVTFP9g4PQuh");


/// Checks that the supplied program ID is the correct one for SPL-token
pub fn check_program_account(nft_program_id: &Pubkey) -> ProgramResult {
    if nft_program_id != &id() {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}
