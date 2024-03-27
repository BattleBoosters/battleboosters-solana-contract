use super::program::ProgramData;
use crate::constants::*;
use crate::types::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
#[instruction(collection_id: CollectionType)]
pub struct CreateSplNft<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
    mut,
    seeds = [MY_APP_PREFIX, PROGRAM_STATE],
    bump
    )]
    pub program: Box<Account<'info, ProgramData>>,
    /// CHECK: This is a PDA used as the mint authority
    #[account(mut, seeds = [MY_APP_PREFIX, MINT_AUTHORITY], bump = program.authority_bump)]
    pub mint_authority: AccountInfo<'info>,
    /// CHECK: This is a PDA used as the mint authority
    #[account(
        init,
        mint::decimals = 0,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
        payer = creator,
        seeds = [MY_APP_PREFIX, MINT, &[collection_id as u8]],
        bump
    )]
    pub minter: Account<'info, Mint>,
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
    associated_token::authority = mint_authority,
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
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: account constraints checked in account trait
    #[account(address = sysvar::instructions::ID)]
    pub sysvar_instructions: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: This is the metadata
    #[account(address = mpl_token_metadata::ID)]
    pub metadata_program: UncheckedAccount<'info>,
}
