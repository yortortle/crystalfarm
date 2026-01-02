use bevy::prelude::*;

#[derive(Resource)]
#[allow(dead_code)]
pub struct ShaderLibrary {
    pub crystal: Handle<StandardMaterial>,
    pub crystal_hover: Handle<StandardMaterial>,
    pub metal: Handle<StandardMaterial>,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ShaderInitSet {
    Init,
}

pub struct ShaderLibraryPlugin;

impl Plugin for ShaderLibraryPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Startup, ShaderInitSet::Init);
        app.add_systems(Startup, init_materials.in_set(ShaderInitSet::Init));
    }
}

fn init_materials(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let crystal = materials.add(StandardMaterial {
        base_color: Color::srgba(0.35, 0.75, 1.0, 0.35),
        perceptual_roughness: 0.05,
        metallic: 0.0,
        reflectance: 0.6,
        emissive: Color::srgb(0.10, 0.35, 0.60).into(),
        alpha_mode: AlphaMode::Blend,
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    let crystal_hover = materials.add(StandardMaterial {
        base_color: Color::srgb_u8(255, 180, 90),
        emissive: Color::srgb(0.4, 0.2, 0.0).into(),
        ..default()
    });

    let metal = materials.add(StandardMaterial {
        base_color: Color::srgb_u8(180, 180, 190),
        metallic: 0.9,
        perceptual_roughness: 0.2,
        ..default()
    });

    commands.insert_resource(ShaderLibrary {
        crystal,
        crystal_hover,
        metal,
    });
}
