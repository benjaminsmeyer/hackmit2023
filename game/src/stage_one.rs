use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

const ENEMY_COUNT: usize = 10;
const ENEMY_SIZE: f32 = 40.;

#[derive(Resource)]
struct StageInput(Option<[u8; ENEMY_COUNT]>);

#[derive(Component)]
struct Enemy {
    health: u8,
}

pub struct StageOnePlugin;

impl Plugin for StageOnePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Startup, spawn_enemies)
            .add_systems(Startup, create_stage_input)
            .add_systems(Update, damage_enemies)
            .add_systems(Update, render_enemies)
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(StageInput(None));
    }
}

fn create_stage_input(input: Res<crate::GameInput>, mut stage_input: ResMut<StageInput>) {
    let inputs = input.0.split(',');
    let line_count = inputs.clone().count();
    if line_count != ENEMY_COUNT {
        panic!("Expected {} lines, got {}", ENEMY_COUNT, line_count);
    }

    let mut healths = [0; ENEMY_COUNT];
    for (i, line) in inputs.enumerate() {
        let health = line.parse::<u8>().unwrap();
        healths[i] = health;
        if health > 100 {
            panic!("Expected health to be between 0 and 100, got {}", health);
        }
    }

    stage_input.0 = Some(healths)
}

fn damage_enemies(mut enemies: Query<&mut Enemy>, game_input: Res<StageInput>) {
    if let Some(healths) = &game_input.0 {
        for (i, mut enemy) in enemies.iter_mut().enumerate() {
            enemy.health = enemy.health - (healths[i] - enemy.health) / 150;
        }
    }
}

fn render_enemies(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut enemies: Query<(&Enemy, &mut Handle<ColorMaterial>)>,
) {
    for (enemy, color_mat) in enemies.iter_mut() {
        let color_mat = materials.get_mut(&color_mat).unwrap();
        color_mat.color.set_r((100. - enemy.health as f32) / 3.);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings {
            intensity: 0.03,
            ..default()
        },
    ));
}

fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let enemies = (0..ENEMY_COUNT).map(|_| Enemy { health: 100 });

    for (i, enemy) in enemies.enumerate() {
        commands.spawn((
            enemy,
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::RegularPolygon::new(ENEMY_SIZE, 6).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::rgb(3., 12., 1.))),
                transform: Transform::from_translation(Vec3::new(
                    ENEMY_SIZE * 2. * i as f32 - (ENEMY_SIZE * ENEMY_COUNT as f32),
                    0.,
                    0.,
                )),
                ..default()
            },
        ));
    }
}
