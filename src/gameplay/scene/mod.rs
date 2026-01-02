use bevy::prelude::*;

pub mod shapes;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            shapes::ShapesPlugin,
        ));
    }
}