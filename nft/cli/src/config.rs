use clap::ArgMatches;
use huione_clap_utils::{
    input_parsers::pubkey_of_signer, input_validators::is_valid_pubkey, keypair::{pubkey_from_path, signer_from_path_with_config, SignerFromPathConfig}
};
use huione_cli_output::OutputFormat;
use huione_client::{blockhash_query::BlockhashQuery, rpc_client::RpcClient};
use huione_remote_wallet::remote_wallet::RemoteWalletManager;
use huione_sdk::{pubkey::Pubkey, signature::Signer};
use std::{fmt::Display, process::exit, sync::Arc};

pub(crate) struct Config<'a> {
    pub(crate) rpc_client: Arc<RpcClient>,
    pub(crate) _websocket_url: String,
    pub(crate) outhuione_format: OutputFormat,
    pub(crate) fee_payer: Pubkey,
    pub(crate) default_keypair_path: String,
    pub(crate) nonce_account: Option<Pubkey>,
    pub(crate) nonce_authority: Option<Pubkey>,
    pub(crate) blockhash_query: BlockhashQuery,
    pub(crate) sign_only: bool,
    pub(crate) debug: bool,
    pub(crate) dump_transaction_message: bool,
    pub(crate) multisigner_pubkeys: Vec<&'a Pubkey>,
    pub(crate) program_id: Pubkey,
}

impl<'a> Config<'a> {

    // Checks if an explicit address was provided, otherwise return the default address.
    pub(crate) fn pubkey_or_default(
        &self,
        arg_matches: &ArgMatches,
        address_name: &str,
        wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
    ) -> Pubkey {
        if address_name != "owner" {
            if let Some(address) =
                pubkey_of_signer(arg_matches, address_name, wallet_manager).unwrap()
            {
                return address;
            }
        }

        return self
            .default_address(arg_matches, wallet_manager)
            .unwrap_or_else(|e| {
                eprintln!("error: {}", e);
                exit(1);
            });
    }

    // Checks if an explicit signer was provided, otherwise return the default signer.
    pub(crate) fn signer_or_default(
        &self,
        arg_matches: &ArgMatches,
        authority_name: &str,
        wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
    ) -> (Box<dyn Signer>, Pubkey) {
        // If there are `--multisig-signers` on the command line, allow `NullSigner`s to
        // be returned for multisig account addresses
        let config = SignerFromPathConfig {
            allow_null_signer: !self.multisigner_pubkeys.is_empty(),
        };
        let mut load_authority = move || {
            // fallback handled in default_signer() for backward compatibility
            if authority_name != "owner" {
                if let Some(keypair_path) = arg_matches.value_of(authority_name) {
                    return signer_from_path_with_config(
                        arg_matches,
                        keypair_path,
                        authority_name,
                        wallet_manager,
                        &config,
                    );
                }
            }

            self.default_signer(arg_matches, wallet_manager, &config)
        };

        let authority = load_authority().unwrap_or_else(|e| {
            eprintln!("error: {}", e);
            exit(1);
        });

        let authority_address = authority.pubkey();
        (authority, authority_address)
    }

    fn default_address(
        &self,
        matches: &ArgMatches,
        wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
    ) -> Result<Pubkey, Box<dyn std::error::Error>> {
        // for backwards compatibility, check owner before cli config default
        if let Some(address) = pubkey_of_signer(matches, "owner", wallet_manager).unwrap() {
            return Ok(address);
        }

        let path = &self.default_keypair_path;
        pubkey_from_path(matches, path, "default", wallet_manager)
    }

    fn default_signer(
        &self,
        matches: &ArgMatches,
        wallet_manager: &mut Option<Arc<RemoteWalletManager>>,
        config: &SignerFromPathConfig,
    ) -> Result<Box<dyn Signer>, Box<dyn std::error::Error>> {
        // for backwards compatibility, check owner before cli config default
        if let Some(owner_path) = matches.value_of("owner") {
            return signer_from_path_with_config(
                matches,
                owner_path,
                "owner",
                wallet_manager,
                config,
            );
        }

        let path = &self.default_keypair_path;
        signer_from_path_with_config(matches, path, "default", wallet_manager, config)
    }
}


pub fn is_valid_pubkey_or_None<T>(str: T) -> Result<(), String>
where
    T: AsRef<str> + Display,
{
    if  str.as_ref() == "NONE" 
    {
        Ok(())
    } else {
        is_valid_pubkey(str.as_ref())
    }
}