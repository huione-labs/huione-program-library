//! State transition types

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use borsh::{BorshDeserialize, BorshSerialize};
use num_enum::TryFromPrimitive;
use huione_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};
use std::string::FromUtf8Error;

/// MAX_ICON_URI_SIZE
pub const MAX_ICON_URI_SIZE: usize = 200;
/// MAX_MINT_NAME_SIZE
pub const MAX_MINT_NAME_SIZE: usize = 32;
/// MAX_MINT_SYMBOL_SIZE
pub const MAX_MINT_SYMBOL_SIZE: usize = 8;
/// MINT_SIZE
pub const MINT_SIZE: usize =
    32 + 8 + 8 + 1 + MAX_MINT_NAME_SIZE + MAX_MINT_SYMBOL_SIZE + 33 + MAX_ICON_URI_SIZE;

/// Mint data.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct NftMint {
    /// Optional authority used to mint new tokens. The mint authority may only be provided during
    /// mint creation. If no mint authority is present then the mint has a fixed supply and no
    /// further tokens may be minted.
    pub mint_authority: Pubkey, //32
    /// supply of tokens in now.
    pub supply: u64, //8
    /// supply of tokens in total.
    pub total_supply: u64, //8
    /// Is `true` if this structure has been initialized
    pub is_initialized: bool, //1
    /// name of nfts
    pub name: String, //32
    /// symbol of nfts
    pub symbol: String, //8
    /// Optional authority to freeze token accounts.
    pub freeze_authority: Option<Pubkey>, // 33
    /// the uri of icon
    pub icon_uri: String, // 200
}
// impl Sealed for NftMint {}
impl IsInitialized for NftMint {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl NftMint {
    /// serialize
    pub fn serialize(&self) -> std::io::Result<Vec<u8>> {
        borsh::to_vec(&self)
    }
    /// serialize_to
    pub fn serialize_to(&self, dst: &mut [u8]) {
        let data = borsh::to_vec(&self).unwrap();
        dst.copy_from_slice(data.as_slice())
    }
    /// deserialize
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        Ok(NftMint::try_from_slice(buf)?)
    }
}

impl Sealed for NftMint {}

impl Pack for NftMint {
    const LEN: usize = MINT_SIZE;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, MINT_SIZE];
        let (
            mint_authority_dst,
            supply_dst,
            total_supply_dst,
            is_initialized_dst,
            name_dst,
            symbol_dst,
            freeze_authority_dst,
            icon_uri_dst,
        ) = mut_array_refs![dst, 32, 8, 8, 1, 32, 8, 33, MAX_ICON_URI_SIZE];
        let NftMint {
            mint_authority,
            supply,
            total_supply,
            is_initialized,
            name,
            symbol,
            freeze_authority,
            icon_uri,
        } = self;

        mint_authority_dst.copy_from_slice(mint_authority.as_ref());
        *supply_dst = supply.to_le_bytes();
        *total_supply_dst = total_supply.to_le_bytes();
        is_initialized_dst[0] = *is_initialized as u8;
        pack_string_into(name, name_dst).expect("invalid length of name");
        pack_string_into(symbol, symbol_dst).expect("invalid length of name");
        pack_option_key_into(freeze_authority, freeze_authority_dst);
        pack_string_into(icon_uri, icon_uri_dst).expect("invalid length of name");
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, MINT_SIZE];
        let (
            mint_authority,
            supply,
            total_supply,
            is_initialized,
            name,
            symbol,
            freeze_authority,
            icon_uri,
        ) = array_refs![
            src,
            32,
            8,
            8,
            1,
            MAX_MINT_NAME_SIZE,
            MAX_MINT_SYMBOL_SIZE,
            33,
            MAX_ICON_URI_SIZE
        ];
        Ok(NftMint {
            mint_authority: Pubkey::new_from_array(*mint_authority),
            supply: u64::from_le_bytes(*supply),
            total_supply: u64::from_le_bytes(*total_supply),
            is_initialized: is_initialized[0] != 0,
            name: unpack_string(name).unwrap(),
            symbol: unpack_string(symbol).unwrap(),
            freeze_authority: unpack_option_key(freeze_authority),
            icon_uri: unpack_string(icon_uri).unwrap(),
        })
    }
}

