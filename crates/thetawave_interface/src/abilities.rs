use bevy_ecs::{bundle::Bundle, component::Component, system::Resource};
use bevy_time::Timer;
use serde::Deserialize;

use crate::{audio::SoundEffectType, spawnable::ProjectileType, weapon::SpreadPattern};

pub enum AbilityType {
    Charge,
    StandardBlast,
    StandardBullet,
    MegaBlast,
}

#[derive(Resource, Deserialize)]
pub struct AbilitiesResource {
    pub charge_ability: ChargeAbilityBundle,
    pub mega_blast_ability: StandardWeaponAbilityBundle,
    pub standard_blast_ability: StandardWeaponAbilityBundle,
    pub standard_bullet_ability: StandardWeaponAbilityBundle,
}

#[derive(Component, Deserialize)]
pub enum AbilitySlotIDComponent {
    One,
    Two,
}

/// Charge ability bundle for spawning entity as a child of player component
#[derive(Bundle, Deserialize)]
pub struct ChargeAbilityBundle {
    slot: AbilitySlotIDComponent,
    ability: ChargeAbilityComponent,
}

#[derive(Component, Deserialize)]
pub struct ChargeAbilityComponent {
    pub action_timer: Timer,
    pub incoming_damage_multiplier: f32,
    pub impulse: f32,
}

/// Standard weapon bundle for spawning entity as a child of player component
#[derive(Bundle, Deserialize)]
pub struct StandardWeaponAbilityBundle {
    slot: AbilitySlotIDComponent,
    ability: StandardWeaponAbilityComponent,
}

#[derive(Component, Deserialize)]
pub struct StandardWeaponAbilityComponent {
    pub spread_pattern: SpreadPattern,
    pub damage_multiplier: f32,
    pub ammunition: ProjectileType,
    pub speed_multiplier: f32,
    pub direction: f32,
    pub despawn_time_multiplier: f32,
    pub size_multiplier: f32,
    pub count_multiplier: f32,
    pub sound: SoundEffectType,
}
