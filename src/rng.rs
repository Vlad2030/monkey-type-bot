pub struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    pub fn new(seed: u64) -> Self {
        Self {
            state: if seed != 0 {
                seed
            } else {
                0xdeadbeef
            },
        }
    }

    pub fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    pub fn next_usize(
        &mut self,
        max: usize,
    ) -> usize {
        (self.next() as usize) % max
    }
}