/// pack option key.
fn pack_option_key_into(src: &Option<Pubkey>, target_dst: &mut [u8; 33]) {
    let (tag, body) = mut_array_refs![target_dst, 1, 32];
    match src {
        Some(key) => {
            *tag = [1];
            body.copy_from_slice(key.as_ref());
        }
        None => {
            *tag = [0];
        }
    }
}

/// unpack option key.
fn unpack_option_key(src_data: &[u8; 33]) -> Option<Pubkey> {
    let (tag, body) = array_refs![src_data, 1, 32];
    match tag[0] {
        0 => Option::None,
        1 => Option::Some(Pubkey::new(body)),
        _ => unreachable!(),
    }
}

/// The source string serialization to the target array, less replace with zero
fn pack_string_into(src_str: &String, target_dst: &mut [u8]) -> Result<(), String> {
    let copy_str = src_str.clone();
    let str_data = copy_str.into_bytes();
    if str_data.len() > target_dst.len() {
        return Err("src_str is too len".to_string());
    }
    let (valid_array, empty_arry) = target_dst.split_at_mut(str_data.len());
    valid_array.copy_from_slice(str_data.as_slice());
    let empty_vec = vec![0 as u8; empty_arry.len()];
    empty_arry.copy_from_slice(empty_vec.as_slice());
    Ok(())
}

/// unpack string.
fn unpack_string(src_data: &[u8]) -> Result<String, FromUtf8Error> {
    let ret = src_data.iter().enumerate().find(|(_index, val)| **val == 0);
    let mut index = src_data.len();
    if let Some(ret) = ret {
        index = ret.0
    }
    let (valid_data, _) = src_data.split_at(index);
    String::from_utf8(valid_data.to_vec())
}

/// MetaAccount data.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct MetaAccount {
    /// The mint associated with this account
    pub collection: Pubkey, //32
    /// The author of this NFT.
    pub author: Pubkey, //32
    /// The proposal of mint this NFT.
    pub proposal: Pubkey, //32
    /// The owner of this account.
    pub owner: Pubkey, //32
    /// The account's state
    pub state: AccountState, //1
    /// Optional authority to close the account
    pub close_authority: Option<Pubkey>, // 33
    /// The  token_id of nft
    pub nft_id: u64, // 8
    /// The mint time Timestamp of nft
    pub mint_timestamp: u64, // 8
    /// The suffix of the nft
    pub nft_uri: String, // 200
}

/// MAX_TOKEN_URI
const MAX_TOKEN_URI_SIZE: usize = 200;
/// max meta data size
pub const MAX_META_DATA_SIZE: usize = 32 + 32 + 32 + 32 + 1 + 33 + 8 + 8 + MAX_TOKEN_URI_SIZE;

impl Sealed for MetaAccount {}
impl IsInitialized for MetaAccount {
    fn is_initialized(&self) -> bool {
        self.state != AccountState::Uninitialized
    }
}

impl Pack for MetaAccount {
    const LEN: usize = MAX_META_DATA_SIZE;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, MAX_META_DATA_SIZE];
        let (
            collection_dst,
            author_dst,
            proposal_dst,
            owner_dst,
            state_dst,
            close_authority_dst,
            nft_id_dst,
            timestamp_dst,
            nft_uri_dst,
        ) = mut_array_refs![dst, 32, 32, 32, 32, 1, 33, 8, 8, MAX_TOKEN_URI_SIZE];
        let MetaAccount {
            collection,
            author,
            proposal,
            owner,
            state,
            close_authority,
            nft_id,
            mint_timestamp,
            nft_uri,
        } = self;

        collection_dst.copy_from_slice(collection.as_ref());
        author_dst.copy_from_slice(author.as_ref());
        proposal_dst.copy_from_slice(proposal.as_ref());
        owner_dst.copy_from_slice(owner.as_ref());
        state_dst[0] = *state as u8;
        pack_option_key_into(close_authority, close_authority_dst);
        *nft_id_dst = nft_id.to_le_bytes();
        *timestamp_dst = mint_timestamp.to_le_bytes();
        pack_string_into(nft_uri, nft_uri_dst).expect("invalid length of token_uri");
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, MAX_META_DATA_SIZE];
        let (
            collection,
            author,
            proposal,
            owner,
            state,
            close_authority,
            nft_id,
            timestamp,
            nft_uri,
        ) = array_refs![src, 32, 32, 32, 32, 1, 33, 8, 8, MAX_TOKEN_URI_SIZE];
        Ok(MetaAccount {
            collection: Pubkey::new_from_array(*collection),
            author: Pubkey::new_from_array(*author),
            proposal: Pubkey::new_from_array(*proposal),
            owner: Pubkey::new_from_array(*owner),
            state: AccountState::try_from_primitive(state[0])
                .or(Err(ProgramError::InvalidAccountData))?,
            close_authority: unpack_option_key(close_authority),
            nft_id: u64::from_le_bytes(*nft_id),
            mint_timestamp: u64::from_le_bytes(*timestamp),
            nft_uri: unpack_string(nft_uri).unwrap(),
        })
    }
}

