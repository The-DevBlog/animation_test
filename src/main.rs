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

// fn setup_scene_once_loaded(
//     mut commands: Commands,
//     animations: Res<Animations>,
//     graphs: Res<Assets<AnimationGraph>>,
//     mut clips: ResMut<Assets<AnimationClip>>,
//     mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
// ) {
//     fn get_clip<'a>(
//         node: AnimationNodeIndex,
//         graph: &AnimationGraph,
//         clips: &'a mut Assets<AnimationClip>,
//     ) -> &'a mut AnimationClip {
//         let node = graph.get(node).unwrap();
//         let clip = match &node.node_type {
//             AnimationNodeType::Clip(handle) => clips.get_mut(handle),
//             _ => unreachable!(),
//         };
//         clip.unwrap()
//     }

//     for (entity, mut player) in &mut players {
//         let graph = graphs.get(&animations.graph).unwrap();

//         // Send `OnStep` events once the fox feet hits the ground in the running animation.
//         let animation = get_clip(animations.animations[0], graph, &mut clips);
//         // You can determine the time an event should trigger if you know witch frame it occurs and
//         // the frame rate of the animation. Let's say we want to trigger an event at frame 15,
//         // and the animation has a frame rate of 24 fps, then time = 15 / 24 = 0.625.
//         animation.add_event_to_target(feet.front_left, 0.625, OnStep);
//         animation.add_event_to_target(feet.front_right, 0.5, OnStep);
//         animation.add_event_to_target(feet.back_left, 0.0, OnStep);
//         animation.add_event_to_target(feet.back_right, 0.125, OnStep);

//         let mut transitions = AnimationTransitions::new();

//         // Make sure to start the animation via the `AnimationTransitions`
//         // component. The `AnimationTransitions` component wants to manage all
//         // the animations and will get confused if the animations are started
//         // directly via the `AnimationPlayer`.
//         transitions
//             .play(&mut player, animations.animations[0], Duration::ZERO)
//             .repeat();

//         commands
//             .entity(entity)
//             .insert(AnimationGraphHandle(animations.graph.clone()))
//             .insert(transitions);
//     }
// }
