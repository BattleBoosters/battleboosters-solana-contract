use crate::constants::*;
use crate::state::mystery_box::MysteryBoxData;
use crate::state::player::PlayerData;
use crate::state::program::ProgramData;
use crate::state::rarity::RarityData;
use anchor_lang::prelude::*;
use anchor_lang::{account, AnchorDeserialize, AnchorSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Accounts)]
#[instruction(mintable_game_asset_link_nonce: u64, player_pubkey: Pubkey)]
pub struct CreateMintableGameAsset<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [MY_APP_PREFIX, PROGRAM_STATE], bump)]
    pub program: Box<Account<'info, ProgramData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, PLAYER, player_pubkey.as_ref()],
    bump,
    )]
    pub player_account: Box<Account<'info, PlayerData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MYSTERY_BOX, mystery_box.nonce.to_le_bytes().as_ref(), player_pubkey.as_ref()],
    bump,
    )]
    pub mystery_box: Box<Account<'info, MysteryBoxData>>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, RARITY],
    bump,
    )]
    pub rarity: Option<Box<Account<'info, RarityData>>>,
    #[account(
    init,
    payer = signer,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, program.mintable_game_asset_nonce.to_le_bytes().as_ref()],
    space = 8 + 1 + 1 + 32 + (4 + 20) + (4 + 100) + (4 + 100) + (4 + 100) + (4 + 100) + (4 + 480) + 8,
    bump
    )]
    pub mintable_game_asset: Box<Account<'info, MintableGameAssetData>>,

    #[account(
    init_if_needed,
    payer = signer,
    seeds = [MY_APP_PREFIX, MINTABLE_GAME_ASSET, mintable_game_asset_link_nonce.to_le_bytes().as_ref(), player_pubkey.as_ref()],
    space = 8 + 32 + 8 + 1 + 8 + 8,
    bump,
    )]
    pub mintable_game_asset_link: Box<Account<'info, MintableGameAssetLinkData>>,

    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

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
    /// Nonce of the `mintable_game_asset`
    pub nonce: u64,
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
    /// Nonce of the `mintable_game_asset_link`
    pub nonce: u64,
}

/// Metatada Standards copy on-chain
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct NftMetadata {
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub animation_url: Option<String>,
    pub external_url: Option<String>,
    pub attributes: Vec<Attribute>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}
