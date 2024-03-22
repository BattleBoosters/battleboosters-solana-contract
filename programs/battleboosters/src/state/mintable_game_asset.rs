use anchor_lang::prelude::*;
use anchor_lang::{account, AnchorDeserialize, AnchorSerialize};
use solana_program::pubkey::Pubkey;

#[account]
pub struct MintableGameAssetData {
    /// is Locked will mean the PDA is in use and cannot be minted or re used
    pub is_locked: bool,
    /// is Burned will mean the PDA have been used and cannot be minted or re used
    pub is_burned: bool,
    /// is Minted mean the PDA have been minted
    pub is_minted: bool,
    /// owner of the PDA can use it in-game,
    /// on mint the owner is set to None which mean it is not available in the game until re-deposited
    pub owner: Option<Pubkey>,
    /// The metadata on-chain, which allow dynamic use on our game
    pub metadata: NftMetadata,
}

#[account]
pub struct MintableGameAssetLinkData {
    /// `Pubkey` of the mintable_game_asset
    pub mintable_game_asset_pubkey: Pubkey,
    /// this is the link to the address of the pda
    pub mintable_game_asset_nonce_tracker: u64,
    /// Checks if a PDA is eligible to update its `mintable_game_asset_nonce`.
    /// The PDA becomes eligible upon minting and withdrawing a `mintable_game_asset`,
    /// which break the link with the last `mintable_game_asset_nonce`.
    pub is_free: bool,
}

/// Metatada Standards copy on-chain
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct NftMetadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub animation_url: Option<String>,
    pub external_url: Option<String>,
    pub attributes: Vec<Attribute>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}