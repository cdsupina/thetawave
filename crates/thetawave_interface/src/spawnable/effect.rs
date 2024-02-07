use super::ConsumableType;
use serde::Deserialize;
use std::default::Default;
use strum_macros::Display;

/// Type that encompasses all spawnable effects
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display, Default)]
pub enum EffectType {
    AllyBlastExplosion,
    AllyBlastDespawn,
    #[default]
    MobExplosion, // defaults to mob explosion
    ConsumableDespawn,
    EnemyBlastExplosion,
    EnemyBlastDespawn,
    EnemyBulletExplosion,
    BarrierGlow,
    AllyBulletDespawn,
    EnemyBulletDespawn,
    AllyBulletExplosion,
    Text(TextEffectType),
}

/// Subtype of effect for text effects
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum TextEffectType {
    DamageDealt,
    ConsumableCollected(ConsumableType),
}
