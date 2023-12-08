use rand::{thread_rng, Rng};

pub fn vecrng_make(len: u32, range: u32) -> Vec<u32> {
    let mut rng = thread_rng();
    let rng_vec: Vec<u32> = (0..len).map(|_| rng.gen_range(0..range)).collect();
    return rng_vec;
}
