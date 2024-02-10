use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, MintTo, Token, TokenAccount, Transfer},
};
use super::program::ProgramData;
use crate::constants::*;

#[derive(Accounts)]
pub struct InitializeEnergyBooster<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    /// CHECK: No problem it is the pda passed as ref
    #[account(mut, seeds = [MY_APP_PREFIX, MINT_AUTHORITY], bump)]
    pub mint_authority: AccountInfo<'info>,
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
    // pub mint_shield_booster: Box<Account<'info, Mint>>,
    // #[account(
    // init,
    // payer = creator,
    // mint::decimals = 0,
    // mint::authority = mint_authority,
    //
    // seeds = [MY_APP_PREFIX, MINT, 3_u8.to_le_bytes().as_ref()],
    // bump,
    // )]
    // pub mint_points_booster: Box<Account<'info, Mint>>,
    // #[account(
    // init,
    // payer = creator,
    // mint::decimals = 0,
    // mint::authority = mint_authority,
    //
    // seeds = [MY_APP_PREFIX, MINT, 4_u8.to_le_bytes().as_ref()],
    // bump,
    // )]
    // pub mint_fighter: Box<Account<'info, Mint>>,
    // #[account(
    // init,
    // payer = creator,
    // mint::decimals = 0,
    // mint::authority = mint_authority,
    //
    // seeds = [MY_APP_PREFIX, MINT, 5_u8.to_le_bytes().as_ref()],
    // bump,
    // )]
    // pub mint_champions_pass: Box<Account<'info, Mint>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}
