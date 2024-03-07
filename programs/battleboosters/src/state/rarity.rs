use crate::constants::*;
use anchor_lang::prelude::*;
use std::fmt;

#[derive(Accounts)]
pub struct InitializeRarity<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        seeds = [MY_APP_PREFIX, RARITY],
        bump,
        space = 8 + 140 + 50 + 50 + 50 + 5 + 5 + 1
    )]
    pub rarity: Account<'info, RarityData>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RarityData {
    /// Rarity tiers for NFTs fighter with associated stats
    pub fighter: Vec<RarityFighter>,
    /// Rarity tiers for NFTs booster with associated stats
    pub energy_booster: Vec<RarityBooster>,
    /// Rarity tiers for NFTs booster with associated stats
    pub shield_booster: Vec<RarityBooster>,
    /// Rarity tiers for NFTs booster with associated stats
    pub points_booster: Vec<RarityBooster>,
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

impl fmt::Display for RarityFighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variant_str = match self {
            RarityFighter::Common {
                energy,
                power,
                lifespan,
            } => "Common",
            RarityFighter::Uncommon {
                energy,
                power,
                lifespan,
            } => "Uncommon",
            RarityFighter::Rare {
                energy,
                power,
                lifespan,
            } => "Rare",
            RarityFighter::Epic {
                energy,
                power,
                lifespan,
            } => "Epic",
            RarityFighter::Legendary {
                energy,
                power,
                lifespan,
            } => "Legendary",
        };
        write!(f, "{}", variant_str)
    }
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

impl fmt::Display for RarityBooster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variant_str = match self {
            RarityBooster::Common { value } => "Common",
            RarityBooster::Uncommon { value } => "Uncommon",
            RarityBooster::Rare { value } => "Rare",
            RarityBooster::Epic { value } => "Epic",
            RarityBooster::Legendary { value } => "Legendary",
        };
        write!(f, "{}", variant_str)
    }
}
// impl Default for Stats {
//     fn default() -> Self {
//         Self {
//             min: 0, // Default value for min
//             max: 0, // Default value for max
//         }
//     }
// }
impl RarityBooster {
    // /// Maps a numerical index to a specific RarityBooster variant.
    // /// This is a class method, not meant to be called on an instance.
    // pub fn from_index(index: usize) -> Option<Self> {
    //     match index {
    //         0 => Some(RarityBooster::Common { value: Stats::default() }),
    //         1 => Some(RarityBooster::Uncommon { value: Stats::default() }),
    //         2 => Some(RarityBooster::Rare { value: Stats::default() }),
    //         3 => Some(RarityBooster::Epic { value: Stats::default() }),
    //         4 => Some(RarityBooster::Legendary { value: Stats::default() }),
    //         _ => None,
    //     }
    // }

    /// Checks if the current instance matches a given rarity index.
    /// Useful for filtering/searching within a collection.
    pub fn matches_index(&self, index: usize) -> bool {
        match (self, index) {
            (RarityBooster::Common { .. }, 0) => true,
            (RarityBooster::Uncommon { .. }, 1) => true,
            (RarityBooster::Rare { .. }, 2) => true,
            (RarityBooster::Epic { .. }, 3) => true,
            (RarityBooster::Legendary { .. }, 4) => true,
            _ => false,
        }
    }
}

impl RarityFighter {
    // /// Maps a numerical index to a specific RarityBooster variant.
    // /// This is a class method, not meant to be called on an instance.
    // pub fn from_index(index: usize) -> Option<Self> {
    //     match index {
    //         0 => Some(RarityBooster::Common { value: Stats::default() }),
    //         1 => Some(RarityBooster::Uncommon { value: Stats::default() }),
    //         2 => Some(RarityBooster::Rare { value: Stats::default() }),
    //         3 => Some(RarityBooster::Epic { value: Stats::default() }),
    //         4 => Some(RarityBooster::Legendary { value: Stats::default() }),
    //         _ => None,
    //     }
    // }

    /// Checks if the current instance matches a given rarity index.
    /// Useful for filtering/searching within a collection.
    pub fn matches_index(&self, index: usize) -> bool {
        match (self, index) {
            (RarityFighter::Common { .. }, 0) => true,
            (RarityFighter::Uncommon { .. }, 1) => true,
            (RarityFighter::Rare { .. }, 2) => true,
            (RarityFighter::Epic { .. }, 3) => true,
            (RarityFighter::Legendary { .. }, 4) => true,
            _ => false,
        }
    }
}
