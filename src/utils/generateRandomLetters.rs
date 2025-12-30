use rand::{thread_rng, Rng};

pub fn generate_random_letters(l: usize) -> String {
    let mut rng = thread_rng();
    (0..l)
        .map(|_| {
            let b: u8 = rng.r#gen(); // Genera un byte aleatorio
            let val = b % 52;
            let char_code = if val < 26 { val + 65 } else { val + 71 };
            char_code as char
        })
        .collect()
}
