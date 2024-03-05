use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter},
};
pub use thetawave_interface::character_selection::PlayerJoinEvent;
use thetawave_interface::game::historical_metrics::{MobsKilledByPlayerCacheT, DEFAULT_USER_ID};

use crate::GameEnterSet;
use thetawave_interface::states;

mod character_selection;
mod game;
mod game_over;
mod instructions;
mod main_menu;
mod pause_menu;
mod victory;

pub use self::character_selection::{
    player_join_system, select_character_system, setup_character_selection_system,
};
use self::{character_selection::toggle_tutorial_system, game::GameUiPlugin};

use self::instructions::setup_instructions_system;
pub use self::{
    game_over::setup_game_over_system,
    main_menu::{bouncing_prompt_system, setup_main_menu_system, BouncingPromptComponent},
    pause_menu::setup_pause_system,
    victory::setup_victory_system,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerJoinEvent>();

        app.add_plugins(GameUiPlugin);

        app.add_systems(Update, bouncing_prompt_system);

        app.add_systems(OnEnter(states::AppStates::MainMenu), setup_main_menu_system);

        app.add_systems(
            OnEnter(states::AppStates::Instructions),
            setup_instructions_system,
        );

        app.add_systems(
            OnEnter(states::AppStates::CharacterSelection),
            setup_character_selection_system,
        );

        app.add_systems(
            Update,
            (
                player_join_system,
                select_character_system,
                toggle_tutorial_system,
            )
                .run_if(in_state(states::AppStates::CharacterSelection)),
        );

        app.add_systems(OnEnter(states::AppStates::GameOver), setup_game_over_system);

        app.add_systems(OnEnter(states::AppStates::Victory), setup_victory_system);

        app.add_systems(OnEnter(states::GameStates::Paused), setup_pause_system);
    }
}

// Consistently format mob+kill-count pairs.
fn pprint_mob_kills_from_data(data: &MobsKilledByPlayerCacheT) -> String {
    match (*data).get(&DEFAULT_USER_ID) {
        None => String::from("No mobs killed"),
        Some(mob_kill_counts) => mob_kill_counts
            .iter()
            .map(|(mobtype, n)| format!("{mobtype}: {n}"))
            .collect::<Vec<String>>()
            .join("\n"),
    }
}
