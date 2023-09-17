use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

const CAMERA_INTERPOLATION: f32 = 0.1;

#[derive(Component)]
struct MainCamera;

const EXPECTED_RESULT: usize = 10;

pub struct StageZeroPlugin;

#[derive(Component)]
struct Ball;

impl Plugin for StageZeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Startup, render)
            .add_systems(Update, verdict)
            .add_systems(Update, move_camera)
            .insert_resource(ClearColor(Color::BLACK));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings {
            intensity: 0.03,
            ..default()
        },
    ));
}

fn move_camera(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    input: Res<crate::GameInput>,
) {
    let value = input.0.parse::<usize>().unwrap();

    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation = camera_transform.translation.lerp(
        Vec3::new(value as f32 * 30.0, 0.0, 0.0),
        CAMERA_INTERPOLATION,
    );
}

fn render(
    mut commands: Commands,
    input: Res<crate::GameInput>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let value = input.0.parse::<usize>().unwrap();

    for i in 0..value {
        commands.spawn((
            Ball,
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
                material: materials.add(ColorMaterial::from(Color::rgb(5.0, 5.0, 0.0))),
                transform: Transform::from_translation(Vec3::new(i as f32 * 10.0 * 3.0, 0.0, 0.0)),
                ..default()
            },
        ));
    }
}

fn verdict(
    mut ball_query: Query<&mut Handle<ColorMaterial>, With<Ball>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    input: Res<crate::GameInput>,
) {
    let value = input.0.parse::<usize>().unwrap();

    for color_mat in ball_query.iter_mut() {
        let color_mat = materials.get_mut(&color_mat).unwrap();
        if value == EXPECTED_RESULT {
            color_mat
                .color
                .set_g(color_mat.color.g() + (10.0 - color_mat.color.g()) * 0.01);
        } else {
            color_mat
                .color
                .set_r(color_mat.color.r() + (10.0 - color_mat.color.r()) * 0.01);
        }
    }
}
