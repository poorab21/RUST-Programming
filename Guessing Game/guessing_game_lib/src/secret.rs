use rand::Rng;

pub fn generate_num_to_guess() -> u32 {
    let mut rng = rand::thread_rng();

    let number = rng.gen_range(1..=100);
    number as u32
}