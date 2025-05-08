use crate::{error::Error, libs::math::is_perfect_sqrt::is_perfect_square};

pub fn check_if_grid_is_valid(size: u8, percent_emptyness: f32) -> Result<(), Error> {
    if size <= 0 {
        return Err(Error::Grid("Grid size cant be negative or zero!!!"));
    }

    if is_perfect_square(size as i32) {
        return Err(Error::Grid("the size should be able to be sqrt perfectly"));
    }

    if percent_emptyness < 0.0 || percent_emptyness > 1.0 {
        return Err(Error::Grid("percent_emptyness must be between 0.0 and 1.0"));
    }

    Ok(())
}
