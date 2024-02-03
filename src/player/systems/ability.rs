use bevy::prelude::*;
use bevy_rapier2d::prelude::{ExternalImpulse, Velocity};
use leafwing_input_manager::prelude::ActionState;
use thetawave_interface::audio::SoundEffectType;
use thetawave_interface::input::PlayerAction;
use thetawave_interface::player::PlayerComponent;
use thetawave_interface::weapon::{WeaponComponent, WeaponProjectileData};

use crate::spawnable::{FireWeaponEvent, InitialMotion};

#[allow(clippy::too_many_arguments)]
pub fn player_ability_system(
    mut player_query: Query<(
        &mut PlayerComponent,
        &mut Velocity,
        &Transform,
        &mut ExternalImpulse,
        &ActionState<PlayerAction>,
        Entity,
        &WeaponComponent,
    )>,
    time: Res<Time>,
    mut fire_weapon: EventWriter<FireWeaponEvent>,
) {
    for (
        mut player_component,
        mut player_vel,
        player_trans,
        mut player_ext_impulse,
        action_state,
        entity,
        weapon_component,
    ) in player_query.iter_mut()
    // No-op for players whose special attack is disabled
    {
        let activate_ability_input = action_state.pressed(PlayerAction::SpecialAttack);
        let up = action_state.pressed(PlayerAction::MoveUp);
        let down = action_state.pressed(PlayerAction::MoveDown);
        let left = action_state.pressed(PlayerAction::MoveLeft);
        let right = action_state.pressed(PlayerAction::MoveRight);

        // TODO: Handle player abilities
    }
}
