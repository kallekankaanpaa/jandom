use super::*;
use std::include;

const SEED: i64 = 12345;

#[test]
fn next_gaussian() {
    let test_data = include!(concat!(env!("OUT_DIR"), "/gaussians.data"));
    let mut random = Random::new(SEED);
    for integer in test_data {
        assert_eq!(random.next_gaussian(), integer);
    }
}

#[test]
fn next_i32() {
    let test_data = include!(concat!(env!("OUT_DIR"), "/integers.data"));
    let mut random = Random::new(SEED);
    for integer in test_data {
        assert_eq!(random.next_i32(), integer);
    }
}

#[test]
fn next_i32_bounded() {
    let test_data = include!(concat!(env!("OUT_DIR"), "/bounded_integers.data"));
    let mut random = Random::new(SEED);
    for (index, integer) in test_data.into_iter().enumerate() {
        assert_eq!(
            random.next_i32_bounded((SEED as i32) + (index as i32)),
            integer
        );
    }
}

#[test]
fn next_i64() {
    let test_data = include!(concat!(env!("OUT_DIR"), "/longs.data"));
    let mut random = Random::new(SEED);
    for integer in test_data {
        assert_eq!(random.next_i64(), integer);
    }
}

#[test]
fn next_f32() {
    let test_data = include!(concat!(env!("OUT_DIR"), "/floats.data"));
    let mut random = Random::new(SEED);
    for integer in test_data {
        assert_eq!(random.next_f32(), integer);
    }
}

#[test]
fn next_f64() {
    let test_data = include!(concat!(env!("OUT_DIR"), "/doubles.data"));
    let mut random = Random::new(SEED);
    for integer in test_data {
        assert_eq!(random.next_f64(), integer);
    }
}

#[test]
fn next_bool() {
    let test_data = include!(concat!(env!("OUT_DIR"), "/booleans.data"));
    let mut random = Random::new(SEED);
    for integer in test_data {
        assert_eq!(random.next_bool(), integer);
    }
}

#[test]
fn next_bytes() {
    let test_data = include!(concat!(env!("OUT_DIR"), "/bytes.data"));
    let mut random = Random::new(SEED);
    let mut bytes = [0_i8; 100];
    random.next_bytes(&mut bytes);
    for i in 0..10 {
        assert_eq!(test_data[i], bytes[i]);
    }
}
