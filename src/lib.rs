const MULTIPLIER: i64 = 0x5deece66d;
const INCREMENT: i64 = 0xb;
const MASK: i64 = (1 << 48) - 1;

pub struct Random {
    state: i64,
}

impl Random {
    fn new(seed: i32) -> Self {
        Self { state: seed as i64 }
    }

    fn next_i32(&self) -> i32 {
        unimplemented!()
    }
    fn next_i64(&self) -> i64 {
        unimplemented!()
    }
    fn next_bool(&self) -> bool {
        unimplemented!()
    }
    fn next_f32(&self) -> f32 {
        unimplemented!()
    }
    fn next_f64(&self) -> f64 {
        unimplemented!()
    }
    fn next_bytes(&self, bytes: &mut [i8]) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::include;

    const SEED: i32 = 12345;

    #[test]
    fn next_i32() {
        let test_data = include!("..\\generated\\integers.data");
        let random = Random::new(SEED);
        for integer in test_data {
            assert_eq!(random.next_i32(), integer);
        }
    }
    fn next_i64() {
        let test_data = include!("..\\generated\\longs.data");
        let random = Random::new(SEED);
        for integer in test_data {
            assert_eq!(random.next_i64(), integer);
        }
    }
    fn next_f32() {
        let test_data = include!("..\\generated\\floats.data");
        let random = Random::new(SEED);
        for integer in test_data {
            assert_eq!(random.next_f32(), integer);
        }
    }
    fn next_f64() {
        let test_data = include!("..\\generated\\doubles.data");
        let random = Random::new(SEED);
        for integer in test_data {
            assert_eq!(random.next_f64(), integer);
        }
    }
    fn next_bool() {
        let test_data = include!("..\\generated\\booleans.data");
        let random = Random::new(SEED);
        for integer in test_data {
            assert_eq!(random.next_bool(), integer);
        }
    }
    fn next_bytes() {
        let test_data = include!("..\\generated\\bytes.data");
        let random = Random::new(SEED);
        let mut bytes = [0_i8; 10];
        random.next_bytes(&mut bytes);
        for i in 0..10 {
            assert_eq!(test_data[i], bytes[i]);
        }
    }
}
