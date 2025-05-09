mod component;
mod systems;

use bevy::{
    color::palettes::{css::PURPLE, tailwind::PURPLE_700},
    prelude::*,
    window::PrimaryWindow,
};

use crate::shared_state::grid::grid::GRID;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let grid = &GRID;
    let size = grid.raw_buffer.len();
    let cell_size = 500.0;

    let window = window_query.single().unwrap();

    for y in 0..size {
        for x in 0..size {
            println!("DEBUG: CREATING CELL");
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(Color::from(PURPLE))),
                Transform::from_xyz(x as f32, y as f32, 0.0),
                component::Grid {},
            ));
        }
    }
}
