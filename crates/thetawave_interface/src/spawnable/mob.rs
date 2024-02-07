use bevy_ecs::{entity::Entity, event::Event};
use bevy_math::{Quat, Vec2};
use serde::Deserialize;
use strum_macros::{Display, EnumString};

/// Type that encompasses all spawnable enemy mobs
#[derive(Deserialize, EnumString, Display, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum EnemyMobType {
    Pawn,
    Drone,
    StraferRight,
    StraferLeft,
    MissileLauncher,
    Missile,
    CrustlingRight,
    CrustlingLeft,
    Repeater,
    Shelly,
}

/// Type that encompasses all spawnable mobs
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum MobType {
    Enemy(EnemyMobType),
    Ally(AllyMobType),
    Neutral(NeutralMobType),
}

impl MobType {
    pub fn get_name(&self) -> String {
        match self {
            MobType::Enemy(enemy_type) => match enemy_type {
                EnemyMobType::Pawn => "Pawn",
                EnemyMobType::Drone => "Drone",
                EnemyMobType::StraferRight | EnemyMobType::StraferLeft => "Strafer",
                EnemyMobType::MissileLauncher => "Missile Launcher",
                EnemyMobType::Missile => "Missile",
                EnemyMobType::CrustlingRight | EnemyMobType::CrustlingLeft => "Crustling",
                EnemyMobType::Repeater => "Repeater",
                EnemyMobType::Shelly => "Shelly",
            },
            MobType::Ally(ally_type) => match ally_type {
                AllyMobType::Hauler2 => "Hauler",
                AllyMobType::Hauler3 => "Hauler",
                AllyMobType::TutorialHauler2 => "Hauler",
            },
            MobType::Neutral(neutral_type) => match neutral_type {
                NeutralMobType::MoneyAsteroid => "Money Asteroid",
                NeutralMobType::TutorialDrone => "Tutorial Drone",
            },
        }
        .to_string()
    }
}

/// Sends information about destroyed mobs
/// Includes information that is lost when the entity is destroyed
#[derive(Event)]
pub struct MobDestroyedEvent {
    pub mob_type: MobType,
    pub entity: Entity,
    pub is_boss: bool,
}

/// Event for spawning mobs
#[derive(Event)]
pub struct SpawnMobEvent {
    /// Type of mob to spawn
    pub mob_type: MobType,
    /// Position to spawn mob
    pub position: Vec2,

    pub rotation: Quat,

    pub boss: bool,
}

#[derive(Event)]
pub struct MobSegmentDestroyedEvent {
    pub mob_segment_type: MobSegmentType,
    pub entity: Entity,
}

/// Type that encompasses all spawnable neutral mobs
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum NeutralMobType {
    MoneyAsteroid,
    TutorialDrone,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum MobSegmentType {
    Neutral(NeutralMobSegmentType),
    Enemy(EnemyMobSegmentType),
}

/// Type that encompasses all spawnable ally mobs
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum AllyMobType {
    Hauler2,
    Hauler3,
    TutorialHauler2,
}

/// Type that encompasses all spawnable ally mob segments
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum NeutralMobSegmentType {
    HaulerBack,
    HaulerMiddle,
    TutorialHaulerBack,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum EnemyMobSegmentType {
    CrustlingTentacle1,
    CrustlingTentacle2,
    CrustlingTentacle3,
    RepeaterBody,
    RepeaterRightShoulder,
    RepeaterLeftShoulder,
    RepeaterRightArm,
    RepeaterLeftArm,
    RepeaterRightClaw,
    RepeaterLeftClaw,
}
