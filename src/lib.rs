const MULTIPLIER: i64 = 0x5deece66d;
const INCREMENT: i64 = 0xb;
const MASK: i64 = (1 << 48) - 1;

pub struct Random {
    state: i64,
}

impl Random {
    pub fn new(seed: i64) -> Self {
        Self {
            state: Self::initalize_state(seed),
        }
    }

    fn initalize_state(seed: i64) -> i64 {
        seed ^ MULTIPLIER & MASK
    }

    fn next(&mut self, bits: u8) -> i32 {
        self.state = self.state.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT) & MASK;
        (self.state >> (48 - bits)) as i32
    }

    pub fn next_i32(&mut self) -> i32 {
        self.next(32)
    }

    pub fn next_i64(&mut self) -> i64 {
        ((self.next(32) as i64) << 32) + (self.next(32) as i64)
    }

    pub fn next_bool(&mut self) -> bool {
        unimplemented!()
    }
    pub fn next_f32(&mut self) -> f32 {
        unimplemented!()
    }
    pub fn next_f64(&mut self) -> f64 {
        unimplemented!()
    }
    pub fn next_bytes(&mut self, bytes: &mut [i8]) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::include;

    const SEED: i64 = 12345;

    #[test]
    fn next_i32() {
        let test_data = include!("..\\generated\\integers.data");
        let mut random = Random::new(SEED);
        for integer in test_data {
            assert_eq!(random.next_i32(), integer);
        }
    }

    #[test]
    fn next_i64() {
        let test_data = include!("..\\generated\\longs.data");
        let mut random = Random::new(SEED);
        for integer in test_data {
            assert_eq!(random.next_i64(), integer);
        }
    }

    #[test]
    fn next_f32() {
        let test_data = include!("..\\generated\\floats.data");
        let mut random = Random::new(SEED);
        for integer in test_data {
            assert_eq!(random.next_f32(), integer);
        }
    }

    #[test]
    fn next_f64() {
        let test_data = include!("..\\generated\\doubles.data");
        let mut random = Random::new(SEED);
        for integer in test_data {
            assert_eq!(random.next_f64(), integer);
        }
    }

    #[test]
    fn next_bool() {
        let test_data = include!("..\\generated\\booleans.data");
        let mut random = Random::new(SEED);
        for integer in test_data {
            assert_eq!(random.next_bool(), integer);
        }
    }

    #[test]
    fn next_bytes() {
        let test_data = include!("..\\generated\\bytes.data");
        let mut random = Random::new(SEED);
        let mut bytes = [0_i8; 10];
        random.next_bytes(&mut bytes);
        for i in 0..10 {
            assert_eq!(test_data[i], bytes[i]);
        }
    }
}
