//! Shows how to render a polygonal [`Mesh`], generated from a [`Rectangle`] primitive, in a 2D scene.

use bevy::app::*;
use bevy::{color::palettes::basic::PURPLE, prelude::*};
use bevy_tokio_tasks::tokio;
use libs::math::generate_random_diagonal_grid::generate_random_diagonal_grid;

pub mod async_algorithm;
pub mod error;
pub mod libs;
pub mod renderer;
pub mod shared_state;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Running,
    Paused,
}

fn main() {
    let mut buffer: Vec<Vec<u8>> = vec![vec![0; 16 as usize]; 16 as usize];
    generate_random_diagonal_grid(&mut buffer);

    App::new()
        .add_plugins(bevy_tokio_tasks::TokioTasksPlugin {
            make_runtime: Box::new(|| {
                let mut runtime = tokio::runtime::Builder::new_multi_thread();
                runtime.enable_all();
                runtime.build().unwrap()
            }),
            ..bevy_tokio_tasks::TokioTasksPlugin::default()
        })
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::default())),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        Transform::default().with_scale(Vec3::splat(128.)),
    ));
}
