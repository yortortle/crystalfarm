use bevy::prelude::*;
// use bevy_hanabi::prelude::*;

pub struct CubePlugin;

use bevy::picking::hover::Hovered;

#[derive(Component)]
struct Cube;

#[derive(Message)]
pub struct SpawnCube {
    pub transform: Transform,
    pub tooltip: &'static str,
    pub mat: Handle<StandardMaterial>
}

#[derive(Component)]
struct HoverTint {
    normal: Color,
    hover: Color,
}

#[derive(Component)]
struct HoverLerp {
    t: f32,        // 0..1
    speed: f32,    // units per second
}

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnCube>()
           .add_systems(Update, handle_spawn_cube)
           .add_systems(Update, lerp_hover_color);
    }
}

fn handle_spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ev: MessageReader<SpawnCube>,
) {
    let normal = Color::srgb_u8(124, 144, 255);
    let hover = Color::srgb_u8(255, 180, 90);
        
    for req in ev.read() {
//         let mat = materials.add(StandardMaterial {
//             base_color: normal,
//             ..default()
//         });

        // let mat = materials.add(library.crystal);
        // let mat = library.crystal.clone();

        let mat = req.mat.clone();
        
        commands.spawn((
            Cube,
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(mat),
            req.transform,
            Hovered(false),
            HoverTint { normal, hover }, 
            HoverLerp { t: 0.0, speed: 4.0 },
            crate::ui::tooltip::HoverTooltip(req.tooltip),
        ));
    }
}

// pub fn spawn_cube(
//     commands: &mut Commands,
//     meshes: &mut Assets<Mesh>,
//     materials: &mut Assets<StandardMaterial>,
//     transform: Transform,
// ) -> Entity {
//     commands
//         .spawn((
//             Cube,
//             Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
//             MeshMaterial3d(materials.add(Color::WHITE)),
//             transform,
//             // crate::ui::tooltip::HoverTooltip("Cube"),
//         ))
//         .id()
// }

fn move_towards(current: f32, target: f32, max_delta: f32) -> f32 {
    if (target - current).abs() <= max_delta {
        target
    } else {
        current + (target - current).signum() * max_delta
    }
}

fn lerp_hover_color(
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut q: Query<(&Hovered, &MeshMaterial3d<StandardMaterial>, &HoverTint, &mut HoverLerp)>,
) {
    for (hovered, mat_handle, tint, mut lerp) in &mut q {
        let target = if hovered.get() { 1.0 } else { 0.0 };
        let dt = time.delta_secs();

        // Move t toward target at lerp.speed per second
        lerp.t = move_towards(lerp.t, target, lerp.speed * dt);

        if let Some(mat) = materials.get_mut(mat_handle) {
            let a = LinearRgba::from(tint.normal);
            let b = LinearRgba::from(tint.hover);
            mat.base_color = Color::from(a.mix(&b, lerp.t));
        }
    }
}

// pub fn add_aether_effect(mut effects: ResMut<Assets<EffectAsset>>) -> Handle<EffectAsset> {
//     // Color: bright “aether” -> transparent
//     let mut color = Gradient::new();
//     color.add_key(0.0, Vec4::new(0.45, 0.85, 1.00, 0.9));
//     color.add_key(0.7, Vec4::new(0.20, 0.60, 1.00, 0.35));
//     color.add_key(1.0, Vec4::new(0.00, 0.00, 0.00, 0.0));
//
//     // Size: small -> smaller
//     let mut size = Gradient::new();
//     size.add_key(0.0, Vec2::splat(0.06));
//     size.add_key(1.0, Vec2::splat(0.015));
//
//     let mut module = Module::default();
//
//     // Spawn on a spherical shell around origin (we’ll attach this to the cube)
//     let init_pos = SetPositionSphereModifier {
//         center: module.lit(Vec3::ZERO),
//         radius: module.lit(0.9),                  // shell radius around cube
//         dimension: ShapeDimension::Surface,       // Surface = ring/shell feel
//     };
//
//     // Give a little initial tangential velocity to start the swirl
//     let init_vel = SetVelocityTangentModifier {
//         origin: module.lit(Vec3::ZERO),
//         axis: module.lit(Vec3::Y),                // swirl around Y
//         speed: module.lit(1.5),
//     };
//
//     // Lifetime is basically always needed so particles die & recycle
//     let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, module.lit(1.2));
//
//     // Pull inward (negative radial accel = “coalescing”)
//     let pull_in = RadialAccelModifier::new(
//         module.lit(Vec3::ZERO),
//         module.lit(-6.0),
//     );
//
//     // Keep them spiraling while they’re being pulled
//     let swirl = TangentAccelModifier::new(
//         module.lit(Vec3::Y),
//         module.lit(8.0),
//     );
//
//     // Damp velocity so it doesn’t look like angry hornets
//     let drag = LinearDragModifier::new(module.lit(2.0));
//
//     // Build the effect
//     let effect = EffectAsset::new(
//         32768,                          // capacity
//         SpawnerSettings::rate(220.0.into()),
//         module,
//     )
//         .with_name("AetherCoalesce")
//         .with_simulation_space(SimulationSpace::Local) // crucial for “stick to cube”
//         .init(init_pos)
//         .init(init_vel)
//         .init(init_lifetime)
//         .update(pull_in)
//         .update(swirl)
//         .update(drag)
//         .render(ColorOverLifetimeModifier { gradient: color })
//         .render(SizeOverLifetimeModifier { gradient: size })
//         .render(RoundModifier {}); // no texture needed
//
//     effects.add(effect)
// }