impl MetaAccount {
    /// Checks if account is frozen
    pub fn is_frozen(&self) -> bool {
        self.state == AccountState::Frozen
    }
    // /// Checks if account is native
    // pub fn is_native(&self) -> bool {
    //     self.is_native.is_some()
    // }
    /// Checks if a token Account's owner is the system_program or the incinerator
    pub fn is_owned_by_system_program_or_incinerator(&self) -> bool {
        huione_program::system_program::check_id(&self.owner)
            || huione_program::incinerator::check_id(&self.owner)
    }
    /// serialize_to
    pub fn serialize_to(&self) -> std::io::Result<Vec<u8>> {
        borsh::to_vec(self)
    }

    /// deserialize
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        MetaAccount::try_from_slice(buf)
    }

    /// meta deserialize
    pub fn meta_deser(buf: &mut &[u8]) -> Result<Self, borsh::maybestd::io::Error> {
        // Metadata corruption shouldn't appear until after edition_nonce.
        let collection: Pubkey = BorshDeserialize::deserialize(buf)?;
        let author: Pubkey = BorshDeserialize::deserialize(buf)?;
        let proposal: Pubkey = BorshDeserialize::deserialize(buf)?;
        let owner: Pubkey = BorshDeserialize::deserialize(buf)?;
        let state: AccountState = BorshDeserialize::deserialize(buf)?;
        let close_authority: Option<Pubkey> = BorshDeserialize::deserialize(buf)?;
        let nft_id: u64 = BorshDeserialize::deserialize(buf)?;
        let mint_timestamp: u64 = BorshDeserialize::deserialize(buf)?;
        let nft_uri: String = BorshDeserialize::deserialize(buf)?;

        /* We can have accidentally valid, but corrupted data, particularly on the Collection struct,
        so to increase probability of catching errors If any of these deserializations fail, set all values to None.
        */

        let metadata = Self {
            collection,
            author,
            proposal,
            owner,
            state,
            close_authority,
            nft_uri,
            nft_id,
            mint_timestamp,
        };

        Ok(metadata)
    }
}

/// Account state.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, TryFromPrimitive, BorshDeserialize, BorshSerialize)]
pub enum AccountState {
    /// Account is not yet initialized
    Uninitialized,
    /// Account is initialized; the account owner and/or delegate may perform permitted operations
    /// on this account
    Initialized,
    /// Account has been frozen by the mint freeze authority. Neither the account owner nor
    /// the delegate are able to perform operations on this account.
    Frozen,
}

impl ToString for AccountState {
    fn to_string(&self) -> String {
        match self {
            AccountState::Frozen => "Frozen".to_string(),
            AccountState::Initialized => "Initialized".to_string(),
            _ => "".to_string(),
        }
    }
}

impl Default for AccountState {
    fn default() -> Self {
        AccountState::Uninitialized
    }
}

/// The offset of state field in Account's C representation
pub const ACCOUNT_INITIALIZED_INDEX: usize = 108;

