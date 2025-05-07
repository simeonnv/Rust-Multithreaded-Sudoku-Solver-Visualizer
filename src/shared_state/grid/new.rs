use super::grid::Grid;

impl Grid {
    pub fn new(size: u8, percent_emptyness: f32) -> Grid {
        let buffer: Vec<Vec<u8>> = vec![vec![0; size as usize]; size as usize];

        if percent_emptyness < 0.0 || percent_emptyness > 1.0 {
            panic!("percent_emptyness must be between 0.0 and 1.0");
        }

        let amount_of_empty_spaces = ((size as f32).powi(2) * percent_emptyness).floor() as u32;

        Grid {
            raw_buffer: buffer,
            amount_of_empty_spaces,
        }
    }
}
