use std::time::Duration;

use bevy::{math::bounding::Aabb2d, prelude::*};
use bevy_rts_camera::{RtsCamera, RtsCameraControls, RtsCameraPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RtsCameraPlugin))
        .add_systems(Startup, (setup, spawn_cube))
        .add_systems(Update, setup_scene_once_loaded)
        .run();
}

#[derive(Resource)]
pub struct Animations {
    pub animations: Vec<AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
}

fn setup(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // animations
    // Build the animation graph
    let mut graph = AnimationGraph::new();
    let animations = graph
        .add_clips(
            // [GltfAssetLabel::Animation(0).from_asset("cube.gltf")]
            [GltfAssetLabel::Animation(0).from_asset("vehicle_depot.gltf")]
                .into_iter()
                .map(|path| asset_server.load(path)),
            1.0,
            graph.root,
        )
        .collect();

    // Insert a resource with the current scene information
    let graph = graphs.add(graph);
    cmds.insert_resource(Animations {
        animations,
        graph: graph.clone(),
    });

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
    // let handle = assets.load("cube.gltf#Scene0");
    let handle = assets.load("vehicle_depot.gltf#Scene0");
    cmds.spawn(SceneRoot(handle));
}

fn setup_scene_once_loaded(
    mut cmds: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in players.iter_mut() {
        let mut transitions = AnimationTransitions::new();
        let animation = animations.animations[0];

        transitions
            .play(&mut player, animation, Duration::ZERO)
            .repeat();

        cmds.entity(entity)
            .insert(AnimationGraphHandle(animations.graph.clone()))
            .insert(transitions);
    }
}
