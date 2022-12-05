use bevy::prelude::*;
use serde::Deserialize;
use std::{collections::HashMap, time::Duration};

use crate::{
    arena::MobReachedBottomGateEvent,
    misc::Health,
    spawnable::{self, BossType},
    states::AppStates,
    tools::weighted_rng,
    ui::EndGameTransitionResource,
};

use super::formation;

/// Structure stored in data file to describe level
pub type LevelsResourceData = HashMap<LevelType, LevelData>;

/// Resource for storing defined predefined levels
#[derive(Clone, Resource)]
pub struct LevelsResource {
    /// Leveltypes maped to levels
    pub levels: HashMap<LevelType, Level>,
}

impl From<LevelsResourceData> for LevelsResource {
    fn from(resource_data: LevelsResourceData) -> Self {
        LevelsResource {
            levels: resource_data
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

/// Names of levels that are used to be mapped to level structs
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum LevelType {
    EarthOrbit,
    TestLevel,
}

/// Data used to initialize levels
#[derive(Deserialize)]
pub struct LevelData {
    /// timeline of the phases of the level
    pub timeline: LevelTimeline,
    /// objective of the level (besides surviving)
    pub objective: ObjectiveType,
}

/// Event to alert when level has been completed
pub struct LevelCompletedEvent;

/// Struct to manage a level
#[derive(Clone, Debug)]
pub struct Level {
    timeline_idx: usize,
    /// Timeline
    pub timeline: LevelTimeline,
    /// Tracks time of phases
    pub phase_timer: Option<Timer>,
    /// Tracks time between spawns
    pub spawn_timer: Option<Timer>,
    /// Level objective
    pub objective: ObjectiveType,
}

/// Types of objectives for a level
#[derive(Clone, Deserialize, Debug)]
pub enum ObjectiveType {
    Defense(Health),
}

impl From<LevelData> for Level {
    fn from(data: LevelData) -> Self {
        Level {
            timeline_idx: 0,
            timeline: data.timeline,
            phase_timer: None,
            spawn_timer: None,
            objective: data.objective,
        }
    }
}

impl Level {
    /// Get a string name of the phase
    pub fn get_phase_name(&self) -> String {
        match &self.timeline.phases[self.timeline_idx].phase_type {
            LevelPhaseType::FormationSpawn { formation_pool, .. } => {
                format!("FormationSpawn({:?})", formation_pool)
            }
            LevelPhaseType::Break { .. } => "Break".to_string(),
            LevelPhaseType::Boss { boss_type, .. } => format!("Boss[{:?}]", boss_type),
        }
    }

    /// Get the index of the current phase
    pub fn get_phase_number(&self) -> String {
        self.timeline_idx.to_string()
    }

    /// Tick the level - manage the objective and phase
    #[allow(clippy::too_many_arguments)]
    pub fn tick(
        &mut self,
        delta: Duration,
        spawn_formation: &mut EventWriter<formation::SpawnFormationEvent>,
        spawn_boss: &mut EventWriter<spawnable::SpawnBossEvent>,
        level_completed: &mut EventWriter<LevelCompletedEvent>,
        mob_reached_bottom: &mut EventReader<MobReachedBottomGateEvent>,
        formation_pools: &formation::FormationPoolsResource,
        end_game_trans_resource: &mut EndGameTransitionResource,
    ) {
        // handle each of the objective types
        #[allow(clippy::single_match)]
        match &mut self.objective {
            ObjectiveType::Defense(health) => {
                // iterate through all the mobs that have reached the bottom
                for event in mob_reached_bottom.iter() {
                    // heal or take damage based on the damage amount
                    if event.0 > 0.0 {
                        health.take_damage(event.0);
                    } else {
                        health.heal(-event.0);
                    }

                    // end the game if defense dies
                    if health.is_dead() {
                        end_game_trans_resource.start(AppStates::GameOver);
                    }
                }
            }
        }

        // check if the current phase if valid and handle the phase
        if let Some(current_phase) = self.get_current_phase() {
            // handle the phase based on the type
            match &current_phase.phase_type {
                LevelPhaseType::FormationSpawn {
                    time: _,
                    initial_delay: _,
                    formation_pool,
                } => {
                    let event_formation_pool = formation_pool.clone();

                    // tick spawn timer and spawn from formation pool when finished
                    if self
                        .spawn_timer
                        .as_mut()
                        .unwrap()
                        .tick(delta)
                        .just_finished()
                    {
                        // get weights of each of the formations in the formation pool
                        let weights = formation_pools.formation_pools[&event_formation_pool]
                            .iter()
                            .map(|x| x.weight)
                            .collect();

                        // get random formation index based on the weights
                        let random_idx = weighted_rng(weights);

                        // set spawn timer duration to the period of the selected formation
                        self.spawn_timer
                            .as_mut()
                            .unwrap()
                            .set_duration(Duration::from_secs_f32(
                                formation_pools.formation_pools[&event_formation_pool][random_idx]
                                    .period,
                            ));

                        // spawn the spawnables from the selected formation
                        spawn_formation.send(formation::SpawnFormationEvent {
                            formation: formation_pools.formation_pools[&event_formation_pool]
                                [random_idx]
                                .clone(),
                        });
                    }
                }
                LevelPhaseType::Boss {
                    boss_type,
                    initial_delay: _,
                    is_defeated: _,
                } => {
                    let boss_type = boss_type.clone();

                    // spawn the boss after the spawn timer has finished
                    if self
                        .spawn_timer
                        .as_mut()
                        .unwrap()
                        .tick(delta)
                        .just_finished()
                    {
                        // spawn the boss
                        spawn_boss.send(spawnable::SpawnBossEvent {
                            boss_type,
                            position: Vec2::new(0.0, 1750.0),
                        })
                    }
                }
                _ => {}
            }

            // if phase timer for phase, tick the timer
            if let Some(phase_timer) = &mut self.phase_timer {
                // check of the phase just ended
                if phase_timer.tick(delta).just_finished() {
                    // set level to next phase in timeline
                    if self.timeline.phases.len() > self.timeline_idx + 1 {
                        self.timeline_idx += 1;
                    } else {
                        // send level completed event when level is completed
                        level_completed.send(LevelCompletedEvent);
                    }
                    // setup the next phase
                    self.setup_next_phase();
                } else {
                    // continue with the current phase
                }
            }
        } else {
            println!("Timeline index: {:?}", self.timeline_idx);
            panic!("Something is wrong. There is not phase at the current timeline index.")
        }
    }

    /// Setup the next phase of the level
    fn setup_next_phase(&mut self) {
        // get the current phase (phase should already be set to the next phase)
        let current_phase = self.get_current_phase();

        if let Some(current_phase) = current_phase {
            // perform setup based on the phase type
            match current_phase.phase_type.clone() {
                LevelPhaseType::FormationSpawn {
                    time,
                    initial_delay,
                    formation_pool: _,
                } => {
                    self.spawn_timer =
                        Some(Timer::from_seconds(initial_delay, TimerMode::Repeating));
                    self.phase_timer = Some(Timer::from_seconds(time, TimerMode::Once));
                }
                LevelPhaseType::Break { time } => {
                    self.spawn_timer = None;
                    self.phase_timer = Some(Timer::from_seconds(time, TimerMode::Once));
                }
                LevelPhaseType::Boss {
                    boss_type: _,
                    initial_delay,
                    is_defeated: _,
                } => {
                    self.spawn_timer = Some(Timer::from_seconds(initial_delay, TimerMode::Once));
                    self.phase_timer = None;
                }
            }
        }
    }

    /// Get the current phase of the level
    fn get_current_phase(&self) -> Option<&LevelPhase> {
        self.timeline.phases.get(self.timeline_idx)
    }
}

/// Level timeline for carrying phases of the level
#[derive(Deserialize, Clone, Debug)]
pub struct LevelTimeline {
    /// level phases
    pub phases: Vec<LevelPhase>,
}

/// A defined section of the level
#[derive(Deserialize, Clone, Debug)]
pub struct LevelPhase {
    /// phase type
    pub phase_type: LevelPhaseType,
}

/// Describes a distinct portion of the level
#[derive(Deserialize, Clone, Debug)]
pub enum LevelPhaseType {
    FormationSpawn {
        time: f32,
        initial_delay: f32,
        formation_pool: formation::FormationPoolType,
    },
    Break {
        time: f32,
    },
    Boss {
        boss_type: BossType,
        initial_delay: f32,
        is_defeated: bool,
    },
}

/// Handles the progression of the level
#[allow(clippy::too_many_arguments)]
pub fn level_system(
    mut run_resource: ResMut<super::RunResource>,
    mut spawn_formation: EventWriter<formation::SpawnFormationEvent>,
    mut spawn_boss: EventWriter<spawnable::SpawnBossEvent>,
    mut level_completed: EventWriter<LevelCompletedEvent>,
    mut mob_reached_bottom: EventReader<MobReachedBottomGateEvent>,
    formation_pools: Res<formation::FormationPoolsResource>,
    time: Res<Time>,
    mut end_game_trans_resource: ResMut<EndGameTransitionResource>,
) {
    // tick the run if ready and the game isn't over
    if run_resource.is_ready() && !end_game_trans_resource.start {
        run_resource.tick(
            time.delta(),
            &mut spawn_formation,
            &mut spawn_boss,
            &mut level_completed,
            &mut mob_reached_bottom,
            &formation_pools,
            &mut end_game_trans_resource,
        );
    }
}

/// Progress to the next level when current level is completed
pub fn next_level_system(
    mut level_completed: EventReader<LevelCompletedEvent>,
    mut end_game_trans_resource: ResMut<EndGameTransitionResource>,
) {
    // TODO: add case for going to next level, instead of instantly winning after one level
    for _level_completed in level_completed.iter() {
        end_game_trans_resource.start(AppStates::Victory);
    }
}

/// Setup first level of the game using values from the first level (phase timer spawn timer, etc)
pub fn setup_first_level(mut run_resource: ResMut<super::RunResource>) {
    if let Some(level) = &mut run_resource.level {
        match &level.timeline.phases[0].phase_type {
            LevelPhaseType::FormationSpawn {
                time,
                initial_delay,
                formation_pool: _,
            } => {
                level.spawn_timer = Some(Timer::from_seconds(*initial_delay, TimerMode::Repeating));
                level.phase_timer = Some(Timer::from_seconds(*time, TimerMode::Once));
            }
            LevelPhaseType::Break { time } => {
                level.spawn_timer = None;
                level.phase_timer = Some(Timer::from_seconds(*time, TimerMode::Once));
            }
            LevelPhaseType::Boss {
                boss_type: _,
                initial_delay,
                is_defeated: _,
            } => {
                level.spawn_timer = Some(Timer::from_seconds(*initial_delay, TimerMode::Once));
                level.phase_timer = None;
            }
        }
    }
}
