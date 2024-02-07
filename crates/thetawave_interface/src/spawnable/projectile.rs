use serde::Deserialize;
use strum_macros::Display;

/// Factions
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum Faction {
    Ally,
    Enemy,
    Neutral,
}

/// Type that encompasses all weapon projectiles
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum ProjectileType {
    Blast(Faction),
    Bullet(Faction),
}

impl ProjectileType {
    pub fn get_faction(&self) -> Faction {
        match self {
            ProjectileType::Blast(faction) => faction.clone(),
            ProjectileType::Bullet(faction) => faction.clone(),
        }
    }
}
