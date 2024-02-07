use serde::Deserialize;
use strum_macros::Display;

/// Type that encompasses all spawnable consumables
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum ConsumableType {
    Money1,
    Money3,
    HealthWrench,
    Armor,
    GainProjectiles,
}
