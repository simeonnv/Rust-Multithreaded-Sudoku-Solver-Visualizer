use lazy_static::lazy_static;

pub struct Grid {
    pub raw_buffer: Vec<Vec<u8>>,
    pub amount_of_empty_spaces: u32,
}

lazy_static! {
    pub static ref GRID: Grid = Grid::new(32, 0.5);
}
