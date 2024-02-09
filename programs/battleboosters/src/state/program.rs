use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, MintTo, Token, TokenAccount, Transfer},
};
use mpl_token_metadata::{
    accounts::Metadata
};

#[derive(Accounts)]
pub struct InitializeProgram<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(init, payer = creator,
    seeds = [MY_APP_PREFIX, PROGRAM_STATE],
    bump,
    space = 8 + 8 + 32 + 8 + 8 + 8 + 8 + 1 + 1)]
    pub program: Account<'info, ProgramData>,
    /// CHECK: This is a PDA used as the mint authority
    #[account(seeds = [MY_APP_PREFIX, MINT_AUTHORITY], bump)]
    pub mint_authority:  AccountInfo<'info>,
    #[account(
        init,
        payer = creator,
        mint::decimals = 0,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
        seeds = [MY_APP_PREFIX, MINT],
        bump,
    )]
    pub mint_energy_booster: Account<'info, Mint>,
    // #[account(
    // init,
    // payer = creator,
    // mint::decimals = 0,
    // mint::authority = mint_authority,
    //
    // seeds = [MY_APP_PREFIX, MINT, 2_u8.to_le_bytes().as_ref()],
    // bump,
    // )]
    // pub mint_shield_booster: Account<'info, Mint>,
    // #[account(
    // init,
    // payer = creator,
    // mint::decimals = 0,
    // mint::authority = mint_authority,
    //
    // seeds = [MY_APP_PREFIX, MINT, 3_u8.to_le_bytes().as_ref()],
    // bump,
    // )]
    // pub mint_points_booster: Account<'info, Mint>,
    // #[account(
    // init,
    // payer = creator,
    // mint::decimals = 0,
    // mint::authority = mint_authority,
    //
    // seeds = [MY_APP_PREFIX, MINT, 4_u8.to_le_bytes().as_ref()],
    // bump,
    // )]
    // pub mint_fighter: Account<'info, Mint>,
    // #[account(
    // init,
    // payer = creator,
    // mint::decimals = 0,
    // mint::authority = mint_authority,
    //
    // seeds = [MY_APP_PREFIX, MINT, 5_u8.to_le_bytes().as_ref()],
    // bump,
    // )]
    // pub mint_champions_pass: Account<'info, Mint>,

    // /// CHECK: This is a metadata account
    // #[account(
    // mut,
    // seeds = [
    // b"metadata".as_ref(),
    // metadata_program.key().as_ref(),
    // mint_energy_booster.key().as_ref(),
    // ],
    // bump,
    // seeds::program = metadata_program.key()
    // )]
    // pub metadata_energy_booster: UncheckedAccount<'info>,
    // /// CHECK: This is a master edition account
    // #[account(
    // mut,
    // seeds = [
    // b"metadata".as_ref(),
    // metadata_program.key().as_ref(),
    // mint_energy_booster.key().as_ref(),
    // b"edition".as_ref(),
    // ],
    // bump,
    // seeds::program = metadata_program.key()
    // )]
    // pub master_edition_account_energy_booster: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,

    // /// CHECK: This is the metadata
    // #[account(address = mpl_token_metadata::ID)]
    // pub metadata_program: AccountInfo<'info>,
}

