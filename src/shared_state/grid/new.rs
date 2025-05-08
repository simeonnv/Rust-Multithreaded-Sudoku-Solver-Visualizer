use crate::libs::{
    math::generate_random_diagonal_grid::generate_random_diagonal_grid,
    sudoku::check_if_grid_is_valid::check_if_grid_is_valid,
};

use super::grid::Grid;

impl Grid {
    pub fn new(size: u8, percent_emptyness: f32) -> Grid {
        let mut buffer: Vec<Vec<u8>> = vec![vec![0; size as usize]; size as usize];

        if let Err(e) = check_if_grid_is_valid(size, percent_emptyness) {
            panic!("{}", e);
        }

        let amount_of_empty_spaces = ((size as f32).powi(2) * percent_emptyness).floor() as u32;

        generate_random_diagonal_grid(&mut buffer);

        Grid {
            raw_buffer: buffer,
            amount_of_empty_spaces,
        }
    }
}
