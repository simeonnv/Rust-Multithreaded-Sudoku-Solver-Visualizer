use rand::{rng, seq::SliceRandom};

pub fn generate_random_diagonal_grid(raw_buffer: &mut Vec<Vec<u8>>) {
    let mut rng = rng();

    let size = raw_buffer.len();
    let chunk_size = (size as f64).sqrt() as u8;

    for block in 0..chunk_size {
        let mut rand_nums: Vec<u8> = (1..(chunk_size * chunk_size + 1)).collect();
        rand_nums.shuffle(&mut rng);

        let start = block as usize * chunk_size as usize;
        let mut rand_nums_pt = 0;

        dbg!(&rand_nums);

        for i in 0..chunk_size {
            for j in 0..chunk_size {
                raw_buffer[start + i as usize][start + j as usize] = rand_nums[rand_nums_pt];
                rand_nums_pt += 1;
            }
        }
    }

    dbg!(&raw_buffer);
}
