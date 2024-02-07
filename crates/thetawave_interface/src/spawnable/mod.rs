use bevy_ecs_macros::Component;
use bevy_math::Vec2;
use serde::Deserialize;
use std::default::Default;
use strum_macros::Display;

pub mod consumable;
pub mod effect;
pub mod item;
pub mod mob;
pub mod projectile;

pub use self::consumable::*;
pub use self::effect::*;
pub use self::item::*;
pub use self::mob::*;
pub use self::projectile::*;

/// Type that encompasses all spawnable entities
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum SpawnableType {
    Projectile(ProjectileType),
    Consumable(ConsumableType),
    Item(ItemType),
    Effect(EffectType),
    Mob(MobType),
    MobSegment(MobSegmentType),
}

impl Default for SpawnableType {
    /// Money1 is default so that SpawnableComponent can derive default
    fn default() -> Self {
        SpawnableType::Consumable(ConsumableType::Money1)
    }
}

/// Tag for applying an in-game thing to the closest player based on the player's "gravity" params.
#[derive(Component)]
pub struct AttractToClosestPlayerComponent;

#[derive(Deserialize, Clone, Debug)]
pub enum SpawnPosition {
    Global(Vec2),
    Local(Vec2),
}
