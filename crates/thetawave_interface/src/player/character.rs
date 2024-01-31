use bevy_math::Vec2;
use serde::Deserialize;

use super::ability;
use crate::{
    health::HealthComponent,
    spawnable::SpawnPosition,
    weapon::{SpreadPattern, WeaponComponent},
};

/// The playable character types. To a player, these will have different appearances and abilities.
#[derive(Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub enum CharacterType {
    Captain,
    Juggernaut,
}

/// Contains data necessary to create a player entity.
/// A character is chosen at the beginning of the game.
/// The base stats of the player are provided from the character.
/// Other data such as sprite sheets are also included with the character.
#[derive(Deserialize, Clone)]
pub struct Character {
    /// Base acceleration
    pub acceleration: Vec2,
    /// Base deceleration
    pub deceleration: Vec2,
    /// Base speed
    pub speed: Vec2,
    /// Collider size (relative to the sprite size)
    pub collider_dimensions: Vec2,
    /// Density of the collider (mass of collider is proportional to its size)
    pub collider_density: f32,
    /// Character type
    pub character_type: CharacterType,
    /// Health of the player
    pub health: usize,
    /// Shields of the player
    pub shields: usize,
    /// Shields recharging rate
    pub shields_recharge_rate: f32,
    /// Amount of damage dealt on contact
    pub collision_damage: usize,
    /// Distance to attract items and consumables
    pub attraction_distance: f32,
    /// Acceleration applied to items and consumables in attraction distance
    pub attraction_acceleration: f32,
    /// Amount of money character has collected
    pub money: usize,
    /// Ability cooldown time
    pub ability_period: f32,
    /// All of the player abilities
    pub ability_loadout: ability::AbilityLoadout,
    pub damage: usize,
    pub projectile_spawn_position: SpawnPosition,
    pub projectile_speed: f32,
    pub projectile_despawn_time: f32,
    pub projectile_count: usize,
    pub projectile_spread_pattern: SpreadPattern,
    pub projectile_size: f32,
}

impl From<&Character> for HealthComponent {
    fn from(character: &Character) -> Self {
        HealthComponent::new(
            character.health,
            character.shields,
            character.shields_recharge_rate,
        )
    }
}
