# bevy_ui_anchor

[![crates.io](https://img.shields.io/crates/v/bevy_ui_anchor)](https://crates.io/crates/bevy_ui_anchor)
[![docs.rs](https://docs.rs/bevy_ui_anchor/badge.svg)](https://docs.rs/bevy_ui_anchor)
[![License](https://img.shields.io/crates/l/bevy_ui_anchor)](https://opensource.org/licenses/MIT)

A Rust crate for anchoring UI elements to specific points or entities in the world using the Bevy game engine.

![](follow.gif)

## Features

Provides an AnchorUiNode component that:

- Anchor UI nodes to world positions or entities.
- Supports horizontal and vertical anchoring.
- Compatible with Bevy's ECS architecture.

| Bevy version | Crate version |
| ------------ | ------------------------ |
| 0.15         | 0.3 - 0.5                |
| 0.14         | 0.1 - 0.2                |

## Example

``` rust
//! Demonstrates how to work with Cubic curves.
use bevy::{
    color::palettes::css::{ORANGE, SILVER, WHITE},
    dev_tools::ui_debug_overlay::{DebugUiPlugin, UiDebugOptions},
    math::vec3,
    prelude::*,
};

use bevy_ui_anchor::{
    AnchorTarget, AnchorUiNode, AnchorUiPlugin, HorizontalAnchor, VerticalAnchor,
};

#[derive(Component)]
struct Curve(CubicCurve<Vec3>);

#[derive(Component)]
pub struct CameraMarker;

fn main() {
    let mut uidebug = UiDebugOptions::default();
    // uidebug.toggle();
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(uidebug)
        .add_plugins(DebugUiPlugin)
        .add_plugins(AnchorUiPlugin::<CameraMarker>::new())
        .add_systems(Startup, setup)
        .add_systems(Update, animate_cube)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Define your control points
    // These points will define the curve the cube will follow
    // You can learn more about bezier curves here
    // https://en.wikipedia.org/wiki/B%C3%A9zier_curve
    let points = [[
        vec3(-6., 2., 0.),
        vec3(12., 8., 0.),
        vec3(-12., 8., 0.),
        vec3(6., 2., 0.),
    ]];

    // Make a CubicCurve
    let bezier = CubicBezier::new(points).to_curve().unwrap();
    // Spawning a cube that the UI node will be anchored to
    let target = commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.3, 0.3, 0.3))),
            MeshMaterial3d(materials.add(Color::from(ORANGE))),
            Transform::from_translation(points[0][0]),
            Curve(bezier),
        ))
        .id();

    commands
        .spawn((
            Node {
                border: UiRect::all(Val::Px(2.)),
                ..Default::default()
            },
            BorderColor(WHITE.into()),
            BorderRadius::all(Val::Px(2.)),
            Outline::default(),
            // Anchor this UI node to the cube entity
            AnchorUiNode {
                target: AnchorTarget::Entity(target),
                offset: None,
                anchorwidth: HorizontalAnchor::Right,
                anchorheight: VerticalAnchor::Bottom,
            },
        ))
        .with_children(|p| {
            p.spawn(Text("Text Anchored in bottom right".into()));
        });

    // Some light to see something
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            ..default()
        },
        Transform::from_xyz(8., 16., 8.),
    ));

    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50., 50.))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));

    // The camera
    commands.spawn((
        CameraMarker,
        IsDefaultUiCamera,
        Camera3d::default(),
        Transform::from_xyz(0., 6., 12.).looking_at(Vec3::new(0., 3., 0.), Vec3::Y),
    ));
}

fn animate_cube(time: Res<Time>, mut query: Query<(&mut Transform, &Curve)>, mut gizmos: Gizmos) {
    let t = (time.elapsed_secs().sin() + 1.) / 2.;

    for (mut transform, cubic_curve) in &mut query {
        // Draw the curve
        gizmos.linestrip(cubic_curve.0.iter_positions(50), WHITE);
        // position takes a point from the curve where 0 is the initial point
        // and 1 is the last point
        transform.translation = cubic_curve.0.position(t);
    }
}
```
