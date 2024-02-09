use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, MintTo, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct CreateNft<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        seeds = [MY_APP_PREFIX, ENERGY_BOOSTER],
        bump,
        mint::decimals = 0,
        mint::authority = creator.key(),
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = creator,
        associated_token::mint = mint,
        associated_token::authority = creator,
    )]
    pub associated_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
