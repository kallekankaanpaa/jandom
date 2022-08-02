use crate::fdlibm::TINY;

extern "C" {
    fn __ieee754_sqrt(x: f64) -> f64;
    fn __ieee754_new_sqrt(x: f64) -> f64;
}
#[test]
fn asdf() {
    //let x = 12.345;
    let x = f64::from_bits(0x1);
    assert_eq!(unsafe { __ieee754_new_sqrt(x) }, super::sqrt(x))
}

#[test]
fn random() {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut integers = [0u64; 128];
    rng.fill(&mut integers);
    let doubles = integers.map(|integer| f64::from_bits(integer));
    for x in doubles {
        let c = unsafe { __ieee754_sqrt(x) };
        let rust = super::sqrt(x);
        if c.is_nan() {
            assert!(rust.is_nan());
        } else {
            assert_eq!(unsafe { __ieee754_sqrt(x) }, super::sqrt(x));
        }
    }
}
#[test]
fn all_bit_patterns() {
    let start = std::time::Instant::now();
    for bits in 0_u64..0x8000000000000000_u64 {
        let x = f64::from_bits(bits);
        let c = unsafe { __ieee754_sqrt(x) };
        let rust = super::sqrt(x);
        if c.is_nan() {
            assert!(rust.is_nan());
        } else {
            assert_eq!(
                unsafe { __ieee754_sqrt(x) },
                super::sqrt(x),
                "bits = {bits:064b}"
            );
        }
    }

    let duration = start.elapsed();
    println!("Time elapsed: {duration:?}");
}

#[test]
fn new() {
    let x = 12.345;
    unsafe {
        assert_eq!(__ieee754_sqrt(x), __ieee754_new_sqrt(x));
    }
    unsafe {
        assert_eq!(__ieee754_sqrt(TINY), __ieee754_new_sqrt(TINY));
    }
}

#[test]
fn sqrt_eq_zero() {
    let x = 0_f64;
    let r_result = super::sqrt(x);
    let c_result = unsafe { __ieee754_sqrt(x) };
    assert_eq!(
        r_result,
        c_result,
        "binary repr left: {:b} right: {:b}",
        r_result.to_bits(),
        c_result.to_bits()
    );
    let x = -0_f64;
    let r_result = super::sqrt(x);
    let c_result = unsafe { __ieee754_sqrt(x) };
    assert_eq!(
        r_result,
        c_result,
        "binary repr left: {:b} right: {:b}",
        r_result.to_bits(),
        c_result.to_bits()
    );
}

#[test]
fn sqrt_eq_inf() {
    let x = f64::INFINITY;
    let r_result = super::sqrt(x);
    let c_result = unsafe { __ieee754_sqrt(x) };
    assert_eq!(
        r_result,
        c_result,
        "binary repr left: {:b} right: {:b}",
        r_result.to_bits(),
        c_result.to_bits()
    );
    let x = f64::NEG_INFINITY;
    let r_result = super::sqrt(x);
    let c_result = unsafe { __ieee754_sqrt(x) };
    assert_eq!(
        r_result.is_nan(),
        c_result.is_nan(),
        "binary repr left: {:b} right: {:b}",
        r_result.to_bits(),
        c_result.to_bits()
    );
}

#[test]
fn sqrt_eq_neg() {
    let x = -1.0;
    let r_result = super::sqrt(x);
    let c_result = unsafe { __ieee754_sqrt(x) };
    assert_eq!(
        r_result.is_nan(),
        c_result.is_nan(),
        "binary repr left: {:b} right: {:b}",
        r_result.to_bits(),
        c_result.to_bits()
    );
}
