use bevy::prelude::*;

pub struct GameplayPlugin;

pub mod scene;
pub mod shaders;
pub(crate) mod field;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MeshPickingPlugin,
            scene::ScenePlugin,
            shaders::ShaderLibraryPlugin
        ));
    }
}