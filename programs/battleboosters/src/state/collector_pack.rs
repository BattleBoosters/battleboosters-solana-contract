use super::rarity::RarityData;
use crate::constants::*;
use crate::state::player::PlayerData;
use crate::state::program::ProgramData;
use crate::types::*;
use anchor_lang::prelude::*;

use anchor_lang::solana_program::sysvar;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, MintTo, Token, TokenAccount, Transfer},
};
use mpl_token_metadata::accounts::Metadata;
use mpl_token_metadata::accounts::TokenRecord;

#[derive(Accounts)]
#[instruction(player_pubkey: Pubkey)]
pub struct InitializeCollectorPack<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        mut,
        seeds = [MY_APP_PREFIX, PLAYER, player_pubkey.as_ref()],
        bump,
    )]
    pub player_account: Account<'info, PlayerData>,
    #[account(
        init,
        payer = creator,
        seeds = [MY_APP_PREFIX, COLLECTOR, player_pubkey.key().as_ref(), player_account.order_nonce.to_le_bytes().as_ref()],
        bump,
        space = 8 + 8 + 8 + 1 + 8
    )]
    pub collector_pack: Account<'info, CollectorPack>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintCollectorPack<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut, seeds = [MY_APP_PREFIX, PROGRAM_STATE], bump)]
    pub program: Box<Account<'info, ProgramData>>,
    /// CHECK: This is a PDA used as the mint authority
    #[account(mut, seeds = [MY_APP_PREFIX, MINT_AUTHORITY], bump)]
    pub mint_authority: AccountInfo<'info>,

    // #[account(
    // mut,
    // seeds = [MY_APP_PREFIX, COLLECTOR, player_pubkey.as_ref(), order_nonce.to_le_bytes().as_ref()],
    // bump,
    // )]
    // pub collector_pack: Account<'info, CollectorPack>,

    // #[account(
    // mut,
    // seeds = [MY_APP_PREFIX, RARITY],
    // bump,
    // )]
    // pub rarity: Box<Account<'info, RarityData>>,

    /*
       Energy Booster
    */
    /// CHECK: This is a energy minter PDA
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, MINT, &[CollectionType::Energy as u8]],
    bump
    )]
    pub energy_minter: Box<Account<'info, Mint>>,
    /// CHECK: This is a metadata account
    #[account(
    mut,
    seeds = [
    b"metadata".as_ref(),
    metadata_program.key().as_ref(),
    energy_minter.key().as_ref(),
    ],
    bump,
    seeds::program = metadata_program.key()
    )]
    pub energy_metadata: UncheckedAccount<'info>,
    /// CHECK: This is a master edition account
    #[account(
    mut,
    seeds = [
    b"metadata".as_ref(),
    metadata_program.key().as_ref(),
    energy_minter.key().as_ref(),
    b"edition".as_ref(),
    ],
    bump,
    seeds::program = metadata_program.key()
    )]
    pub energy_master_edition: UncheckedAccount<'info>,

    #[account(
    init,
    mint::decimals = 0,
    mint::authority = mint_authority,
    mint::freeze_authority = mint_authority,
    payer = creator,
    seeds = [MY_APP_PREFIX, MINT, program.pre_mint_nonce.to_le_bytes().as_ref()],
    bump
    )]
    pub minter: Box<Account<'info, Mint>>,
    /// CHECK: This is a metadata account
    #[account(
    mut,
    seeds = [
    b"metadata".as_ref(),
    metadata_program.key().as_ref(),
    minter.key().as_ref(),
    ],
    bump,
    seeds::program = metadata_program.key()
    )]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is a master edition account
    #[account(
    mut,
    seeds = [
    b"metadata".as_ref(),
    metadata_program.key().as_ref(),
    minter.key().as_ref(),
    b"edition".as_ref(),
    ],
    bump,
    seeds::program = metadata_program.key()
    )]
    pub master_edition: UncheckedAccount<'info>,

    #[account(
    init,
    payer = creator,
    associated_token::mint = minter,
    associated_token::authority = creator,
    )]
    pub token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: This is a token record account
    #[account(
    mut,
    seeds = [
    b"metadata".as_ref(),
    metadata_program.key().as_ref(),
    minter.key().as_ref(),
    b"token_record",
    token_account.key().as_ref(),
    ],
    seeds::program = metadata_program.key(),
    bump,
    )]
    pub token_record: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

    /// CHECK: account constraints checked in account trait
    #[account(address = sysvar::instructions::ID)]
    pub sysvar_instructions: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: This is the metadata
    #[account(address = mpl_token_metadata::ID)]
    pub metadata_program: UncheckedAccount<'info>,
}

#[account]
pub struct CollectorPack {
    pub fighter_mint_allowance: u64,
    pub booster_mint_allowance: u64,
    pub randomness: Option<Vec<u8>>,
}
// {
// "attributes": [
//     {
//     "trait_type": "Fighting Style",
//     "value": "Boxing"
//     },
//     {
//     "trait_type": "Category",
//     "value": "Striker"
//     },
//     {
//     "trait_type": "Rarity",
//     "value": "Common"
//     },
//     {
//     "trait_type": "Health",
//     "value": 100
//     },
//     {
//     "trait_type": "Power",
//     "value": 100
//     },
//     {
//     "trait_type": "Energy",
//     "value": 200
//     },
// ]
// }
