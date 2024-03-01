use crate::constants::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeRarity<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        seeds = [MY_APP_PREFIX, RARITY],
        bump,
        space = 8 + 140 + 50 + 5 + 5 + 1
    )]
    pub rarity: Account<'info, RarityData>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RarityData {
    /// Rarity tiers for NFTs fighter with associated stats
    pub fighter: Vec<RarityFighter>,
    /// Rarity tiers for NFTs booster with associated stats
    pub booster: Vec<RarityBooster>,
    /// Drop probabilities for each NFTs fighter rarity tier, represented as percentage
    pub fighter_probabilities: Vec<u8>,
    /// Drop probabilities for each NFTs booster rarity tier, represented as percentage
    pub booster_probabilities: Vec<u8>,
    /// This data prevent re-initialization
    pub is_initialized: bool,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Stats {
    pub min: u32,
    pub max: u32,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub enum RarityFighter {
    Common {
        energy: Stats,
        power: Stats,
        lifespan: Stats,
    },
    Uncommon {
        energy: Stats,
        power: Stats,
        lifespan: Stats,
    },
    Rare {
        energy: Stats,
        power: Stats,
        lifespan: Stats,
    },
    Epic {
        energy: Stats,
        power: Stats,
        lifespan: Stats,
    },
    Legendary {
        energy: Stats,
        power: Stats,
        lifespan: Stats,
    },
}

// #[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
// pub enum Booster {
//     Points { rarity: RarityBooster },
//     Shield { rarity: RarityBooster },
//     Energy { rarity: RarityBooster },
// }

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub enum RarityBooster {
    Common { value: Stats },
    Uncommon { value: Stats },
    Rare { value: Stats },
    Epic { value: Stats },
    Legendary { value: Stats },
}
