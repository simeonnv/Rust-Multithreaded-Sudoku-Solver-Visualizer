//! Shows how to render a polygonal [`Mesh`], generated from a [`Rectangle`] primitive, in a 2D scene.

use bevy::app::*;
use bevy::{color::palettes::basic::PURPLE, prelude::*};
use bevy_tokio_tasks::tokio;
use libs::sudoku::generate_random_diagonal_grid::generate_random_diagonal_grid;
use renderer::grid::GridPlugin;

pub mod async_algorithm;
pub mod error;
pub mod libs;
pub mod renderer;
pub mod shared_state;

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
        .add_plugins(GridPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
