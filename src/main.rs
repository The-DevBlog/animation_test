use bevy::{math::bounding::Aabb2d, prelude::*};
use bevy_rts_camera::{RtsCamera, RtsCameraControls, RtsCameraPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RtsCameraPlugin))
        .add_systems(Startup, (setup, spawn_cube))
        .run();
}

fn setup(mut cmds: Commands) {
    // camera
    cmds.spawn((
        Camera3d::default(),
        RtsCamera {
            bounds: Aabb2d::new(Vec2::ZERO, Vec2::new(1000.0, 1000.0)),
            min_angle: 60.0f32.to_radians(),
            height_max: 50.0,
            ..default()
        },
        RtsCameraControls {
            edge_pan_width: 0.01,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_up: KeyCode::KeyW,
            key_down: KeyCode::KeyS,
            pan_speed: 165.0,
            zoom_sensitivity: 0.2,
            ..default()
        },
    ));

    // light
    cmds.spawn((
        DirectionalLight {
            illuminance: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::YXZ,
            150.0f32.to_radians(),
            -40.0f32.to_radians(),
            0.0,
        )),
        Name::new("Light"),
    ));
}

fn spawn_cube(mut cmds: Commands, assets: Res<AssetServer>) {
    let handle = assets.load("cube.gltf#Scene0");
    cmds.spawn(SceneRoot(handle));
}
