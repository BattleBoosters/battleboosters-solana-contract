use anchor_lang::prelude::*;
use std::fmt;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum CollectionType {
    Energy = 0,
    Shield = 1,
    Points = 2,
    Fighter = 3,
    ChampionsPass = 4,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub struct PurchaseRequest {
    pub nft_type: NftType,
    pub quantity: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub struct OpenRequest {
    pub nft_type: NftType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug, Eq, Hash)]
pub enum NftType {
    Booster,
    FighterPack,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub enum BoosterType {
    Points,
    Shield,
    Energy,
}

impl fmt::Display for BoosterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variant_str = match self {
            BoosterType::Points => "Points",
            BoosterType::Shield => "Shield",
            BoosterType::Energy => "Energy",
        };
        write!(f, "{}", variant_str)
    }
}

impl BoosterType {
    pub fn from_index(index: usize) -> Option<BoosterType> {
        match index {
            0 => Some(BoosterType::Points),
            1 => Some(BoosterType::Shield),
            2 => Some(BoosterType::Energy),
            _ => None, // Return None if the index is out of bounds
        }
    }

    pub fn from_name(index: &str) -> Option<BoosterType> {
        match index {
            "Points" => Some(BoosterType::Points),
            "Shield" => Some(BoosterType::Shield),
            "Energy" => Some(BoosterType::Energy),
            _ => None, // Return None if the index is out of bounds
        }
    }
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub enum FighterType {
    Boxer,
    MuayThai,
    Energy,
}
