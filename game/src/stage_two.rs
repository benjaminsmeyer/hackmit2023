use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Tile;

pub struct StageTwoPlugin;

enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Resource)]
struct StageInput(Vec<Vec2>);

const TILE_SIZE: f32 = 10.;
const PLAYER_SIZE: f32 = 3.;
const CAMERA_INTERPOLATION: f32 = 0.1;

const PLAYER_START: (f32, f32) = (-300. + TILE_SIZE, -300.);

const MAZE: &str = "##############################
.............................#
#.#####.##.#####.##.##.##.####
#.#####.##.#####.##.##.##.####
#....##.##....##.##.##.##....#
#.##.########.##############.#
#.##.########.##############.#
#.##.##...................##.#
#.##.##.##.#####.########.##.#
#.##.##.##.#####.########.##.#
#.##.##.##.##..........##.##.#
#.##.#####.##.##.########.##.#
#.##.#####.##.##.########.##.#
#.##....##.##.##.......##.##.#
#.##.##.#####.##.########.##.#
#.##.##.#####.##.########.##.#
#.##.##....##.##.......##.##.#
#.##.#####.##.#####.##.##.####
#.##.#####.##.#####.##.##.####
#.##....##.##....##.##.##....#
#.##.##.########.##.##.##.##.#
#.##.##.########.##.##.##.##.#
#.##.##.......##....##.##.##.#
#.##.##.#####.#####.########.#
#.##.##.#####.#####.########.#
#.##.##....##.##..........##.#
####.########.########.##.##.#
####.########.########.##.##.#
#.......##..........##.##.##.#
######################.#######";

impl Plugin for StageTwoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Startup, render_maze)
            .add_systems(Startup, spawn_player)
            .add_systems(Startup, create_stage_input)
            .add_systems(Update, camera_follow_player)
            .add_systems(Update, check_player_collision)
            .add_systems(Update, move_player)
            .insert_resource(ClearColor(Color::BLACK));
    }
}

fn create_stage_input(mut commands: Commands, input: Res<crate::GameInput>) {
    let inputs = input.0.split(',');

    let mut directions = Vec::new();
    for input in inputs {
        let direction = match input.to_lowercase().as_str() {
            "n" => Direction::North,
            "s" => Direction::South,
            "e" => Direction::East,
            "w" => Direction::West,
            _ => panic!("Unknown direction: {}", input),
        };

        directions.push(direction);
    }

    let mut positions = Vec::new();
    for dir in &directions {
        let previous_position = *positions
            .last()
            .unwrap_or(&Vec2::new(PLAYER_START.0, PLAYER_START.1));

        match dir {
            Direction::North => positions.push(Vec2::new(
                previous_position.x,
                previous_position.y + TILE_SIZE,
            )),
            Direction::South => positions.push(Vec2::new(
                previous_position.x,
                previous_position.y - TILE_SIZE,
            )),
            Direction::East => positions.push(Vec2::new(
                previous_position.x + TILE_SIZE,
                previous_position.y,
            )),
            Direction::West => positions.push(Vec2::new(
                previous_position.x - TILE_SIZE,
                previous_position.y,
            )),
        }
    }

    commands.insert_resource(StageInput(positions));
}

fn camera_follow_player(
    mut camera_query: Query<(&mut Transform, With<MainCamera>)>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let player_transform = player_query.single();

    let mut camera_transform = camera_query.single_mut();

    camera_transform.0.translation = camera_transform
        .0
        .translation
        .lerp(player_transform.translation, CAMERA_INTERPOLATION);
}

fn setup_camera(mut commands: Commands) {
    let projection = OrthographicProjection {
        far: 1000.,
        near: -1000.,
        scale: 0.2,
        ..Default::default()
    };
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            projection,
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings {
            intensity: 0.01,
            ..default()
        },
        MainCamera,
    ));
}

fn render_maze(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (i, line) in MAZE.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes
                            .add(shape::Quad::new(Vec2::new(TILE_SIZE, TILE_SIZE)).into())
                            .into(),
                        material: materials.add(ColorMaterial::from(Color::rgb(
                            8. + ((i as f32 / MAZE.lines().count() as f32) * 3.),
                            8. + ((j as f32 / line.chars().count() as f32) * 3.),
                            8. + ((i as f32 / MAZE.lines().count() as f32) * 3.),
                        ))),
                        transform: Transform::from_translation(Vec3::new(
                            i as f32 * TILE_SIZE - 300.,
                            j as f32 * TILE_SIZE - 300.,
                            0.,
                        )),
                        ..default()
                    },
                    Tile,
                ));
            }
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::RegularPolygon::new(PLAYER_SIZE, 6).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::rgb(7.5, 0., 7.5))),
            transform: Transform::from_translation(Vec3::new(PLAYER_START.0, PLAYER_START.1, 0.)),
            ..default()
        },
        Player,
    ));
}

fn check_player_collision(
    player_query: Query<&Transform, (With<Player>, Without<Tile>)>,
    mut tiles_query: Query<(&Transform, &mut Handle<ColorMaterial>), With<Tile>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut input: ResMut<StageInput>,
) {
    let player_transform = player_query.single();

    let collided = tiles_query.iter().any(|(tile_transform, _)| {
        player_transform
            .translation
            .distance(tile_transform.translation)
            < 0.8
    });

    if collided {
        input.0.clear();
        for (_, tile_color_mat) in tiles_query.iter_mut() {
            let color_mat = materials.get_mut(&tile_color_mat).unwrap();
            color_mat.color.set_r(40.0);
        }
    }
}

fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    mut input: ResMut<StageInput>,
) {
    let mut player_transform = player_query.single_mut();

    if let Some(position) = input.0.get(0) {
        player_transform.translation = player_transform
            .translation
            .lerp(Vec3::new(position.x, position.y, 0.0), 0.2);

        if player_transform.translation.distance(position.extend(0.0)) < 0.8 && input.0.len() >= 2 {
            input.0.remove(0);
        }
    }
}
