//! Demonstrates how to work with Cubic curves.
use bevy::{
    color::palettes::css::{ORANGE, SILVER, WHITE},
    dev_tools::ui_debug_overlay::{DebugUiPlugin, UiDebugOptions},
    math::vec3,
    prelude::*,
};
use bevy_ui_anchor::{AnchorUiNode, AnchorUiPlugin};

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
    // These points will define the curve
    // You can learn more about bezier curves here
    // https://en.wikipedia.org/wiki/B%C3%A9zier_curve
    let points = [[
        vec3(-6., 2., 0.),
        vec3(12., 8., 0.),
        vec3(-12., 8., 0.),
        vec3(6., 2., 0.),
    ]];

    // Make a CubicCurve
    let bezier = CubicBezier::new(points).to_curve();
    // Spawning a cube to experiment on
    let target = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.3, 0.3, 0.3)),
                material: materials.add(Color::from(ORANGE)),
                transform: Transform::from_translation(points[0][0]),
                ..default()
            },
            Curve(bezier),
        ))
        .id();

    commands
        .spawn((
            NodeBundle {
                border_color: BorderColor(WHITE.into()),
                border_radius: BorderRadius::all(Val::Px(2.)),
                style: Style {
                    border: UiRect::all(Val::Px(2.)),
                    ..Default::default()
                },
                ..Default::default()
            },
            Outline::default(),
            AnchorUiNode {
                target: bevy_ui_anchor::AnchorTarget::Entity(target),
                anchorwidth: bevy_ui_anchor::HorizontalAnchor::Right,
                anchorheight: bevy_ui_anchor::VerticalAnchor::Bottom,
            },
        ))
        .with_children(|p| {
            p.spawn(TextBundle {
                text: Text::from_section("Text Anchored in bottom right", Default::default()),
                ..default()
            });
        });

    // Some light to see something
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(8., 16., 8.),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50., 50.)),
        material: materials.add(Color::from(SILVER)),
        ..default()
    });

    // The camera
    commands.spawn((
        CameraMarker,
        IsDefaultUiCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(0., 6., 12.).looking_at(Vec3::new(0., 3., 0.), Vec3::Y),
            ..default()
        },
    ));
}

fn animate_cube(time: Res<Time>, mut query: Query<(&mut Transform, &Curve)>, mut gizmos: Gizmos) {
    let t = (time.elapsed_seconds().sin() + 1.) / 2.;

    for (mut transform, cubic_curve) in &mut query {
        // Draw the curve
        gizmos.linestrip(cubic_curve.0.iter_positions(50), WHITE);
        // position takes a point from the curve where 0 is the initial point
        // and 1 is the last point
        transform.translation = cubic_curve.0.position(t);
    }
}
