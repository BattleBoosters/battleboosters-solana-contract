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
pub enum FighterCategory {
    Striker,
    Grappler,
}

impl fmt::Display for FighterCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variant_str = match self {
            FighterCategory::Striker => "Striker",
            FighterCategory::Grappler => "Grappler",
        };
        write!(f, "{}", variant_str)
    }
}

impl FighterCategory {
    pub fn from_name(index: &str) -> Option<FighterCategory> {
        match index {
            "Striker" => Some(FighterCategory::Striker),
            "Grappler" => Some(FighterCategory::Grappler),

            _ => None, // Return None if the index is out of bounds
        }
    }
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub enum FighterType {
    Boxer,
    MuayThai,
    Taekwondo,
    Karate,
    Judo,
    Wrestling,
    BrazilianJiuJitsu,
    Sambo,
}

impl fmt::Display for FighterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variant_str = match self {
            FighterType::Boxer => "Boxer",
            FighterType::MuayThai => "MuayThai",
            FighterType::Taekwondo => "Taekwondo",
            FighterType::Karate => "Karate",
            FighterType::Judo => "Judo",
            FighterType::Wrestling => "Wrestling",
            FighterType::BrazilianJiuJitsu => "BrazilianJiuJitsu",
            FighterType::Sambo => "Sambo",
        };
        write!(f, "{}", variant_str)
    }
}

impl FighterType {
    pub fn from_index(index: usize) -> Option<FighterType> {
        match index {
            0 => Some(FighterType::Boxer),
            1 => Some(FighterType::MuayThai),
            2 => Some(FighterType::Taekwondo),
            3 => Some(FighterType::Karate),
            4 => Some(FighterType::Judo),
            5 => Some(FighterType::Wrestling),
            6 => Some(FighterType::BrazilianJiuJitsu),
            7 => Some(FighterType::Sambo),
            _ => None, // Return None if the index is out of bounds
        }
    }

    pub fn from_name(index: &str) -> Option<FighterType> {
        match index {
            "Boxer" => Some(FighterType::Boxer),
            "MuayThai" => Some(FighterType::MuayThai),
            "Taekwondo" => Some(FighterType::Taekwondo),
            "Karate" => Some(FighterType::Karate),
            "Judo" => Some(FighterType::Judo),
            "Wrestling" => Some(FighterType::Wrestling),
            "BrazilianJiuJitsu" => Some(FighterType::BrazilianJiuJitsu),
            "Sambo" => Some(FighterType::Sambo),
            _ => None, // Return None if the index is out of bounds
        }
    }

    // Determine the category of the fighter
    pub fn category(&self) -> FighterCategory {
        match self {
            FighterType::Boxer
            | FighterType::MuayThai
            | FighterType::Taekwondo
            | FighterType::Karate => FighterCategory::Striker,

            FighterType::Judo
            | FighterType::Wrestling
            | FighterType::BrazilianJiuJitsu
            | FighterType::Sambo => FighterCategory::Grappler,
        }
    }
}
