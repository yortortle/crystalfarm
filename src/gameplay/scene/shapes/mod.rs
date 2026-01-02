// mod cube;
// pub use cube::*;

use bevy::prelude::*;

mod cube;

// Re-export the public API you want to expose from this folder:
pub use cube::{CubePlugin, SpawnCube};

pub struct ShapesPlugin;

impl Plugin for ShapesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CubePlugin,
        ));
    }
}