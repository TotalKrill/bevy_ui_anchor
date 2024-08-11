use std::marker::PhantomData;

use bevy::{prelude::*, window::PrimaryWindow};

pub enum AnchorTarget {
    /// Anchor towards an entity with a [`Transform`] in the world
    Entity(Entity),
    /// Anchor towards a fixed point in the world
    Translation(Vec3),
}

// Defines at what height of the node should be anchored
pub enum VerticalAnchor {
    Top,
    Mid,
    Bottom,
}
// Defines at what width the node should be anchored
pub enum HorizontalAnchor {
    Left,
    Mid,
    Right,
}

#[derive(Component)]
pub struct AnchorUiNode {
    pub target: AnchorTarget,
    pub anchorwidth: HorizontalAnchor,
    pub anchorheight: VerticalAnchor,
}

pub struct AnchorUiPlugin<SingleCameraMarker: Component> {
    _component: PhantomData<SingleCameraMarker>,
}

impl<SingleCameraMarker: Component> AnchorUiPlugin<SingleCameraMarker> {
    pub fn new() -> Self {
        Self {
            _component: PhantomData::default(),
        }
    }
}

impl<SingleCameraMarker: Component> Plugin for AnchorUiPlugin<SingleCameraMarker> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            system_move_ui_nodes::<SingleCameraMarker>.after(TransformSystem::TransformPropagate),
        );
    }
}

fn system_move_ui_nodes<C: Component>(
    cameras: Query<(&Camera, &GlobalTransform), With<C>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut uinodes: Query<(Entity, &mut Style, &Node, &AnchorUiNode)>,
    targets: Query<&GlobalTransform>,
) {
    let Ok(window) = window.get_single() else {
        bevy::log::error!("more than one primary window");
        return;
    };
    let Ok((main_camera, main_camera_transform)) = cameras.get_single() else {
        bevy::log::error!("more than one camera with the specified marker component");
        return;
    };

    for (uientity, mut style, node, uinode) in uinodes.iter_mut() {
        // what location should we sync to
        let world_location = match uinode.target {
            AnchorTarget::Entity(e) => {
                if let Ok(gt) = targets.get(e) {
                    gt.translation()
                } else {
                    warn!("AnchorTarget({e}) was not found for uinode: {uientity}");
                    continue;
                }
            }
            AnchorTarget::Translation(world_location) => world_location,
        };

        let Some(position) = main_camera.world_to_viewport(main_camera_transform, world_location)
        else {
            // Object is offscreen and should not be drawn
            bevy::log::debug!("world location is offscreen, and thus we dont change the position");
            continue;
        };

        if style.as_ref().position_type != PositionType::Absolute {
            style.position_type = PositionType::Absolute;
        }

        let nodewidth = node.size().x;
        let leftpos = match uinode.anchorwidth {
            HorizontalAnchor::Left => Val::Px(position.x),
            HorizontalAnchor::Mid => Val::Px(position.x - nodewidth / 2.0),
            HorizontalAnchor::Right => Val::Px(position.x - nodewidth),
        };

        if check_if_not_close(style.as_ref().left, leftpos) {
            style.left = leftpos;
        }

        let window_height = window.height();

        let newheight = match uinode.anchorheight {
            VerticalAnchor::Top => Val::Px(window_height - position.y - node.size().y),
            VerticalAnchor::Mid => Val::Px(window_height - position.y - node.size().y / 2.0),
            VerticalAnchor::Bottom => Val::Px(window_height - position.y),
        };

        if check_if_not_close(style.as_ref().bottom, newheight) {
            style.bottom = newheight;
        }
    }
}

// only move if the change position is more than one pixel from each other, stops vibrations
fn check_if_not_close(a: Val, b: Val) -> bool {
    if a == b {
        return false;
    }

    match (a, b) {
        (Val::Px(a), Val::Px(b)) => (a - b).abs() > 1.0, // If they are more than a pixel from eachother
        _ => true,
    }
}
