use bevy::{prelude::*, time::Stopwatch};
use bevy_kira_audio::{AudioEasing, AudioTween};
use serde::Deserialize;
use std::{
    collections::{HashMap, VecDeque},
    time::Duration,
};
use thetawave_interface::{
    audio::{BGMusicType, ChangeBackgroundMusicEvent},
    spawnable::MobType,
};

use crate::spawnable::{BossesDestroyedEvent, SpawnMobEvent};

use super::{objective::Objective, FormationPoolsResource, SpawnFormationEvent};

#[derive(Resource, Deserialize)]
pub struct PremadeLevelsResource {
    pub levels_data: HashMap<String, LevelData>,
}

/// A defined section of the level
#[derive(Deserialize, Clone, Debug)]
pub struct LevelPhase {
    /// phase type
    pub phase_type: LevelPhaseType,
    /// music to play during phase
    pub bg_music_transition: Option<BGMusicTransition>,
    #[serde(default)]
    pub phase_time: Stopwatch,
}

/// Background music transition
#[derive(Deserialize, Clone, Debug)]
pub struct BGMusicTransition {
    pub loop_from: Option<f64>,
    pub bg_music_type: Option<BGMusicType>,
    pub fade_in: Option<f32>,
    pub fade_out: Option<f32>,
}

impl From<&BGMusicTransition> for ChangeBackgroundMusicEvent {
    fn from(value: &BGMusicTransition) -> Self {
        ChangeBackgroundMusicEvent {
            bg_music_type: value.bg_music_type.clone(),
            loop_from: value.loop_from,
            fade_in_tween: value.fade_in.map(|fade_in| {
                AudioTween::new(Duration::from_secs_f32(fade_in), AudioEasing::Linear)
            }),
            fade_out_tween: value.fade_out.map(|fade_out| {
                AudioTween::new(Duration::from_secs_f32(fade_out), AudioEasing::Linear)
            }),
        }
    }
}

/// Describes a distinct portion of the level
#[derive(Deserialize, Clone, Debug)]
pub enum LevelPhaseType {
    FormationSpawn {
        phase_timer: Timer,
        spawn_timer: Timer,
        formation_pool: String,
    },
    Break {
        phase_timer: Timer,
    },
    Boss {
        mob_type: MobType,
        position: Vec2,
        spawn_timer: Timer,
    },
}

/// Data used to initialize levels
#[derive(Deserialize)]
pub struct LevelData {
    /// timeline of the phases of the level
    pub phases: Vec<LevelPhase>,
    /// objective of the level (besides surviving)
    pub objective: Objective,
}

/// Event to alert when level has been completed
#[derive(Event)]
pub struct LevelCompletedEvent;

pub type LevelPhases = VecDeque<LevelPhase>;

/// Struct to manage a level
#[derive(Clone, Debug)]
pub struct Level {
    /// Phases that have been completed so far in the run
    pub completed_phases: LevelPhases,
    /// Phase that is currently active
    pub current_phase: Option<LevelPhase>,
    /// Phases that have yet to be played in the level
    pub queued_phases: LevelPhases,
    /// Objective is an additional failure condition for a level
    pub objective: Objective,
    /// Tracks how long the player has been in the level
    pub level_time: Stopwatch,
}

impl From<&LevelData> for Level {
    fn from(data: &LevelData) -> Self {
        Level {
            completed_phases: vec![].into(),
            current_phase: None,
            queued_phases: data.phases.clone().into(),
            objective: data.objective.clone(),
            level_time: Stopwatch::new(),
        }
    }
}

impl Level {
    pub fn cycle_phase(&mut self) -> bool {
        // clone the current phase (if it exists) into the back of the completed phases queue
        if let Some(current_phase) = &self.current_phase {
            self.completed_phases.push_back(current_phase.clone());
            self.current_phase = None;
        }

        // pop the next level (if it exists) into the the current level
        self.current_phase = self.queued_phases.pop_front();

        info!("Phase cycled");

        // return true if no phase was available to cycle to the current phase
        self.current_phase.is_none()
    }

    pub fn init_phase(
        &mut self,
        change_bg_music_event_writer: &mut EventWriter<ChangeBackgroundMusicEvent>,
    ) {
        if let Some(current_phase) = &self.current_phase {
            if let Some(bg_music_transition) = &current_phase.bg_music_transition {
                // change music
                change_bg_music_event_writer
                    .send(ChangeBackgroundMusicEvent::from(bg_music_transition));
            }
        }
    }

    // returns true if level has been completed
    pub fn tick(
        &mut self,
        time: &Time,
        spawn_formation_event_writer: &mut EventWriter<SpawnFormationEvent>,
        formations_res: &FormationPoolsResource,
        spawn_mob_event_writer: &mut EventWriter<SpawnMobEvent>,
        bosses_destroyed_event_reader: &mut EventReader<BossesDestroyedEvent>,
        change_bg_music_event_writer: &mut EventWriter<ChangeBackgroundMusicEvent>,
    ) -> bool {
        self.level_time.tick(time.delta());

        if let Some(mut modified_current_phase) = self.current_phase.clone() {
            let phase_completed = match &mut modified_current_phase.phase_type {
                LevelPhaseType::FormationSpawn {
                    phase_timer,
                    spawn_timer,
                    formation_pool,
                } => {
                    Self::tick_spawn_timer(
                        spawn_timer,
                        time,
                        spawn_formation_event_writer,
                        formations_res,
                        formation_pool.to_string(),
                    );

                    Self::tick_phase_timer(phase_timer, time)
                }
                LevelPhaseType::Break { phase_timer } => Self::tick_phase_timer(phase_timer, time),
                LevelPhaseType::Boss {
                    mob_type,
                    position,
                    spawn_timer,
                } => {
                    if spawn_timer.finished() {
                        // check if no entities with a BossComponent tag exist
                        !bosses_destroyed_event_reader.is_empty()
                    } else {
                        spawn_timer.tick(time.delta());
                        if spawn_timer.just_finished() {
                            spawn_mob_event_writer.send(SpawnMobEvent {
                                mob_type: mob_type.clone(),
                                position: *position,
                                rotation: Quat::default(),
                                boss: true,
                            });
                        }
                        false
                    }
                }
            };

            self.current_phase = Some(modified_current_phase);

            // this will short circuit and not call cycle_phase if !phase_completed
            if phase_completed && !self.cycle_phase() {
                self.init_phase(change_bg_music_event_writer);
            }

            false
        } else {
            true
        }
    }

    fn tick_phase_timer(phase_timer: &mut Timer, time: &Time) -> bool {
        phase_timer.tick(time.delta());

        phase_timer.just_finished()
    }

    pub fn tick_spawn_timer(
        spawn_timer: &mut Timer,
        time: &Time,
        spawn_formation_event_writer: &mut EventWriter<SpawnFormationEvent>,
        formations_res: &FormationPoolsResource,
        formation_key: String,
    ) {
        let formation = formations_res.get_random_formation(formation_key);
        spawn_timer.tick(time.delta());

        if spawn_timer.just_finished() {
            spawn_formation_event_writer.send(SpawnFormationEvent {
                formation: formation.clone(),
            });
            spawn_timer.set_duration(Duration::from_secs_f32(formation.period));
            spawn_timer.reset();
            info!("Spawn timer duration reset to: {}", formation.period);
        }
    }
}
