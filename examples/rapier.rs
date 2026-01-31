use bevy::{
    color::{self, palettes::css::BLACK},
    prelude::*,
};
use bevy_rapier3d::prelude::*;
use bevy_ui_anchor::{AnchorPoint, AnchorUiConfig, AnchorUiPlugin, AnchoredUiNodes};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(bevy_editor_cam::DefaultEditorCamPlugins)
        .add_plugins(AnchorUiPlugin::<CameraMarker>::new())
        .add_systems(Startup, (setup_graphics, setup_physics))
        .run();
}

#[derive(Component)]
pub struct CameraMarker;

pub fn setup_graphics(mut commands: Commands) {
    let transform =
        Transform::from_xyz(-30.0, 30.0, 100.0).looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y);
    commands.spawn((
        CameraMarker,
        Camera3d::default(),
        bevy_editor_cam::prelude::EditorCam {
            last_anchor_depth: -transform.translation.length() as f64,
            ..default()
        },
        transform.clone(),
    ));
    let transform = Transform::from_xyz(-30.0, 30.0, 10.0);
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 100_000_000.,
            range: 500.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        transform,
    ));
}

pub fn setup_physics(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    /*
     * Ground
     */
    let ground_size = 200.1;
    let ground_height = 0.1;

    commands.spawn((
        Transform::from_xyz(0.0, -ground_height, 0.0),
        Collider::cuboid(ground_size, ground_height, ground_size),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(ground_size)))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(color::palettes::basic::SILVER))),
    ));

    /*
     * Create the cubes
     */
    let num = 3;
    let rad = 3.0;

    let shift = rad * 2.0 + rad;
    let centerx = shift * (num / 2) as f32;
    let centery = shift / 2.0;
    let centerz = shift * (num / 2) as f32;

    let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;
    let mut color = 0;
    let colors = [
        Hsla::hsl(220.0, 1.0, 0.3),
        Hsla::hsl(180.0, 1.0, 0.3),
        Hsla::hsl(260.0, 1.0, 0.7),
    ];

    let materials: Vec<Handle<StandardMaterial>> = colors
        .iter()
        .map(|c| materials.add(StandardMaterial::from_color(*c)))
        .collect();

    // let material = materials.add(StandardMaterial::from_color(color::palettes::basic::BLUE));
    let mesh = meshes.add(Sphere::new(rad));

    for j in 0usize..20 {
        for i in 0..num {
            for k in 0usize..num {
                let x = i as f32 * shift - centerx + offset;
                let y = j as f32 * shift + centery + 3.0;
                let z = k as f32 * shift - centerz + offset;
                color += 1;

                let mut ec = commands.spawn(());
                // we want the target inside the spawned text fields
                let target = ec.id();

                ec.insert((
                    Transform::from_xyz(x, y, z),
                    RigidBody::Dynamic,
                    Collider::ball(rad),
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(materials[color % colors.len()].clone()),
                    // ColliderDebugColor(colors[color % 3]),
                    AnchoredUiNodes::spawn_one((
                        Node {
                            border: UiRect::all(Val::Px(2.)),
                            border_radius: BorderRadius::all(px(2)),
                            ..Default::default()
                        },
                        BorderColor::all(BLACK),
                        Outline::default(),
                        AnchorUiConfig {
                            anchorpoint: AnchorPoint::middle(),
                            offset: None,
                            ..Default::default()
                        },
                        Children::spawn_one((
                            Text(format!("{target}")),
                            TextFont {
                                font_size: 10.,
                                ..Default::default()
                            },
                            TextColor(BLACK.into()),
                        )),
                    )),
                ));
            }
        }

        offset -= 0.05 * rad * (num as f32 - 1.0);
    }
}
