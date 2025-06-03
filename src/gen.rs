use crate::rng::XorShift64;

const SYMBOLS: &[u8] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes();

pub struct LolGen64([u8; 64]);

impl LolGen64 {
    pub fn new(val: &[u8]) -> Self {
        let seed = val.iter().fold(0u64, |acc, &b| {
            acc.wrapping_mul(31).wrapping_add(b as u64)
        });
        let mut result = [0; 64];
        let mut rng = XorShift64::new(seed);

        for index in 0..64 {
            result[index] = SYMBOLS[rng.next_usize(SYMBOLS.len())];
        }

        Self(result)
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap()
    }

    pub fn to_string(&self) -> String {
        String::from_utf8(self.0.to_vec()).unwrap()
    }
}
