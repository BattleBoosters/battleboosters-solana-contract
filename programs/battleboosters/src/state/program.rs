use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, MintTo, Token, TokenAccount, Transfer},
};
use mpl_token_metadata::accounts::Metadata;

#[derive(Accounts)]
#[instruction(authority_bump: u8)]
pub struct InitializeProgram<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(init, payer = creator,
    seeds = [MY_APP_PREFIX, PROGRAM_STATE],
    bump,
    space = 8 + 8 + 32 + 8 + 8 + 8 + 8 + 1 + 1 + 1)]
    pub program: Account<'info, ProgramData>,
    //
    /// CHECK: This is a PDA used as the mint authority
    #[account(mut, seeds = [MY_APP_PREFIX, MINT_AUTHORITY], bump = authority_bump)]
    pub mint_authority: AccountInfo<'info>,

    /// CHECK: This is a PDA used as the mint authority
    #[account(init, mint::decimals = 0, mint::authority = mint_authority, mint::freeze_authority = mint_authority, payer = creator, seeds = [MY_APP_PREFIX, MINT], bump)]
    pub energy_minter: Account<'info, Mint>,

    // #[account(mut)]
    // pub energy_minter: Signer<'info>,
    // #[account(mut)]
    // pub shield_minter: Signer<'info>,
    // #[account(mut)]
    // pub points_minter: Signer<'info>,
    // #[account(mut)]
    // pub fighter_minter: Signer<'info>,
    // #[account(mut)]
    // pub champions_pass_minter: Signer<'info>,

    // /// CHECK: account checked in CPI
    //pub mint_energy_booster: UncheckedAccount<'info>,
    // /// CHECK: account checked in CPI
    // pub mint_shield_booster: UncheckedAccount<'info>,
    // /// CHECK: account checked in CPI
    // pub mint_points_booster: UncheckedAccount<'info>,
    // /// CHECK: account checked in CPI
    // pub mint_fighter: UncheckedAccount<'info>,
    // /// CHECK: account checked in CPI
    // pub mint_champions_pass: UncheckedAccount<'info>,
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
    pub metadata_energy_booster: UncheckedAccount<'info>,
    // /// CHECK: This is a metadata account
    // #[account(
    // mut,
    // seeds = [
    // b"metadata".as_ref(),
    // metadata_program.key().as_ref(),
    // energy_minter.key().as_ref(),
    // ],
    // bump,
    // seeds::program = metadata_program.key()
    // )]
    // pub metadata_shield_booster: UncheckedAccount<'info>,
    // /// CHECK: This is a metadata account
    // #[account(
    // mut,
    // seeds = [
    // b"metadata".as_ref(),
    // metadata_program.key().as_ref(),
    // energy_minter.key().as_ref(),
    // ],
    // bump,
    // seeds::program = metadata_program.key()
    // )]
    // pub metadata_points_booster: UncheckedAccount<'info>,
    // /// CHECK: This is a metadata account
    // #[account(
    // mut,
    // seeds = [
    // b"metadata".as_ref(),
    // metadata_program.key().as_ref(),
    // energy_minter.key().as_ref(),
    // ],
    // bump,
    // seeds::program = metadata_program.key()
    // )]
    // pub metadata_fighter: UncheckedAccount<'info>,
    // /// CHECK: This is a metadata account
    // #[account(
    // mut,
    // seeds = [
    // b"metadata".as_ref(),
    // metadata_program.key().as_ref(),
    // energy_minter.key().as_ref(),
    // ],
    // bump,
    // seeds::program = metadata_program.key()
    // )]
    // pub metadata_champions_pass: UncheckedAccount<'info>,
    // /// CHECK: This is a master edition account
    // #[account(
    //     mut,
    //     seeds = [
    //         b"metadata".as_ref(),
    //         metadata_program.key().as_ref(),
    //         energy_minter.key().as_ref(),
    //         b"edition".as_ref(),
    //     ],
    //     bump,
    //     seeds::program = metadata_program.key()
    // )]
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
    pub master_edition_account_energy_booster: UncheckedAccount<'info>,
    ///// CHECK: This is a master edition account
    // #[account(
    // mut,
    // seeds = [
    // b"metadata".as_ref(),
    // metadata_program.key().as_ref(),
    // energy_minter.key().as_ref(),
    // b"edition".as_ref(),
    // ],
    // bump,
    // seeds::program = metadata_program.key()
    // )]
    // pub master_edition_account_shield_booster: UncheckedAccount<'info>,
    // /// CHECK: This is a master edition account
    // #[account(
    // mut,
    // seeds = [
    // b"metadata".as_ref(),
    // metadata_program.key().as_ref(),
    // energy_minter.key().as_ref(),
    // b"edition".as_ref(),
    // ],
    // bump,
    // seeds::program = metadata_program.key()
    // )]
    // pub master_edition_account_points_booster: UncheckedAccount<'info>,
    // /// CHECK: This is a master edition account
    // #[account(
    // mut,
    // seeds = [
    // b"metadata".as_ref(),
    // metadata_program.key().as_ref(),
    // energy_minter.key().as_ref(),
    // b"edition".as_ref(),
    // ],
    // bump,
    // seeds::program = metadata_program.key()
    // )]
    // pub master_edition_account_fighter: UncheckedAccount<'info>,
    // /// CHECK: This is a master edition account
    // #[account(
    // mut,
    // seeds = [
    // b"metadata".as_ref(),
    // metadata_program.key().as_ref(),
    // energy_minter.key().as_ref(),
    // b"edition".as_ref(),
    // ],
    // bump,
    // seeds::program = metadata_program.key()
    // )]
    // pub master_edition_account_champions_pass: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: account constraints checked in account trait
    #[account(address = sysvar::instructions::ID)]
    pub sysvar_instructions: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: This is the metadata
    #[account(address = mpl_token_metadata::ID)]
    pub metadata_program: UncheckedAccount<'info>,
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
    /// Authority bump
    pub authority_bump: u8,
}
