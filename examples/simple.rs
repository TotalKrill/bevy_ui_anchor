//! Demonstrates how to work with Cubic curves.
use bevy::{
    color::palettes::css::{ORANGE, SILVER, WHITE},
    prelude::*,
};

use bevy_ui_anchor::{AnchorPoint, AnchorUiConfig, AnchorUiPlugin, AnchoredUiNodes};

#[derive(Component)]
/// We need a marker for the camera, so the plugin knows which camera to perform position
/// calculations towards
pub struct UiCameraMarker;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // We need to define, which camera the anchorplugin will be anchored to
        .add_plugins(AnchorUiPlugin::<UiCameraMarker>::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // The camera
    commands.spawn((
        // mark it
        UiCameraMarker,
        IsDefaultUiCamera,
        Camera3d::default(),
        Transform::from_xyz(0., 6., 12.).looking_at(Vec3::new(0., 3., 0.), Vec3::Y),
        DirectionalLight::default(),
    ));

    // Spawning a cube with anchored UI, using bevy relations
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.3, 0.3, 0.3))),
        MeshMaterial3d(materials.add(Color::from(ORANGE))),
        Transform::from_translation([0.0, 0.5, 0.0].into()),
        // Define the anchor relationship
        AnchoredUiNodes::spawn_one((
            AnchorUiConfig {
                anchorpoint: AnchorPoint::bottomleft(),
                offset: Some(Vec3::new(0.0, 0.5, 0.0)),
                ..Default::default()
            },
            Node {
                border: UiRect::all(Val::Px(2.)),
                border_radius: BorderRadius::all(Val::Px(2.)),
                ..Default::default()
            },
            BorderColor::all(WHITE),
            Outline::default(),
            Children::spawn_one(
                // text
                Text("Text Anchored in bottom left".into()),
            ),
        )),
    ));

    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50., 50.))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));
}
