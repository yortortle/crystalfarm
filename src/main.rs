mod ui;
mod gameplay;

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use ui::UIPlugin;
use gameplay::GameplayPlugin;


// MAIN RUST / BEVY TECH
// fn my_system(time: Option<Res<Time>>) {
//     if let Some(time) = time {
//         ...
//     }
// }
// Query<&Machine, Added<Machine>>
// Query<&Hovered, Changed<Hovered>>

// asking the query for entitly explicitly is a method.
// Query<(Entity, &Hovered)>
// use marker components (empty struct like cube for example) for everything. This is the core identifier.

// Situation	Best Tool
// “This entity is hovered”	Component
// “This entity was hovered this frame”	Event
// “Spawn something now”	Event → Command
// “Global time”	Resource
// “Only react when value changes”	Changed<T>
// “One-time setup”	Startup system

#[derive(Resource)]
struct StartupDelay(Timer);

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        // .add_plugins(crate::gameplay::field::FieldTestPlugin)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                visible: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(GameplayPlugin)
        .add_systems(Startup, spawn_cubes.after(crate::gameplay::shaders::shader_library::ShaderInitSet::Init))
        .add_systems(Startup, init_delay)
        .add_systems(Startup, setup)
        .add_systems(Update, show_window_after_delay)
        .run();
}

fn init_delay(mut commands: Commands) {
    commands.insert_resource(StartupDelay(
        Timer::from_seconds(0.01, TimerMode::Once),
    ));
}

fn show_window_after_delay(
    time: Res<Time>,
    mut delay: ResMut<StartupDelay>,
    mut windows: Query<&mut Window>,
) {
    delay.0.tick(time.delta());

    if delay.0.is_finished() {
        if let Ok(mut window) = windows.single_mut() {
            if !window.visible {
                window.visible = true;
            }
        }
    }
}

fn spawn_cubes(
    mut spawn_cube: MessageWriter<crate::gameplay::scene::shapes::SpawnCube>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    library: Res<gameplay::shaders::shader_library::ShaderLibrary>,
) {
    // let grid_size = 10;        // 10x10 = 100 cubes
    // let spacing = 1.5;         // > cube size to avoid overlap
    // let start_y = 0.75;
    //
    // for x in 0..grid_size {
    //     for z in 0..grid_size {
    //         let mat = instanced_material_from_template(
    //             &mut materials,
    //             &library.crystal,
    //         );
    //
    //         let world_x = x as f32 * spacing;
    //         let world_z = z as f32 * spacing;
    //
    //         spawn_cube.write(crate::gameplay::scene::shapes::SpawnCube {
    //             transform: Transform::from_xyz(world_x, start_y, world_z),
    //             tooltip: "Stress Cube",
    //             mat,
    //         });
    //     }
    // }

    let unique_handle1 = instanced_material_from_template(&mut materials, &library.crystal);
    spawn_cube.write(crate::gameplay::scene::shapes::SpawnCube {
        transform: Transform::from_xyz(0.0, 0.5002, 0.0),
        tooltip: "Cube 1",
        mat: unique_handle1,
    });

    let unique_handle2 = instanced_material_from_template(&mut materials, &library.crystal);
    spawn_cube.write(crate::gameplay::scene::shapes::SpawnCube {
      transform: Transform::from_xyz(1.2, 0.5002, 1.2),
      tooltip: "Cube 2",
      mat: unique_handle2,
    });
}

fn instanced_material_from_template(
    materials: &mut Assets<StandardMaterial>,
    template: &Handle<StandardMaterial>,
) -> Handle<StandardMaterial> {
    let mat = materials
        .get(template)
        .expect("Template material not found in Assets<StandardMaterial>")
        .clone();
    materials.add(mat)
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circle
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(300.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    
    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera + Pan/Orbit controller
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 8.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera::default(),
    ));
    
//     let mesh = meshes.add(crystal_mesh(7, 0.35, 1.2, 0.55));
//     let material = materials.add(StandardMaterial {
//         base_color: Color::srgb(0.35, 0.85, 1.0),
//         perceptual_roughness: 0.08,
//         reflectance: 0.7,
//         ..default()
//     });
// 
//     commands.spawn(MaterialMeshBundle {
//         mesh,
//         material,
//         transform: Transform::from_rotation(Quat::from_rotation_y(0.35)),
//         ..default()
//     });


}

// Faceted crystal: N-gon prism + tip. Flat shaded by duplicating verts per triangle.
// fn crystal_mesh(sides: usize, radius: f32, height: f32, tip: f32) -> Mesh {
//     let mut positions: Vec<[f32; 3]> = Vec::new();
//     let mut normals: Vec<[f32; 3]> = Vec::new();
//     let mut indices: Vec<u32> = Vec::new();
// 
//     let mut ring_bot = Vec::with_capacity(sides);
//     let mut ring_top = Vec::with_capacity(sides);
//     for i in 0..sides {
//         let a = (i as f32) * std::f32::consts::TAU / (sides as f32);
//         let x = a.cos() * radius;
//         let y = a.sin() * radius;
//         ring_bot.push(Vec3::new(x, y, 0.0));
//         ring_top.push(Vec3::new(x, y, height));
//     }
//     let tip_p = Vec3::new(0.0, 0.0, height + tip);
// 
//     let mut add_tri = |a: Vec3, b: Vec3, c: Vec3| {
//         let n = (b - a).cross(c - a).normalize_or_zero();
//         let base = positions.len() as u32;
//         positions.extend([[a.x, a.y, a.z], [b.x, b.y, b.z], [c.x, c.y, c.z]]);
//         normals.extend([[n.x, n.y, n.z], [n.x, n.y, n.z], [n.x, n.y, n.z]]);
//         indices.extend([base, base + 1, base + 2]);
//     };
// 
//     // sides (two tris per quad)
//     for i in 0..sides {
//         let j = (i + 1) % sides;
//         let b0 = ring_bot[i];
//         let b1 = ring_bot[j];
//         let t0 = ring_top[i];
//         let t1 = ring_top[j];
//         add_tri(b0, t0, t1);
//         add_tri(b0, t1, b1);
//     }
// 
//     // tip
//     for i in 0..sides {
//         let j = (i + 1) % sides;
//         add_tri(ring_top[i], tip_p, ring_top[j]);
//     }
// 
//     // bottom fan (winding reversed so it points down)
//     let center = Vec3::ZERO;
//     for i in 0..sides {
//         let j = (i + 1) % sides;
//         add_tri(ring_bot[j], center, ring_bot[i]);
//     }
// 
//     let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
//     mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
//     mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
//     mesh.insert_indices(Indices::U32(indices));
//     mesh
// }