// #[derive(Accounts)]
// pub struct InitializeMetadata<'info> {
//     // Metadata accounts for each mint
//     /// CHECK: This is a metadata account
//     #[account(
//     mut,
//     seeds = [
//     b"metadata".as_ref(),
//     metadata_program.key().as_ref(),
//     mint_energy_booster.key().as_ref(),
//     ],
//     bump,
//     seeds::program = metadata_program.key()
//     )]
//     pub metadata_energy_booster: UncheckedAccount<'info>,
//     /// CHECK: This is a metadata account
//     #[account(
//     mut,
//     seeds = [
//     b"metadata".as_ref(),
//     metadata_program.key().as_ref(),
//     mint_shield_booster.key().as_ref(),
//     ],
//     bump,
//     seeds::program = metadata_program.key()
//     )]
//     pub metadata_shield_booster: UncheckedAccount<'info>,
//     /// CHECK: This is a metadata account
//     #[account(
//     mut,
//     seeds = [
//     b"metadata".as_ref(),
//     metadata_program.key().as_ref(),
//     mint_points_booster.key().as_ref(),
//     ],
//     bump,
//     seeds::program = metadata_program.key()
//     )]
//     pub metadata_points_booster: UncheckedAccount<'info>,
//     /// CHECK: This is a metadata account
//     #[account(
//     mut,
//     seeds = [
//     b"metadata".as_ref(),
//     metadata_program.key().as_ref(),
//     mint_fighter.key().as_ref(),
//     ],
//     bump,
//     seeds::program = metadata_program.key()
//     )]
//     pub metadata_fighter: UncheckedAccount<'info>,
//     /// CHECK: This is a metadata account
//     #[account(
//     mut,
//     seeds = [
//     b"metadata".as_ref(),
//     metadata_program.key().as_ref(),
//     mint_champions_pass.key().as_ref(),
//     ],
//     bump,
//     seeds::program = metadata_program.key()
//     )]
//     pub metadata_champions_pass: UncheckedAccount<'info>,
//
//
//     /// CHECK: This is a master edition account
//     #[account(
//     mut,
//     seeds = [
//     b"metadata".as_ref(),
//     metadata_program.key().as_ref(),
//     mint_energy_booster.key().as_ref(),
//     b"edition".as_ref(),
//     ],
//     bump,
//     seeds::program = metadata_program.key()
//     )]
//     pub master_edition_account_energy_booster: UncheckedAccount<'info>,
//     /// CHECK: This is a master edition account
//     #[account(
//     mut,
//     seeds = [
//     b"metadata".as_ref(),
//     metadata_program.key().as_ref(),
//     mint_shield_booster.key().as_ref(),
//     b"edition".as_ref(),
//     ],
//     bump,
//     seeds::program = metadata_program.key()
//     )]
//     pub master_edition_account_shield_booster: UncheckedAccount<'info>,
//     /// CHECK: This is a master edition account
//     #[account(
//     mut,
//     seeds = [
//     b"metadata".as_ref(),
//     metadata_program.key().as_ref(),
//     mint_points_booster.key().as_ref(),
//     b"edition".as_ref(),
//     ],
//     bump,
//     seeds::program = metadata_program.key()
//     )]
//     pub master_edition_account_points_booster: UncheckedAccount<'info>,
//     /// CHECK: This is a master edition account
//     #[account(
//     mut,
//     seeds = [
//     b"metadata".as_ref(),
//     metadata_program.key().as_ref(),
//     mint_fighter.key().as_ref(),
//     b"edition".as_ref(),
//     ],
//     bump,
//     seeds::program = metadata_program.key()
//     )]
//     pub master_edition_account_fighter: UncheckedAccount<'info>,
//     /// CHECK: This is a master edition account
//     #[account(
//     mut,
//     seeds = [
//     b"metadata".as_ref(),
//     metadata_program.key().as_ref(),
//     mint_champions_pass.key().as_ref(),
//     b"edition".as_ref(),
//     ],
//     bump,
//     seeds::program = metadata_program.key()
//     )]
//     pub master_edition_account_champions_pass: UncheckedAccount<'info>,
//
//     /// CHECK: This is the metadata
//     #[account(address = mpl_token_metadata::ID)]
//     pub metadata_program: AccountInfo<'info>,
//
//
// }



#[account]
pub struct ProgramData {
    /// Represent the current amount of created event
    pub event_counter: u64,
    /// The authority which are allowed to administrate the contract
    pub admin_pubkey: Pubkey,
    /// The price in USD of each NFT fighter pack
    pub fighter_pack_price: u64,
    /// The price in USD of each NFT points booster
    pub booster_points_price: u64,
    /// The price in USD of each NFT energy booster
    pub booster_energy_price: u64,
    /// The price in USD of each NFT shield booster
    pub booster_shield_price: u64,
    /// The amount of fighters contained on each NFT fighter pack
    pub fighter_pack_amount: u8,
    /// This data prevent re-initialization
    pub is_initialized: bool,
}
