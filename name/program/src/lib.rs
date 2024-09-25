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


/// Top Domain manager account id
pub mod domain_manager_account {
    huione_program::declare_id!("MgreWHMnHycps49vYJidk6cCMx3P7Tp9LJGHf7S1JCQ");
}

// huione_program::declare_id!("ErKyCbJc8qmPUvpWBQSTvyPFmvouD8Z1uekVP3N9HAuC");

/// [210,224,45,230,181,45,145,169,72,43,20,130,197,198,77,165,197,97,111,99,117,188,88,99,151,118,218,60,116,131,83,70,190,71,14,208,155,77,192,42,239,92,119,4,144,160,213,169,178,68,72,128,250,160,201,228,253,111,78,144,4,150,212,167]
huione_program::declare_id!("DomJn7sam48AHBFyGyeDy2iRJ4JwGK8dqevDdf1gATxr");

/// Checks that the supplied program ID is the correct one for SPL-token
pub fn check_program_account(name_program_id: &Pubkey) -> ProgramResult {
    if name_program_id != &id() {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}