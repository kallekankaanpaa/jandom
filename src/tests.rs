use super::*;
use std::include;

const SEED: i64 = 12345;

#[test]
fn next_gaussian() {
    let test_data = if cfg!(target_os = "windows") {
        include!("..\\generated\\gaussians.data")
    } else {
        include!("../generated/gaussians.data")
    };
    let mut random = Random::new(SEED);
    for integer in test_data {
        assert_eq!(random.next_gaussian(), integer);
    }
}

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
fn next_i32_bounded() {
    let test_data = if cfg!(target_os = "windows") {
        include!("..\\generated\\bounded_integers.data")
    } else {
        include!("../generated/bounded_integers.data")
    };
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
    let mut bytes = [0_i8; 100];
    random.next_bytes(&mut bytes);
    for i in 0..10 {
        assert_eq!(test_data[i], bytes[i]);
    }
}
