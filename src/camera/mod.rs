use crate::game;
use bevy::app::{App, Plugin, Startup};
use bevy::core_pipeline::bloom::BloomPrefilterSettings;
use bevy::core_pipeline::core_2d::{Camera2d, Camera2dBundle};
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping};
use bevy::ecs::system::{Commands, Res};
use bevy::math::Vec3;
use bevy::render::camera::{Camera, PerspectiveProjection, Projection};
use bevy::transform::components::Transform;
use bevy::utils::default;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cameras_system);
    }
}

pub fn setup_cameras_system(
    mut commands: Commands,
    game_parameters: Res<game::GameParametersResource>,
) {
    // setup cameras
    // 2d camera for sprites

    let camera_2d = Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, game_parameters.camera_z),
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
        },
        camera: Camera {
            order: 1,
            hdr: true,
            ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface,
        ..default()
    };

    commands.spawn((
        camera_2d,
        BloomSettings {
            prefilter_settings: BloomPrefilterSettings {
                threshold: 1.0,
                threshold_softness: 0.2,
            },
            ..BloomSettings::OLD_SCHOOL
        },
    ));

    // 3d camera for background objects
    let camera_3d = Camera3dBundle {
        camera: Camera {
            order: 0,
            hdr: true,
            ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface,
        transform: Transform::from_xyz(0.0, 0.0, game_parameters.camera_z)
            .looking_at(Vec3::ZERO, Vec3::Y),
        projection: Projection::Perspective(PerspectiveProjection {
            far: 10000.0,
            ..Default::default()
        }),
        ..Default::default()
    };
    commands.spawn((camera_3d, BloomSettings::default()));
}