/// Check if the account data buffer represents an initialized account.
/// This is checking the `state` (AccountState) field of an Account object.
pub fn is_initialized_account(account_data: &[u8]) -> bool {
    *account_data
        .get(ACCOUNT_INITIALIZED_INDEX)
        .unwrap_or(&(AccountState::Uninitialized as u8))
        != AccountState::Uninitialized as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use huione_program::account_info::AccountInfo;
    use std::panic;
    use std::str::FromStr;

    #[test]
    fn test_nft_meta_pack() {
        // empty mint
        let meta = MetaAccount {
            collection: Default::default(),
            author: Default::default(),
            proposal: Default::default(),
            owner: Default::default(),
            state: Default::default(),
            close_authority: None,
            nft_id: 0,
            nft_uri: "".to_string(),
            mint_timestamp: 1,
        };
        let mut dst = [0 as u8; MAX_META_DATA_SIZE];
        meta.pack_into_slice(&mut dst);
        println!("empty meta data {:?}", dst);
        // unpack test
        let unpack_ret = MetaAccount::unpack_from_slice(&dst);
        println!("unpack empty ret {:?}", unpack_ret);

        // nomal mint
        let meta = MetaAccount {
            collection: Pubkey::from_str("3u8SXMVLiceaDFdSR3iaig2WLdmRQPYNs6Xb2KkmMvXF").unwrap(),
            owner: Pubkey::from_str("3u8SXMVLiceaDFdSR3iaig2WLdmRQPYNs6Xb2KkmMvXF").unwrap(),
            state: AccountState::Initialized,
            close_authority: None,
            nft_id: 12344,
            token_uri: "www.baidu.com".to_string(),
        };
        let mut dst = [0 as u8; MAX_META_DATA_SIZE];
        meta.pack_into_slice(&mut dst);
        println!("normal meta data {:?}", dst);
        // unpack test
        let unpack_ret = MetaAccount::unpack_from_slice(&dst);
        println!("unpack normal ret {:?}", unpack_ret);
    }

    #[test]
    fn test_mint_pack() {
        // empty mint
        let mint = NftMint {
            mint_authority: Default::default(),
            supply: 0,
            total_supply: 0,
            is_initialized: false,
            name: "".to_string(),
            symbol: "".to_string(),
            freeze_authority: None,
            icon_uri: "".to_string(),
        };
        let mut dst = [0 as u8; MINT_SIZE];
        mint.pack_into_slice(&mut dst);
        println!("empty mint data {:?}", dst);
        // unpack test
        let unpack_ret = NftMint::unpack_from_slice(&dst);
        println!("unpack empty ret {:?}", unpack_ret);

        // nomal mint
        let mint = NftMint {
            mint_authority: Pubkey::from_str("3u8SXMVLiceaDFdSR3iaig2WLdmRQPYNs6Xb2KkmMvXF")
                .unwrap(),
            supply: 123,
            total_supply: 1234,
            is_initialized: false,
            name: "nftt mint 12345".to_string(),
            symbol: "usdt".to_string(),
            freeze_authority: None,
            icon_uri: "www.baidu.com".to_string(),
        };
        let mut dst = [0 as u8; MINT_SIZE];
        mint.pack_into_slice(&mut dst);
        println!("normal mint data {:?}", dst);
        // unpack test
        let unpack_ret = NftMint::unpack_from_slice(&dst);
        println!("unpack normal ret {:?}", unpack_ret);

        // name full mint
        let mint = NftMint {
            mint_authority: Pubkey::from_str("3u8SXMVLiceaDFdSR3iaig2WLdmRQPYNs6Xb2KkmMvXF")
                .unwrap(),
            supply: 123,
            total_supply: 1234,
            is_initialized: false,
            //name is too long
            name: "3u8SXMVLiceaDFdSR3iaig2WLdmRQPYNs6Xb2KkmMvXF11".to_string(),
            symbol: "usdt".to_string(),
            freeze_authority: None,
            icon_uri: "www.baidu.com".to_string(),
        };
        let mut dst = [0 as u8; MINT_SIZE];
        let result = panic::catch_unwind(move || mint.pack_into_slice(&mut dst));
        assert_eq!(true, result.is_err())
    }

    #[test]
    fn test_pack_string_into() {
        let s = "Hello world!".to_string();
        let taget_dst = &mut [0 as u8; 100];
        let _ = pack_string_into(&s, taget_dst);
        println!("valid ret {:?}", taget_dst);
        // unpack
        let unpack_ret = unpack_string(taget_dst);
        println!("unpack ret {:?}", unpack_ret);

        let taget_dst = &mut [0 as u8; 11];
        let invalid_ret = pack_string_into(&s, taget_dst);
        println!("invalid ret {:?}", invalid_ret.err().unwrap());
        let taget_dst = &mut [0 as u8; 12];
        let _ = pack_string_into(&s, taget_dst);
        println!("valid ret without filling {:?}", taget_dst);
        let unpack_ret = unpack_string(taget_dst);
        println!("without filling unpack ret {:?}", unpack_ret);

        // empty array unpack
        let src_data = [0 as u8; 0];
        let ret = unpack_string(&src_data);
        println!("empty array unpack ret {:?}", ret);
    }

    #[test]
    fn test_serialize_mint() {
        let mut mint = NftMint::default();
        mint.mint_authority = Pubkey::new_unique();
        mint.supply = 0;
        mint.total_supply = 10000;
        mint.is_initialized = true;
        mint.freeze_authority = None;
        mint.name = "terry".to_string();
        mint.symbol = "tr".to_string();
        let mint_data = mint.serialize().unwrap();
        println!("the ret is {:?} , len[{}]", mint_data, mint_data.len())
    }

    #[test]
    fn test_deserialize_mint() {
        let mut mint = NftMint::default();
        mint.mint_authority =
            Pubkey::from_str("BCE3vk474Htg2stYBtJSfKwcvxpRCTVvr7whPjMiUyZb").unwrap();
        mint.supply = 0;
        mint.total_supply = 10000;
        mint.is_initialized = true;
        mint.freeze_authority = None;
        mint.name = "terry".to_string();
        mint.symbol = "tr".to_string();
        let mint_data = mint.serialize();
        println!("the se ret is {:?}", mint_data);
        let new_mint = NftMint::deserialize(mint_data.as_ref().unwrap());
        println!("the de ret is {:?}", new_mint)
    }

    #[test]
    fn test_serialize_nft_meta() {
        let mut meta = MetaAccount::default();
        meta.collection = Pubkey::new_unique();
        meta.token_uri = "www.baidu.com".to_string();
        meta.nft_id = 1;
        meta.close_authority = None;
        meta.owner = Pubkey::new_unique();

        let meta_data = meta.serialize_to();
        println!("the ret is {:?}", meta_data)
    }

    #[test]
    fn test_deserialize_nft_meta() {
        let mut meta = MetaAccount::default();
        meta.collection = Pubkey::new_unique();
        meta.token_uri = "www.baidu.com".to_string();
        meta.nft_id = 1;
        meta.close_authority = None;
        meta.owner = Pubkey::new_unique();

        let meta_data = meta.serialize_to();
        println!("the ret is {:?}", meta_data);
        let new_meta = MetaAccount::deserialize(meta_data.as_ref().unwrap());
        println!("the de ret is {:?}", new_meta)
    }

    #[test]
    fn test_serialize_nft_meta_desir() {
        let mut empty_tail = [0; MAX_META_DATA_SIZE];
        let key = &Pubkey::new_unique();
        let owner = &Pubkey::new_unique();
        let mut lamport = 0;
        let account = &AccountInfo::new(
            key,
            false,
            false,
            &mut lamport,
            &mut empty_tail,
            owner,
            false,
            1,
        );

        let mut new_meta =
            MetaAccount::meta_deser(&mut account.data.borrow_mut().as_ref()).unwrap();
        // println!("before data {:?}", new_meta);
        new_meta.collection = Pubkey::new_unique();
        new_meta.token_uri = "www.baidu.com".to_string();
        new_meta.nft_id = 1;
        new_meta.close_authority = None;
        new_meta.owner = Pubkey::new_unique();
        let mut meta_data = new_meta.serialize_to().unwrap();
        println!("before data {:?}", meta_data);

        let mut empty_tail = vec![0 as u8; MAX_META_DATA_SIZE - meta_data.len()];
        meta_data.append(&mut empty_tail);
        println!("before data {:?}", meta_data);
        let ret = MetaAccount::meta_deser(&mut meta_data.as_slice());
        println!("the ret is {:?}", ret);

        // let mut account_data = account.data.try_borrow_mut().unwrap();
        // account_data.copy_from_slice(meta_data.as_slice());

        let sere_ret = new_meta.serialize(&mut *account.data.borrow_mut());
        println!("the se ret is {:?}", sere_ret);

        let after_meta = MetaAccount::meta_deser(&mut account.data.borrow_mut().as_ref()).unwrap();
        let after_data = after_meta.serialize_to().unwrap();
        println!("the de ret is {:?}", after_data);
    }
}
