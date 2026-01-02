use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

pub struct FieldTestPlugin;


// -----------------------------
// Tunables
// -----------------------------

const W: i32 = 24;
const H: i32 = 24;
const CELL_SPACING: f32 = 1.0;

const DIFFUSION: f32 = 6.0;
const DECAY: f32 = 0.35;
const MAX_AETHER: f32 = 10.0;

// -----------------------------
// Resources + Components
// -----------------------------

#[derive(Resource)]
struct FieldGrid {
    w: i32,
    h: i32,
    aether: Vec<f32>,
    crystal: Vec<f32>,
}

impl FieldGrid {
    fn new(w: i32, h: i32) -> Self {
        let n = (w * h) as usize;
        Self {
            w,
            h,
            aether: vec![0.0; n],
            crystal: vec![0.0; n],
        }
    }

    fn idx(&self, x: i32, y: i32) -> usize {
        (y * self.w + x) as usize
    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.w && y < self.h
    }
}

#[derive(Component)]
struct Cell {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct CellMat(Handle<StandardMaterial>);

#[derive(Component)]
struct CursorViz;

#[derive(Resource, Clone, Copy)]
struct CursorCell {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tool {
    Emitter,
    Sink,
    Stabilizer,
}

#[derive(Resource)]
struct SelectedTool(Tool);

#[derive(Component, Clone, Copy)]
struct Machine {
    kind: Tool,
    strength: f32,
    radius: i32,
}

impl Plugin for FieldTestPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.03, 0.03, 0.05)))
            .insert_resource(FieldGrid::new(W, H))
            .insert_resource(CursorCell { x: W / 2, y: H / 2 })
            .insert_resource(SelectedTool(Tool::Emitter))
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    cursor_input,
                    tool_input,
                    place_machine,
                    apply_machines_to_field,
                    diffuse_and_decay_field,
                    stabilizers_make_crystal,
                    update_cell_visuals,
                    update_cursor_visual,
                )
                    .chain(),
            );
    }
}

// -----------------------------
// Setup
// -----------------------------

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera (0.17 idiomatic: spawn the component, required components are inserted automatically)
    // commands.spawn((
    //     Camera3d::default(),
    //     Transform::from_xyz(0.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    // ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 8.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera::default(),
    ));

    // Light
    commands.spawn((
        DirectionalLight {
            illuminance: 30_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.1, 0.6, 0.0)),
    ));

    // Shared cell mesh
    let cell_mesh = meshes.add(Cuboid::new(1.0, 0.2, 1.0));

    // Grid cells
    for y in 0..H {
        for x in 0..W {
            let mat = materials.add(StandardMaterial {
                base_color: Color::srgb(0.08, 0.10, 0.14),
                perceptual_roughness: 0.9,
                metallic: 0.0,
                ..default()
            });

            commands.spawn((
                Mesh3d(cell_mesh.clone()),
                MeshMaterial3d(mat.clone()),
                Transform::from_xyz(
                    (x - W / 2) as f32 * CELL_SPACING,
                    0.0,
                    (y - H / 2) as f32 * CELL_SPACING,
                ),
                Cell { x, y },
                CellMat(mat),
            ));
        }
    }

    // Cursor visualization
    let cursor_mesh = meshes.add(Cuboid::new(1.02, 0.6, 1.02));
    let cursor_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.9, 0.2),
        emissive: Color::srgb(0.3, 0.3, 0.05).into(),
        ..default()
    });

    commands.spawn((
        Mesh3d(cursor_mesh),
        MeshMaterial3d(cursor_mat),
        Transform::from_xyz(0.0, 0.55, 0.0),
        CursorViz,
    ));
}

// -----------------------------
// Input
// -----------------------------

fn cursor_input(keys: Res<ButtonInput<KeyCode>>, mut cursor: ResMut<CursorCell>) {
    let mut dx = 0;
    let mut dy = 0;

    if keys.just_pressed(KeyCode::ArrowLeft) {
        dx -= 1;
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        dx += 1;
    }
    if keys.just_pressed(KeyCode::ArrowUp) {
        dy -= 1;
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        dy += 1;
    }

    cursor.x = (cursor.x + dx).clamp(0, W - 1);
    cursor.y = (cursor.y + dy).clamp(0, H - 1);
}

fn tool_input(keys: Res<ButtonInput<KeyCode>>, mut tool: ResMut<SelectedTool>) {
    if keys.just_pressed(KeyCode::Digit1) {
        tool.0 = Tool::Emitter;
    }
    if keys.just_pressed(KeyCode::Digit2) {
        tool.0 = Tool::Sink;
    }
    if keys.just_pressed(KeyCode::Digit3) {
        tool.0 = Tool::Stabilizer;
    }
}

fn place_machine(
    keys: Res<ButtonInput<KeyCode>>,
    cursor: Res<CursorCell>,
    tool: Res<SelectedTool>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    machines: Query<&Transform, With<Machine>>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    let cursor_world = cell_world(cursor.x, cursor.y);

    // prevent stacking multiple machines on the same cell
    for t in &machines {
        if t.translation.distance(cursor_world) < 0.1 {
            return;
        }
    }

    let (color, strength, radius, height) = match tool.0 {
        Tool::Emitter => (Color::srgb(0.2, 0.9, 0.9), 8.0, 3, 1.1),
        Tool::Sink => (Color::srgb(0.95, 0.25, 0.3), -8.0, 3, 0.9),
        Tool::Stabilizer => (Color::srgb(0.75, 0.75, 1.0), 0.0, 2, 1.3),
    };

    let mesh = meshes.add(Cuboid::new(0.55, 0.8, 0.55));
    let mat = materials.add(StandardMaterial {
        base_color: color,
        emissive: (LinearRgba::from(color) * 0.25).into(),
        ..default()
    });

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(mat),
        Transform::from_translation(cursor_world + Vec3::Y * height),
        Machine {
            kind: tool.0,
            strength,
            radius,
        },
    ));
}

