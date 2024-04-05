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
        space = 8 + 140 + 50 + 50 + 50 + 4 + (3 * 6) + 1
    )]
    pub rarity: Account<'info, RarityData>,
    pub system_program: Program<'info, System>,
}
/*
   TODO: Create update ratity Accounts
*/
// #[derive(Accounts)]
// pub struct UpdateRarity<'info> {
//     TODO
// }

#[account]
pub struct RarityData {
    /// Rarity tiers for NFTs fighter with associated stats
    pub fighter: Vec<RarityFighter>,
    ///// Rarity tiers for NFTs booster with associated stats
    //pub energy_booster: Vec<RarityBooster>,
    /// Rarity tiers for NFTs booster with associated stats
    pub shield_booster: Vec<RarityBooster>,
    /// Rarity tiers for NFTs booster with associated stats
    pub points_booster: Vec<RarityBooster>,
    /// Drop probabilities for each NFTs rarity tier, represented as percentage
    pub probability_tiers: Vec<TierProbabilities>,
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
        //energy: Stats,
        power: Stats,
        lifespan: Stats,
    },
    Uncommon {
        //energy: Stats,
        power: Stats,
        lifespan: Stats,
    },
    Rare {
        //energy: Stats,
        power: Stats,
        lifespan: Stats,
    },
    Epic {
        //energy: Stats,
        power: Stats,
        lifespan: Stats,
    },
    Legendary {
        //energy: Stats,
        power: Stats,
        lifespan: Stats,
    },
}

impl fmt::Display for RarityFighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variant_str = match self {
            RarityFighter::Common { .. } => "Common",
            RarityFighter::Uncommon { .. } => "Uncommon",
            RarityFighter::Rare { .. } => "Rare",
            RarityFighter::Epic { .. } => "Epic",
            RarityFighter::Legendary { .. } => "Legendary",
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
            RarityBooster::Common { .. } => "Common",
            RarityBooster::Uncommon { .. } => "Uncommon",
            RarityBooster::Rare { .. } => "Rare",
            RarityBooster::Epic { .. } => "Epic",
            RarityBooster::Legendary { .. } => "Legendary",
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum TierProbabilities {
    Tier1(Vec<u8>), // Tier1 has the highest probability for minting rare mintable game assets often tied to MainCards.
    Tier2(Vec<u8>), // Tier2 has a moderate probability for minting rare mintable game assets often tied to Prelims.
    Tier3(Vec<u8>), // Tier3 has the lowest probability for minting rare mintable game assets often tied to Early Prelims.
}

impl TierProbabilities {
    // Returns the probability vector associated with the instance's tier.
    pub fn get_probability_for_tier(&self) -> Vec<u8> {
        match self {
            TierProbabilities::Tier1(probs) => probs.clone(),
            TierProbabilities::Tier2(probs) => probs.clone(),
            TierProbabilities::Tier3(probs) => probs.clone(),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
pub enum TierType {
    Tier1,
    Tier2,
    Tier3,
}
impl RarityData {
    // Function to get probabilities for a specified tier
    pub fn get_probability_by_tier(&self, tier_type: TierType) -> Option<TierProbabilities> {
        for tier in &self.probability_tiers {
            match (tier, tier_type) {
                (TierProbabilities::Tier1(_), TierType::Tier1)
                | (TierProbabilities::Tier2(_), TierType::Tier2)
                | (TierProbabilities::Tier3(_), TierType::Tier3) => return Some(tier.clone()),
                _ => continue,
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::state::rarity::{RarityData, TierProbabilities, TierType};

    #[test]
    fn it_get_probability_tier() {
        let data: RarityData = RarityData {
            fighter: vec![],
            //energy_booster: vec![],
            shield_booster: vec![],
            points_booster: vec![],
            probability_tiers: vec![
                TierProbabilities::Tier1(vec![1, 2, 3, 4, 6]),
                TierProbabilities::Tier2(vec![1, 2, 3, 4, 2]),
                TierProbabilities::Tier3(vec![1, 2, 3, 4, 1]),
            ],
            is_initialized: false,
        };

        let prob = data.get_probability_by_tier(TierType::Tier1);
        let tier_probs: TierProbabilities = prob.unwrap();

        assert_eq!(tier_probs.get_probability_for_tier(), vec![1, 2, 3, 4, 6]);

        // println!("{:?}", tier_probs.get_probability_for_tier());
    }
}
