use bevy::prelude::*;

pub mod tooltip;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            tooltip::TooltipPlugin,
        ));
    }
}