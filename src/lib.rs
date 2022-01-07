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
        self.next(1) != 0
    }
    pub fn next_f32(&mut self) -> f32 {
        self.next(24) as f32 / ((1 << 24) as f32)
    }
    pub fn next_f64(&mut self) -> f64 {
        ((((self.next(26) as i64) << 27) + (self.next(27) as i64)) as f64) / ((1_i64 << 53) as f64)
    }

    pub fn next_bytes(&mut self, bytes: &mut [i8]) {
        let max = bytes.len() & !0x3;

        for i in (0..max).step_by(4) {
            let random = self.next(32);
            bytes[i] = random as i8;
            bytes[i + 1] = (random >> 8) as i8;
            bytes[i + 2] = (random >> 16) as i8;
            bytes[i + 3] = (random >> 24) as i8;
        }
        if max < bytes.len() {
            let mut random = self.next(32);
            for j in max..bytes.len() {
                bytes[j] = random as i8;
                random = random >> 8;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::include;

    const SEED: i64 = 12345;

    #[test]
    fn next_i32() {
        let test_data = if cfg!(target_os = "windows") {
            include!("..\\generated\\integers.data")
        } else {
            include!("../generated/integers.data")
        };
        let mut random = Random::new(SEED);
        for integer in test_data {
            assert_eq!(random.next_i32(), integer);
        }
    }

    #[test]
    fn next_i64() {
        let test_data = if cfg!(target_os = "windows") {
            include!("..\\generated\\longs.data")
        } else {
            include!("../generated/longs.data")
        };
        let mut random = Random::new(SEED);
        for integer in test_data {
            assert_eq!(random.next_i64(), integer);
        }
    }

    #[test]
    fn next_f32() {
        let test_data = if cfg!(target_os = "windows") {
            include!("..\\generated\\floats.data")
        } else {
            include!("../generated/floats.data")
        };
        let mut random = Random::new(SEED);
        for integer in test_data {
            assert_eq!(random.next_f32(), integer);
        }
    }

    #[test]
    fn next_f64() {
        let test_data = if cfg!(target_os = "windows") {
            include!("..\\generated\\doubles.data")
        } else {
            include!("../generated/doubles.data")
        };
        let mut random = Random::new(SEED);
        for integer in test_data {
            assert_eq!(random.next_f64(), integer);
        }
    }

    #[test]
    fn next_bool() {
        let test_data = if cfg!(target_os = "windows") {
            include!("..\\generated\\booleans.data")
        } else {
            include!("../generated/booleans.data")
        };
        let mut random = Random::new(SEED);
        for integer in test_data {
            assert_eq!(random.next_bool(), integer);
        }
    }

    #[test]
    fn next_bytes() {
        let test_data = if cfg!(target_os = "windows") {
            include!("..\\generated\\bytes.data")
        } else {
            include!("../generated/bytes.data")
        };
        let mut random = Random::new(SEED);
        let mut bytes = [0_i8; 10];
        random.next_bytes(&mut bytes);
        for i in 0..10 {
            assert_eq!(test_data[i], bytes[i]);
        }
    }
}