fn update_cursor_visual(cursor: Res<CursorCell>, mut q: Query<&mut Transform, With<CursorViz>>) {
    if !cursor.is_changed() {
        return;
    }
    let Ok(mut t) = q.single_mut() else { return };
    t.translation = cell_world(cursor.x, cursor.y) + Vec3::Y * 0.55;
}

fn cell_world(x: i32, y: i32) -> Vec3 {
    Vec3::new(
        (x - W / 2) as f32 * CELL_SPACING,
        0.0,
        (y - H / 2) as f32 * CELL_SPACING,
    )
}

// -----------------------------
// Simulation
// -----------------------------

fn apply_machines_to_field(
    time: Res<Time>,
    mut grid: ResMut<FieldGrid>,
    machines: Query<(&Transform, &Machine)>,
) {
    let dt = time.delta_secs();
    for (t, m) in &machines {
        let gx = ((t.translation.x / CELL_SPACING).round() as i32) + W / 2;
        let gy = ((t.translation.z / CELL_SPACING).round() as i32) + H / 2;

        for yy in (gy - m.radius)..=(gy + m.radius) {
            for xx in (gx - m.radius)..=(gx + m.radius) {
                if !grid.in_bounds(xx, yy) {
                    continue;
                }
                let dx = xx - gx;
                let dy = yy - gy;
                if dx * dx + dy * dy > m.radius * m.radius {
                    continue;
                }

                let idx = grid.idx(xx, yy);
                match m.kind {
                    Tool::Emitter | Tool::Sink => {
                        grid.aether[idx] =
                            (grid.aether[idx] + m.strength * dt).clamp(0.0, MAX_AETHER);
                    }
                    Tool::Stabilizer => {}
                }
            }
        }
    }
}

fn diffuse_and_decay_field(time: Res<Time>, mut grid: ResMut<FieldGrid>) {
    let dt = time.delta_secs();
    let (w, h) = (grid.w, grid.h);
    let mut next = grid.aether.clone();

    // 4-neighbor diffusion
    for y in 0..h {
        for x in 0..w {
            let i = grid.idx(x, y);
            let c = grid.aether[i];

            let mut sum = 0.0;
            let mut count = 0.0;
            for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                if grid.in_bounds(nx, ny) {
                    sum += grid.aether[grid.idx(nx, ny)];
                    count += 1.0;
                }
            }

            let neighbor_avg = if count > 0.0 { sum / count } else { c };
            let diff = (neighbor_avg - c) * DIFFUSION * dt;
            let decayed = (c + diff) * (1.0 - DECAY * dt);

            next[i] = decayed.clamp(0.0, MAX_AETHER);
        }
    }

    grid.aether = next;
}

fn stabilizers_make_crystal(
    time: Res<Time>,
    mut grid: ResMut<FieldGrid>,
    machines: Query<(&Transform, &Machine)>,
) {
    let dt = time.delta_secs();

    for (t, m) in &machines {
        if m.kind != Tool::Stabilizer {
            continue;
        }

        let gx = ((t.translation.x / CELL_SPACING).round() as i32) + W / 2;
        let gy = ((t.translation.z / CELL_SPACING).round() as i32) + H / 2;

        for yy in (gy - m.radius)..=(gy + m.radius) {
            for xx in (gx - m.radius)..=(gx + m.radius) {
                if !grid.in_bounds(xx, yy) {
                    continue;
                }
                let idx = grid.idx(xx, yy);
                let a = grid.aether[idx];

                // “sweet spot” stabilizer: converts Aether -> Crystal
                if a >= 3.0 && a <= 7.5 {
                    let convert = (1.2 * dt).min(a);
                    grid.aether[idx] -= convert;
                    grid.crystal[idx] += convert;
                }
            }
        }
    }
}

// -----------------------------
// Visualization
// -----------------------------

fn update_cell_visuals(
    grid: Res<FieldGrid>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cells: Query<(&Cell, &CellMat)>,
) {
    if !grid.is_changed() {
        return;
    }

    for (c, mat_h) in &cells {
        let i = grid.idx(c.x, c.y);
        let a = grid.aether[i];
        let cr = grid.crystal[i];

        let a_t = (a / MAX_AETHER).clamp(0.0, 1.0);
        let cr_t = (cr / 12.0).clamp(0.0, 1.0);

        let base = Vec3::new(0.07, 0.09, 0.13);
        let aether_col = Vec3::new(0.10, 0.85, 0.95) * a_t;
        let crystal_col = Vec3::new(0.85, 0.85, 1.00) * cr_t;

        let rgb = (base + aether_col + crystal_col).clamp(Vec3::ZERO, Vec3::splat(1.0));

        if let Some(mat) = materials.get_mut(&mat_h.0) {
            mat.base_color = Color::srgb(rgb.x, rgb.y, rgb.z);
            let e = aether_col * 0.25;
            mat.emissive = Color::srgb(e.x, e.y, e.z).into();
        }
    }
}
