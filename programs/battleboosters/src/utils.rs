use crate::errors::ErrorCode;
use anchor_lang::prelude::*;

pub fn only_admin(creator: &Pubkey, admin: &Pubkey) -> Result<()> {
    require!(creator == admin, ErrorCode::Unauthorized);
    Ok(())
}
