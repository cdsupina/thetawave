use bevy_ecs::event::Event;
use bevy_ecs_macros::Component;
use bevy_math::Vec2;
use serde::Deserialize;
use strum_macros::Display;

/// Type that encompasses all spawnable items
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum ItemType {
    EnhancedPlating,
    /*
    SteelBarrel,
    PlasmaBlasts,
    HazardousReactor,
    WarpThruster,
    Tentaclover,
    DefenseSatellite,
    DoubleBarrel,
    YithianPlague,
    Spice,
    StructureReinforcement,
    BlasterSizeEnhancer,
    FrequencyAugmentor,
    TractorBeam,
    BlastRepeller,
    */
}

#[derive(Component)]
pub struct ItemComponent {
    pub item_type: ItemType,
}

#[derive(Event)]
pub struct SpawnItemEvent {
    pub item_type: ItemType,
    pub position: Vec2,
}
