use std::marker::PhantomData;

use bevy::{ecs::query::QuerySingleError, prelude::*, ui::UiSystem, window::PrimaryWindow};

/// Defines to what the UI entity should be anchored to, can be either another entity (which must have a ['GlobalTransform'])
/// or an in world location
#[derive(Reflect, Debug, PartialEq, Clone)]
pub enum AnchorTarget {
    /// Anchor towards an entity with a [`Transform`] in the world
    Entity(Entity),
    /// Anchor towards a fixed point in the world
    Translation(Vec3),
}

/// Defines where the point that is anchored is located on the height of UI node that is anchored
#[derive(Default, Reflect, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VerticalAnchor {
    Top,
    #[default]
    Mid,
    Bottom,
}
/// Defines where the point that is anchored is located on the width of UI node that is anchored
#[derive(Default, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalAnchor {
    Left,
    #[default]
    Mid,
    Right,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
#[reflect(Component)]
/// Component that will continuosly update the UI location on screen, to match an in world location either chosen as a fixed
/// position, or chosen as another entities ['GlobalTransformation']
pub struct AnchorUiNode {
    /// The Ui will adapts its place towards the chosen location
    pub target: AnchorTarget,
    /// Defines where the horizontal part of the UI tries to synchronize towards the chosen target
    pub anchorwidth: HorizontalAnchor,
    /// Defines where the vertical part of the UI tries to synchronize towards the chosen target
    pub anchorheight: VerticalAnchor,
    /// Offset will be calculated for the 'AnchorTarget'
    /// and the chosen anchoring of the UI element, and can be used to put UI elements away from what they are targeted to
    pub offset: Option<Vec3>,
}

impl AnchorUiNode {
    /// Will anchor the midpoint of this UI element towards the chosen entity
    pub fn to_entity(entity: Entity) -> Self {
        Self {
            target: AnchorTarget::Entity(entity),
            anchorwidth: HorizontalAnchor::Mid,
            anchorheight: VerticalAnchor::Mid,
            offset: None,
        }
    }
    /// Will anchor the midpoint of this UI element towards the chosen spot in the world
    pub fn to_translation(translation: Vec3) -> Self {
        Self {
            target: AnchorTarget::Translation(translation),
            anchorwidth: HorizontalAnchor::Mid,
            anchorheight: VerticalAnchor::Mid,
            offset: None,
        }
    }
    pub fn with_offset(mut self, offset: Vec3) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn with_horizontal_anchoring(mut self, horizontal: HorizontalAnchor) -> Self {
        self.anchorwidth = horizontal;
        self
    }
    pub fn with_vertical_anchoring(mut self, vertical: VerticalAnchor) -> Self {
        self.anchorheight = vertical;
        self
    }
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
            system_move_ui_nodes::<SingleCameraMarker>
                .before(TransformSystem::TransformPropagate)
                .before(UiSystem::Layout),
        );

        app.register_type::<AnchorUiNode>();
    }
}

fn system_move_ui_nodes<C: Component>(
    cameras: Query<(Entity, &Camera), With<C>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut uinodes: Query<(Entity, &mut Node, &ComputedNode, &AnchorUiNode)>,
    transformhelper: TransformHelper,
) {
    let window = match window.get_single() {
        Ok(window) => window,
        Err(QuerySingleError::NoEntities(_)) => return,
        Err(err @ QuerySingleError::MultipleEntities(_)) => {
            bevy::log::error!("more than one primary window: {err}");
            return;
        }
    };
    let (camera_entity, main_camera) = match cameras.get_single() {
        Ok(camera) => camera,
        Err(QuerySingleError::NoEntities(_)) => return,
        Err(err @ QuerySingleError::MultipleEntities(_)) => {
            bevy::log::error!("more than one camera with the specified marker component: {err}");
            return;
        }
    };
    let Ok(main_camera_transform) = transformhelper.compute_global_transform(camera_entity) else {
        warn!("Failed computing global transform for Camera Entity");
        return;
    };

    for (uientity, mut node, computed_node, uinode) in uinodes.iter_mut() {
        // what location should we sync to
        let world_location = match uinode.target {
            AnchorTarget::Entity(entity) => {
                if let Ok(gt) = transformhelper.compute_global_transform(entity) {
                    gt.translation()
                } else {
                    warn!("AnchorTarget({entity}) failed to compute global transform, uinode: {uientity} will not be updated");
                    continue;
                }
            }
            AnchorTarget::Translation(world_location) => world_location,
        };

        let world_location = if let Some(offset) = uinode.offset {
            world_location + offset
        } else {
            world_location
        };

        let Ok(position) =
            main_camera.world_to_viewport_with_depth(&main_camera_transform, world_location)
        else {
            // Object is offscreen and should not be drawn
            bevy::log::debug!("world location is offscreen, and thus we dont change the position");
            continue;
        };

        if node.as_ref().position_type != PositionType::Absolute {
            node.position_type = PositionType::Absolute;
        }

        let nodewidth = if let Val::Px(width) = node.width {
            width
        } else {
            computed_node.size().x * computed_node.inverse_scale_factor()
        };
        let leftpos = match uinode.anchorwidth {
            HorizontalAnchor::Left => Val::Px(position.x),
            HorizontalAnchor::Mid => Val::Px(position.x - nodewidth / 2.0),
            HorizontalAnchor::Right => Val::Px(position.x - nodewidth),
        };

        if check_if_not_close(node.as_ref().left, leftpos) {
            node.left = leftpos;
        }

        let window_height = window.height();

        let nodeheight = if let Val::Px(height) = node.height {
            height
        } else {
            computed_node.size().y * computed_node.inverse_scale_factor()
        };

        let newheight = match uinode.anchorheight {
            VerticalAnchor::Top => Val::Px(window_height - position.y - nodeheight),
            VerticalAnchor::Mid => Val::Px(window_height - position.y - nodeheight / 2.0),
            VerticalAnchor::Bottom => Val::Px(window_height - position.y),
        };

        if check_if_not_close(node.as_ref().bottom, newheight) {
            node.bottom = newheight;
        }
    }
}

// only move if the change position is more than one pixel from each other, stops vibrations
fn check_if_not_close(a: Val, b: Val) -> bool {
    return true;
    if a == b {
        return false;
    }

    match (a, b) {
        (Val::Px(a), Val::Px(b)) => (a - b).abs() > 1.0, // If they are more than a pixel from eachother
        _ => true,
    }
}
